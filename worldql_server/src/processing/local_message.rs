use color_eyre::Result;
use uuid::Uuid;
use worldql_messages::server_bound::LocalMessageRequest;
use worldql_subscriptions::SubscriptionManager;

use crate::{trace_packet, transport::ThreadPeerMap};

pub(super) async fn handle_local_message(
    peer: Uuid,
    peer_map: &mut ThreadPeerMap,
    request: LocalMessageRequest,
    manager: &mut SubscriptionManager,
) -> Result<()> {
    trace_packet!("{:?}", &request);

    todo!()
}
