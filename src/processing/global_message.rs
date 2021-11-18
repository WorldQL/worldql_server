use color_eyre::Result;
use tracing::warn;

use crate::constants::GLOBAL_WORLD;
use crate::structures::{Message, Replication};
use crate::subscriptions::WorldMap;
use crate::trace_packet;
use crate::transport::ThreadPeerMap;

pub async fn handle_global_message(
    message: Message,
    peer_map: &ThreadPeerMap,
    world_map: &WorldMap,
) -> Result<()> {
    trace_packet!("{}", &message);

    let uuid = message.sender_uuid;
    if message.world_name == GLOBAL_WORLD {
        // Broadcast to all
        let mut map = peer_map.write().await;

        let _ = match message.replication {
            Replication::ExceptSelf => map.broadcast_except(message, uuid).await,
            Replication::IncludingSelf => map.broadcast_all(message).await,
            Replication::OnlySelf => {
                let peer = map.get_mut(&uuid);
                if peer.is_none() {
                    warn!("Missing peer {} for GlobalMessage send!", &uuid);
                    return Ok(());
                }

                let peer = peer.unwrap();
                peer.send(message).await
            }
        };
    } else {
        // Broadcast to subscribed
        let area_map = world_map.get(&message.world_name);
        if area_map.is_none() {
            // No subscriptions, return early
            return Ok(());
        }

        // We duplicate the broadcast function to avoid holding the lock for longer than we need
        let area_map = area_map.unwrap();
        match message.replication {
            Replication::ExceptSelf => {
                // Filer out self
                let peers = area_map
                    .get_subscribed_any_peers()
                    .filter(|peer| *peer != uuid);

                let mut map = peer_map.write().await;
                let _ = map.broadcast_to(message, peers).await;
            }
            Replication::IncludingSelf => {
                // Don't filter
                let peers = area_map.get_subscribed_any_peers();

                let mut map = peer_map.write().await;
                let _ = map.broadcast_to(message, peers).await;
            }
            Replication::OnlySelf => {
                // Filter out not self
                let peers = area_map
                    .get_subscribed_any_peers()
                    .filter(|peer| *peer == uuid);

                let mut map = peer_map.write().await;
                let _ = map.broadcast_to(message, peers).await;
            }
        };
    }

    Ok(())
}
