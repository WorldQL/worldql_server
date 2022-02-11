use color_eyre::Result;
use uuid::Uuid;
use worldql_messages::client_bound::HeartbeatReply;
use worldql_messages::server_bound::HeartbeatRequest;

use crate::trace_packet;
use crate::transport::ThreadPeerMap;

pub(super) async fn handle_heartbeat(
    sender: Uuid,
    request: HeartbeatRequest,
    peer_map: &ThreadPeerMap,
) -> Result<()> {
    trace_packet!("{:?}", &request);

    let mut map = peer_map.write().await;
    if let Some(peer) = map.get_mut(&sender) {
        peer.update_heartbeat();

        let reply: HeartbeatReply = request.into();
        // TODO: Handle errors
        let _ = peer.send_message(&reply.into()).await;
    }

    Ok(())
}
