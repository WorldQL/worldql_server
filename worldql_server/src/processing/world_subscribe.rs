use color_eyre::Result;
use tracing::debug;
use uuid::Uuid;
use worldql_messages::client_bound::{ClientMessageReply, Status, WorldSubscribeReply};
use worldql_messages::server_bound::WorldSubscribeRequest;
use worldql_subscriptions::SubscriptionManager;

use crate::errors::{err_invalid_world_name, ERR_WORLD_SUB_GLOBAL_WORLD};
use crate::trace_packet;
use crate::transport::ThreadPeerMap;
use crate::utils::{sanitize_world_name, GLOBAL_WORLD};

pub(super) async fn handle_world_subscribe(
    sender: Uuid,
    peer_map: &mut ThreadPeerMap,
    request: WorldSubscribeRequest,
    manager: &mut SubscriptionManager,
) -> Result<()> {
    trace_packet!("{:?}", &request);

    let reply = process_message(sender, request, manager);
    let reply = ClientMessageReply::WorldSubscribe(reply);

    let mut map = peer_map.write().await;
    if let Some(peer) = map.get_mut(&sender) {
        peer.send_message(&reply.into()).await?;
    }

    Ok(())
}

fn process_message(
    sender: Uuid,
    request: WorldSubscribeRequest,
    manager: &mut SubscriptionManager,
) -> Status<WorldSubscribeReply> {
    if request.world_name == GLOBAL_WORLD {
        return ERR_WORLD_SUB_GLOBAL_WORLD.clone().into();
    }

    let world_name = match sanitize_world_name(&request.world_name) {
        Ok(world_name) => world_name,

        Err(error) => {
            debug!(
                "peer {} sent invalid world name: {} ({})",
                &sender, &request.world_name, error
            );

            let message = format!("invalid world name: {}", error);
            let error = err_invalid_world_name(message);

            return error.into();
        }
    };

    let updated = manager.subscribe_to_world(sender, world_name);
    let reply = WorldSubscribeReply::new(updated);

    Status::Ok(reply)
}
