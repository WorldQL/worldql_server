use color_eyre::Result;
use tracing::warn;

use crate::structures::Message;
use crate::utils::GLOBAL_WORLD;
use crate::{trace_packet, DatabaseClient, ThreadPeerMap};

pub(super) async fn handle_record_create(
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
    for record in &message.records {
        let result = database_client.insert_record(record).await;

        if let Err(error) = result {
            warn!("peer {} record create error: {}", uuid, error);
        }
    }

    Ok(())
}
