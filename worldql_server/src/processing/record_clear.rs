use color_eyre::Result;
use tracing::debug;
use uuid::Uuid;
use worldql_messages::client_bound::{ClientMessageReply, RecordClearReply, Status};
use worldql_messages::server_bound::RecordClearRequest;

use crate::database::DatabaseClient;
use crate::errors::err_invalid_world_name;
use crate::trace_packet;
use crate::transport::ThreadPeerMap;
use crate::utils::sanitize_world_name;

pub(super) async fn handle_record_clear(
    sender: Uuid,
    peer_map: &mut ThreadPeerMap,
    request: RecordClearRequest,
    db: &DatabaseClient,
) -> Result<()> {
    trace_packet!("{:?}", &request);

    let reply = process_message(sender, request, db).await;
    let reply = ClientMessageReply::RecordClear(reply);

    let mut map = peer_map.write().await;
    if let Some(peer) = map.get_mut(&sender) {
        // TODO: Handle errors
        let _ = peer.send_message(&reply.into()).await;
    }

    Ok(())
}

async fn process_message(
    sender: Uuid,
    request: RecordClearRequest,
    db: &DatabaseClient,
) -> Status<RecordClearReply> {
    match request {
        RecordClearRequest::ByWorld { world_name } => {
            if let Some(error) = sanitize_world_name(&world_name) {
                debug!(
                    "peer {} sent invalid world name: {} ({})",
                    &sender, &world_name, error
                );

                let message = format!("invalid world name: {}", error);
                let error = err_invalid_world_name(message);

                return error.into();
            }

            match db.clear_records_in_world(&world_name).await {
                Err(error) => error.into(),

                Ok(affected) => {
                    let reply = RecordClearReply::new(affected);
                    Status::Ok(reply)
                }
            }
        }

        RecordClearRequest::ByArea {
            world_name,
            pos_1,
            pos_2,
        } => {
            if let Some(error) = sanitize_world_name(&world_name) {
                debug!(
                    "peer {} sent invalid world name: {} ({})",
                    &sender, &world_name, error
                );

                let message = format!("invalid world name: {}", error);
                let error = err_invalid_world_name(message);

                return error.into();
            }

            match db.clear_records_in_area(&world_name, pos_1, pos_2).await {
                Err(error) => error.into(),

                Ok(affected) => {
                    let reply = RecordClearReply::new(affected);
                    Status::Ok(reply)
                }
            }
        }
    }
}
