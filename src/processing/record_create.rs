use crate::{DatabaseClient, ThreadPeerMap, trace_packet};
use color_eyre::Result;
use crate::structures::Message;
use crate::utils::GLOBAL_WORLD;


pub(super) async fn handle_record_create(
    message: Message,
    peer_map: &ThreadPeerMap,
    database_client: &mut DatabaseClient,
) -> Result<()> {
    trace_packet!("{}", &message);

    // Ignore global world
    if message.world_name == GLOBAL_WORLD {
        return Ok(());
    }

    let records = message.records;
    for record in records {
        database_client.insert_record(record).await?;
    }

    Ok(())
}
