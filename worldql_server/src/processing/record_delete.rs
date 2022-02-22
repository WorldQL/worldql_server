use color_eyre::Result;
use uuid::Uuid;
use worldql_messages::client_bound::{ClientMessageReply, RecordDeleteReply, Status};
use worldql_messages::server_bound::RecordDeleteRequest;

use crate::database::DatabaseClient;
use crate::trace_packet;
use crate::transport::ThreadPeerMap;

pub(super) async fn handle_record_delete(
    sender: Uuid,
    peer_map: &mut ThreadPeerMap,
    request: RecordDeleteRequest,
    db: &DatabaseClient,
) -> Result<()> {
    trace_packet!("{:?}", &request);

    let status = match db.delete_records(request.records).await {
        Err(error) => error.into(),

        Ok(affected) => {
            let reply = RecordDeleteReply::new(affected);
            Status::Ok(reply)
        }
    };

    let mut map = peer_map.write().await;
    if let Some(peer) = map.get_mut(&sender) {
        let reply: ClientMessageReply = status.into();

        // TODO: Handle errors
        let _ = peer.send_message(&reply.into()).await;
    }

    Ok(())
}
