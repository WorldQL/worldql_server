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
    db: &mut DatabaseClient,
) -> Result<()> {
    trace_packet!("{:?}", &request);

    let world_name = request.world_name;
    let reply: ClientMessageReply = match request.position {
        None => {
            let status = match db.clear_records_in_world(&world_name).await {
                Err(error) => error.into(),

                Ok(affected) => {
                    let reply = RecordClearReply::new(affected);
                    Status::Ok(reply)
                }
            };

            status.into()
        }

        Some(position) => {
            let status = match db.clear_records_in_area(&world_name, position).await {
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
