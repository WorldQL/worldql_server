use color_eyre::Result;
use tracing::warn;
use uuid::Uuid;
use worldql_messages::client_bound::{ClientMessageReply, Status, WorldUnsubscribeReply};
use worldql_messages::server_bound::WorldUnsubscribeRequest;
use worldql_subscriptions::SubscriptionManager;

use crate::errors::{err_invalid_world_name, ERR_WORLD_UNSUB_GLOBAL_WORLD};
use crate::trace_packet;
use crate::transport::ThreadPeerMap;
use crate::utils::{sanitize_world_name, GLOBAL_WORLD};

pub(super) async fn handle_world_unsubscribe(
    peer: Uuid,
    peer_map: &mut ThreadPeerMap,
    request: WorldUnsubscribeRequest,
    manager: &mut SubscriptionManager,
) -> Result<()> {
    trace_packet!("{:?}", &request);

    let reply = process_message(peer, request, manager);
    let reply = ClientMessageReply::WorldUnsubscribe(reply);

    let mut map = peer_map.write().await;
    if let Some(peer) = map.get_mut(&peer) {
        peer.send_message(&reply.into()).await?;
    }

    Ok(())
}

fn process_message(
    peer: Uuid,
    request: WorldUnsubscribeRequest,
    manager: &mut SubscriptionManager,
) -> Status<WorldUnsubscribeReply> {
    if request.world_name == GLOBAL_WORLD {
        return ERR_WORLD_UNSUB_GLOBAL_WORLD.clone().into();
    }

    let world_name = match sanitize_world_name(&request.world_name) {
        Ok(world_name) => world_name,

        Err(error) => {
            warn!(
                "peer {} sent invalid world name: {} ({})",
                &peer, &request.world_name, error
            );

            let message = format!("invalid world name: {}", error);
            let error = err_invalid_world_name(message);

            return error.into();
        }
    };

    let updated = manager.subscribe_to_world(peer, world_name);
    let reply = WorldUnsubscribeReply::new(updated);

    Status::Ok(reply)
}
