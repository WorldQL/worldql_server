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
    db: &DatabaseClient,
) -> Result<()> {
    trace_packet!("{:?}", &request);

    let reply = process_message(sender, request, db).await;
    let reply = ClientMessageReply::RecordSet(reply);

    let mut map = peer_map.write().await;
    if let Some(peer) = map.get_mut(&sender) {
        // TODO: Handle errors
        let _ = peer.send_message(&reply.into()).await;
    }

    Ok(())
}

async fn process_message(
    sender: Uuid,
    request: RecordSetRequest,
    db: &DatabaseClient,
) -> Status<RecordSetReply> {
    match db.set_records(request.records).await {
        Err(error) => error.into(),

        Ok((created, updated)) => {
            let reply = RecordSetReply::new(created, updated);
            Status::Ok(reply)
        }
    }
}
