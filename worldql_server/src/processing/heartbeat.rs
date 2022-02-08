use color_eyre::Result;
use worldql_messages::server_bound::HeartbeatRequest;

use crate::transport::ThreadPeerMap;

pub(super) async fn handle_heartbeat(
    request: &HeartbeatRequest,
    peer_map: &ThreadPeerMap,
) -> Result<()> {
    todo!()
}
