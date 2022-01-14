use color_eyre::Result;
use tracing::{debug, warn};

use crate::structures::Message;
use crate::subscriptions::WorldMap;
use crate::trace_packet;
use crate::transport::ThreadPeerMap;
use crate::utils::{sanitize_world_name, GLOBAL_WORLD};

pub(super) fn handle_area_subscribe(
    message: Message,
    peer_map: &ThreadPeerMap,
    world_map: &mut WorldMap,
) -> Result<()> {
    trace_packet!("{}", &message);

    // Ignore global world
    if message.world_name == GLOBAL_WORLD {
        return Ok(());
    }

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

    let cube = match message.position {
        Some(pos) => pos,
        None => {
            // TODO: Disconnect peer
            debug!(
                "invalid AreaSubscribe from peer {}, missing position",
                &uuid
            );

            return Ok(());
        }
    };

    let area_map = world_map.get_mut(&world_name);
    area_map.add_subscription(uuid, cube);

    Ok(())
}
