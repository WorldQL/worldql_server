use color_eyre::Result;
use tracing::warn;

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
                .get_records_in_region(&message.world_name, position)
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

            let reply = Message {
                instruction: Instruction::RecordReply,
                world_name: message.world_name,
                records,
                ..Default::default()
            };

            let mut map = peer_map.write().await;
            let peer = map.get_mut(&uuid);
            if peer.is_none() {
                warn!("Missing peer {} for GlobalMessage send!", &uuid);
                return Ok(());
            }

            let peer = peer.unwrap();
            let _ = peer.send(reply).await;
        }

        // Handle messages without position
        // TODO
        None => todo!(),
    }

    Ok(())
}
