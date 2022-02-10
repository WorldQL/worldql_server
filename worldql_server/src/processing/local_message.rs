use color_eyre::Result;
use uuid::Uuid;
use worldql_messages::server_bound::LocalMessageRequest;
use worldql_subscriptions::SubscriptionManager;

use crate::trace_packet;
use crate::transport::ThreadPeerMap;

pub(super) async fn handle_local_message(
    sender: Uuid,
    peer_map: &mut ThreadPeerMap,
    request: LocalMessageRequest,
    manager: &mut SubscriptionManager,
) -> Result<()> {
    trace_packet!("{:?}", &request);

    todo!()
}
