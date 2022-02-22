use color_eyre::Result;
use tracing::debug;
use uuid::Uuid;
use worldql_messages::client_bound::{ClientMessageReply, RecordDeleteReply, Status};
use worldql_messages::server_bound::RecordDeleteRequest;

use crate::database::DatabaseClient;
use crate::errors::err_invalid_world_name;
use crate::trace_packet;
use crate::transport::ThreadPeerMap;
use crate::utils::sanitize_world_name;

pub(super) async fn handle_record_delete(
    sender: Uuid,
    peer_map: &mut ThreadPeerMap,
    request: RecordDeleteRequest,
    db: &DatabaseClient,
) -> Result<()> {
    trace_packet!("{:?}", &request);

    let reply = process_message(sender, request, db).await;
    let reply = ClientMessageReply::RecordDelete(reply);

    let mut map = peer_map.write().await;
    if let Some(peer) = map.get_mut(&sender) {
        // TODO: Handle errors
        let _ = peer.send_message(&reply.into()).await;
    }

    Ok(())
}

async fn process_message(
    sender: Uuid,
    request: RecordDeleteRequest,
    db: &DatabaseClient,
) -> Status<RecordDeleteReply> {
    let sanitize_error = request.records.iter().find_map(|record| {
        sanitize_world_name(&record.world_name).map(|error| (error, &record.world_name))
    });

    if let Some((error, world_name)) = sanitize_error {
        debug!(
            "peer {} sent invalid world name: {} ({})",
            &sender, world_name, error
        );

        let message = format!("invalid world name: {}", error);
        let error = err_invalid_world_name(message);

        return error.into();
    }

    match db.delete_records(request.records).await {
        Err(error) => error.into(),

        Ok(affected) => {
            let reply = RecordDeleteReply::new(affected);
            Status::Ok(reply)
        }
    }
}
