use color_eyre::Result;
use uuid::Uuid;
use worldql_messages::client_bound::{ClientMessageReply, RecordClearReply, Status};
use worldql_messages::server_bound::RecordClearRequest;

use crate::database::DatabaseClient;
use crate::trace_packet;
use crate::transport::ThreadPeerMap;

pub(super) async fn handle_record_clear(
    sender: Uuid,
    peer_map: &mut ThreadPeerMap,
    request: RecordClearRequest,
    db: &DatabaseClient,
) -> Result<()> {
    trace_packet!("{:?}", &request);

    let reply: ClientMessageReply = match request {
        RecordClearRequest::ByWorld { world_name } => {
            let status = match db.clear_records_in_world(&world_name).await {
                Err(error) => error.into(),

                Ok(affected) => {
                    let reply = RecordClearReply::new(affected);
                    Status::Ok(reply)
                }
            };

            status.into()
        }

        RecordClearRequest::ByArea {
            world_name,
            pos_1,
            pos_2,
        } => {
            let status = match db.clear_records_in_area(&world_name, pos_1, pos_2).await {
                Err(error) => error.into(),

                Ok(affected) => {
                    let reply = RecordClearReply::new(affected);
                    Status::Ok(reply)
                }
            };

            status.into()
        }
    };

    let mut map = peer_map.write().await;
    if let Some(peer) = map.get_mut(&sender) {
        // TODO: Handle errors
        let _ = peer.send_message(&reply.into()).await;
    }

    Ok(())
}
