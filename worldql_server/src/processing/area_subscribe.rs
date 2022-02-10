use color_eyre::Result;
use tracing::warn;
use uuid::Uuid;
use worldql_messages::client_bound::{AreaSubscribeReply, ClientMessageReply, Status};
use worldql_messages::server_bound::AreaSubscribeRequest;
use worldql_subscriptions::{Area, SubscriptionManager};

use crate::errors::{err_invalid_world_name, ERR_AREA_SUB_GLOBAL_WORLD};
use crate::trace_packet;
use crate::transport::ThreadPeerMap;
use crate::utils::{sanitize_world_name, GLOBAL_WORLD};

pub(super) async fn handle_area_subscribe(
    peer: Uuid,
    peer_map: &mut ThreadPeerMap,
    request: AreaSubscribeRequest,
    manager: &mut SubscriptionManager,
) -> Result<()> {
    trace_packet!("{:?}", &request);

    let reply = process_message(peer, request, manager);
    let reply = ClientMessageReply::AreaSubscribe(reply);

    let mut map = peer_map.write().await;
    if let Some(peer) = map.get_mut(&peer) {
        peer.send_message(&reply.into()).await?;
    }

    Ok(())
}

fn process_message(
    peer: Uuid,
    request: AreaSubscribeRequest,
    manager: &mut SubscriptionManager,
) -> Status<AreaSubscribeReply> {
    if request.world_name == GLOBAL_WORLD {
        return ERR_AREA_SUB_GLOBAL_WORLD.clone().into();
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

    let (x, y, z) = request.position.coords();
    let area = Area::new_clamped(x, y, z, manager.area_size());

    let updated = manager.subscribe_to_area(peer, world_name, area);
    let reply = AreaSubscribeReply::new(updated);

    Status::Ok(reply)
}
