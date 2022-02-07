use color_eyre::Result;
use tracing::warn;

use crate::structures::Message;
use crate::utils::GLOBAL_WORLD;
use crate::{trace_packet, DatabaseClient, ThreadPeerMap};

pub(super) async fn handle_record_delete(
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
    let errors = database_client.delete_records(message.records).await;
    for error in errors {
        warn!("peer {} record remove error: {}", uuid, error);
    }

    Ok(())
}
