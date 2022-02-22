use color_eyre::Result;
use tracing::debug;
use uuid::Uuid;
use worldql_messages::client_bound::LocalMessageEvent;
use worldql_messages::common::Replication;
use worldql_messages::server_bound::LocalMessageRequest;
use worldql_subscriptions::{Area, SubscriptionManager};

use crate::errors::{err_invalid_world_name, ERR_LOCAL_MESSAGE_GLOBAL_WORLD};
use crate::trace_packet;
use crate::transport::ThreadPeerMap;
use crate::utils::{sanitize_world_name, GLOBAL_WORLD};

pub(super) async fn handle_local_message(
    sender: Uuid,
    peer_map: &mut ThreadPeerMap,
    request: LocalMessageRequest,
    manager: &mut SubscriptionManager,
) -> Result<()> {
    trace_packet!("{:?}", &request);

    let event = LocalMessageEvent::new(sender, &request.world_name, request.position, request.data);
    let event = event.into();

    if request.world_name == GLOBAL_WORLD {
        let mut map = peer_map.write().await;
        if let Some(peer) = map.get_mut(&sender) {
            let error = ERR_LOCAL_MESSAGE_GLOBAL_WORLD.clone().into();
            // TODO: Handle errors
            let _ = peer.send_message(&error).await;
        } else {
            debug!(
                "peer {} missing, cannot send local message error event",
                &sender
            );
        }

        return Ok(());
    }

    if let Some(error) = sanitize_world_name(&request.world_name) {
        debug!(
            "peer {} sent invalid world name: {} ({})",
            &sender, &request.world_name, error
        );

        let message = format!("invalid world name: {}", error);
        let error = err_invalid_world_name(message);

        let mut map = peer_map.write().await;
        if let Some(peer) = map.get_mut(&sender) {
            // TODO: Handle errors
            let _ = peer.send_message(&error.into()).await;
        } else {
            debug!(
                "peer {} missing, cannot send local message error event",
                &sender
            );
        }

        return Ok(());
    }

    let (x, y, z) = request.position.coords();
    let area = Area::new_clamped(x, y, z, manager.area_size());

    // Early exit if no peers are subscribed
    let sub_count = manager.area_subscription_count(&request.world_name, area);
    if sub_count == 0 {
        return Ok(());
    }

    match request.replication {
        Replication::ExceptSelf => {
            // Filer out self
            let peers = manager
                .get_subscribed_to_area(&request.world_name, area)
                .filter(|uuid| *uuid != sender);

            let mut map = peer_map.write().await;
            // TODO: Handle errors
            let _ = map.broadcast_to(event, peers).await;
        }

        Replication::IncludingSelf => {
            // Don't filter
            let peers = manager.get_subscribed_to_area(&request.world_name, area);

            let mut map = peer_map.write().await;
            // TODO: Handle errors
            let _ = map.broadcast_to(event, peers).await;
        }

        Replication::OnlySelf => {
            // Filter out not self
            let peers = manager
                .get_subscribed_to_area(&request.world_name, area)
                .filter(|uuid| *uuid == sender);

            let mut map = peer_map.write().await;
            // TODO: Handle errors
            let _ = map.broadcast_to(event, peers).await;
        }
    }

    Ok(())
}
