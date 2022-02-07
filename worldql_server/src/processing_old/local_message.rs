use color_eyre::Result;
use tracing::{debug, warn};

use crate::structures::{Message, Replication};
use crate::subscriptions::WorldMap;
use crate::trace_packet;
use crate::transport::ThreadPeerMap;
use crate::utils::{sanitize_world_name, GLOBAL_WORLD};

pub(super) async fn handle_local_message(
    message: Message,
    peer_map: &ThreadPeerMap,
    world_map: &WorldMap,
) -> Result<()> {
    trace_packet!("{}", &message);

    if message.world_name == GLOBAL_WORLD {
        debug!(
            "invalid LocalMessage from peer {}, uses \"@global\" world",
            &message.sender_uuid
        );

        return Ok(());
    }

    let cube = match message.position {
        Some(pos) => pos,
        None => {
            // TODO: Disconnect peer
            debug!(
                "invalid LocalMessage from peer {}, missing position",
                &message.sender_uuid
            );

            return Ok(());
        }
    };

    let uuid = message.sender_uuid;
    let world_name = match sanitize_world_name(&message.world_name) {
        Ok(world_name) => world_name,
        Err(error) => {
            warn!(
                "peer {} sent invalid world name: {} ({})",
                uuid, &message.world_name, error
            );

            return Ok(());
        }
    };

    let area_map = world_map.get(&world_name);
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
                .get_subscribed_peers(cube)
                .filter(|peer| *peer != uuid);

            let mut map = peer_map.write().await;
            let _ = map.broadcast_to(message, peers).await;
        }
        Replication::IncludingSelf => {
            // Don't filter
            let peers = area_map.get_subscribed_peers(cube);

            let mut map = peer_map.write().await;
            let _ = map.broadcast_to(message, peers).await;
        }
        Replication::OnlySelf => {
            // Filter out not self
            let peers = area_map
                .get_subscribed_peers(cube)
                .filter(|peer| *peer == uuid);

            let mut map = peer_map.write().await;
            let _ = map.broadcast_to(message, peers).await;
        }
    };

    Ok(())
}
