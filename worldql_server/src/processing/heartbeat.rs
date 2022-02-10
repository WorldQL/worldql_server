use color_eyre::Result;
use uuid::Uuid;
use worldql_messages::server_bound::HeartbeatRequest;

use crate::trace_packet;
use crate::transport::ThreadPeerMap;

pub(super) async fn handle_heartbeat(
    peer: Uuid,
    request: &HeartbeatRequest,
    peer_map: &ThreadPeerMap,
) -> Result<()> {
    trace_packet!("{:?}", &request);

    todo!()
}
