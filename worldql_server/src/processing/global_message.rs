use color_eyre::Result;
use tracing::debug;
use uuid::Uuid;
use worldql_messages::client_bound::GlobalMessageEvent;
use worldql_messages::common::Replication;
use worldql_messages::server_bound::GlobalMessageRequest;
use worldql_subscriptions::SubscriptionManager;

use crate::errors::err_invalid_world_name;
use crate::trace_packet;
use crate::transport::ThreadPeerMap;
use crate::utils::{sanitize_world_name, GLOBAL_WORLD};

pub(super) async fn handle_global_message(
    sender: Uuid,
    peer_map: &mut ThreadPeerMap,
    request: GlobalMessageRequest,
    manager: &mut SubscriptionManager,
) -> Result<()> {
    trace_packet!("{:?}", &request);

    let event = GlobalMessageEvent::new(sender, &request.world_name, request.data);
    let event = event.into();

    if request.world_name == GLOBAL_WORLD {
        // Broadcast to everyone
        let mut map = peer_map.write().await;

        let _ = match request.replication {
            Replication::ExceptSelf => {
                let _ = map.broadcast_except(event, sender).await;
            }

            Replication::IncludingSelf => {
                let _ = map.broadcast_all(event).await;
            }

            Replication::OnlySelf => {
                if let Some(peer) = map.get_mut(&sender) {
                    let _ = peer.send_message(&event.into()).await;
                } else {
                    debug!("peer {} missing, cannot send global message event", &sender);
                }
            }
        };
    } else {
        // Broadcast to those subscribed
        let world_name = match sanitize_world_name(&request.world_name) {
            Ok(world_name) => world_name,

            Err(error) => {
                debug!(
                    "peer {} sent invalid world name: {} ({})",
                    &sender, &request.world_name, error
                );

                let message = format!("invalid world name: {}", error);
                let error = err_invalid_world_name(message);

                let mut map = peer_map.write().await;
                if let Some(peer) = map.get_mut(&sender) {
                    let _ = peer.send_message(&error.into()).await;
                } else {
                    debug!("peer {} missing, cannot send global message event", &sender);
                }

                return Ok(());
            }
        };

        // Early exit if no peers are subscribed
        let sub_count = manager.world_subscription_count(&world_name);
        if sub_count == 0 {
            return Ok(());
        }

        match request.replication {
            Replication::ExceptSelf => {
                // Filer out self
                let peers = manager
                    .get_subscribed_to_world(&world_name)
                    .filter(|uuid| *uuid != sender);

                let mut map = peer_map.write().await;
                let _ = map.broadcast_to(event, peers).await;
            }

            Replication::IncludingSelf => {
                // Don't filter
                let peers = manager.get_subscribed_to_world(&world_name);

                let mut map = peer_map.write().await;
                let _ = map.broadcast_to(event, peers).await;
            }

            Replication::OnlySelf => {
                // Filter out not self
                let peers = manager
                    .get_subscribed_to_world(&world_name)
                    .filter(|uuid| *uuid == sender);

                let mut map = peer_map.write().await;
                let _ = map.broadcast_to(event, peers).await;
            }
        }
    }

    Ok(())
}
