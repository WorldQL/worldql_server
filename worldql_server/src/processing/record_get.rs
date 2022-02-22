use color_eyre::Result;
use tracing::debug;
use uuid::Uuid;
use worldql_messages::client_bound::{ClientMessageReply, RecordGetReply, Status};
use worldql_messages::server_bound::RecordGetRequest;

use crate::database::DatabaseClient;
use crate::errors::err_invalid_world_name;
use crate::trace_packet;
use crate::transport::ThreadPeerMap;
use crate::utils::sanitize_world_name;

pub(super) async fn handle_record_get(
    sender: Uuid,
    peer_map: &mut ThreadPeerMap,
    request: RecordGetRequest,
    db: &DatabaseClient,
) -> Result<()> {
    trace_packet!("{:?}", &request);

    let reply = process_message(sender, request, db).await;
    let reply = ClientMessageReply::RecordGet(reply);

    let mut map = peer_map.write().await;
    if let Some(peer) = map.get_mut(&sender) {
        // TODO: Handle errors
        let _ = peer.send_message(&reply.into()).await;
    }

    Ok(())
}

async fn process_message(sender: Uuid, request: RecordGetRequest, db: &DatabaseClient) -> Status<RecordGetReply> {
    match request {
        RecordGetRequest::ByArea {
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

            match db.get_records_by_area(&world_name, pos_1, pos_2).await {
                Err(error) => error.into(),

                Ok(records) => {
                    let reply = RecordGetReply::new(records);
                    Status::Ok(reply)
                }
            }
        }

        RecordGetRequest::ByUuid { records } => {
            match db.get_records_by_id(records).await {
                Err(error) => error.into(),

                Ok(records) => {
                    let reply = RecordGetReply::new(records);
                    Status::Ok(reply)
                }
            }
        }
    }
}
