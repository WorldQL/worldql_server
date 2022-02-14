use color_eyre::Result;
use uuid::Uuid;
use worldql_messages::client_bound::{ClientMessageReply, RecordGetReply, Status};
use worldql_messages::server_bound::RecordGetRequest;

use crate::database::DatabaseClient;
use crate::trace_packet;
use crate::transport::ThreadPeerMap;

pub(super) async fn handle_record_get(
    sender: Uuid,
    peer_map: &mut ThreadPeerMap,
    request: RecordGetRequest,
    db: &mut DatabaseClient,
) -> Result<()> {
    trace_packet!("{:?}", &request);

    let reply: ClientMessageReply = match request {
        RecordGetRequest::Area {
            world_name,
            position,
        } => {
            let status = match db.get_records_by_area(&world_name, position).await {
                Err(error) => error.into(),

                Ok(records) => {
                    let reply = RecordGetReply::new(records);
                    Status::Ok(reply)
                }
            };

            status.into()
        }

        RecordGetRequest::Uuid { records } => {
            let status = match db.get_records_by_id(records).await {
                Err(error) => error.into(),

                Ok(records) => {
                    let reply = RecordGetReply::new(records);
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
