use color_eyre::Result;
use uuid::Uuid;
use worldql_messages::client_bound::{ClientMessageReply, RecordSetReply, Status};
use worldql_messages::server_bound::RecordSetRequest;

use crate::database::DatabaseClient;
use crate::trace_packet;
use crate::transport::ThreadPeerMap;

pub(super) async fn handle_record_set(
    sender: Uuid,
    peer_map: &mut ThreadPeerMap,
    request: RecordSetRequest,
    db: &mut DatabaseClient,
) -> Result<()> {
    trace_packet!("{:?}", &request);

    let status = match db.set_records(request.records).await {
        Err(error) => error.into(),

        Ok((created, updated)) => {
            let reply = RecordSetReply::new(created, updated);
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
