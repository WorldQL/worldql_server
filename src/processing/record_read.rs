use std::collections::HashMap;

use color_eyre::Result;
use tracing::warn;

use crate::database::DedupeData;
use crate::structures::{Instruction, Message};
use crate::utils::GLOBAL_WORLD;
use crate::{trace_packet, DatabaseClient, ThreadPeerMap};

pub(super) async fn handle_record_read(
    message: Message,
    database_client: &mut DatabaseClient,
    peer_map: &ThreadPeerMap,
) -> Result<()> {
    trace_packet!("{}", &message);

    // Ignore global world
    if message.world_name == GLOBAL_WORLD {
        return Ok(());
    }

    let uuid = message.sender_uuid;
    match message.position {
        // Handle messages with position
        Some(position) => {
            let result = database_client
                .get_records_in_region(&message.world_name, position, None)
                .await;

            let records = match result {
                Ok(records) => records,
                Err(error) => {
                    warn!("error getting records for {}: {}", uuid, error);
                    return Ok(());
                }
            };

            // Early return to avoid locking the peer map
            if records.is_empty() {
                return Ok(());
            }

            // Deduplicate records
            let deduped = {
                let mut map = HashMap::new();
                for (ts, record) in records {
                    match map.get(&record.uuid) {
                        // Not seen before, insert
                        None => {
                            map.insert(record.uuid, (ts, record));
                        }

                        // Seen before, check timestamp
                        Some((existing_ts, _)) => {
                            // Only insert if timestamp is later
                            if &ts >= existing_ts {
                                map.insert(record.uuid, (ts, record));
                            }
                        }
                    }
                }

                map.into_values().collect::<Vec<_>>()
            };

            // Extract dedupe command information from list
            let dedupe_ops = deduped
                .iter()
                .map(|(ts, record)| {
                    let data: DedupeData = (
                        record.uuid,
                        *ts,
                        record.world_name.clone(),
                        // TODO: Handle records without position
                        record.position.unwrap(),
                    );

                    data
                })
                .collect::<Vec<DedupeData>>();

            // Extract only records from deduplicated list
            let records = deduped
                .into_iter()
                .map(|(_, record)| record)
                .collect::<Vec<_>>();

            let reply = Message {
                instruction: Instruction::RecordReply,
                world_name: message.world_name,
                records,
                ..Default::default()
            };

            // Lock peer map for only this section
            {
                let mut map = peer_map.write().await;
                let peer = map.get_mut(&uuid);
                if peer.is_none() {
                    warn!("Missing peer {} for GlobalMessage send!", &uuid);
                    return Ok(());
                }

                let peer = peer.unwrap();
                let _ = peer.send(reply).await;
            }

            // Deduplicate records in background
            let result = database_client.dedupe_records(dedupe_ops).await;
            if let Err(error) = result {
                warn!("error deduping records for {}: {}", uuid, error);
                return Ok(());
            }
        }

        // Handle messages without position
        // TODO
        None => todo!(),
    }

    Ok(())
}
