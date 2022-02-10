use color_eyre::Result;
use uuid::Uuid;
use worldql_messages::server_bound::AreaSubscribeRequest;
use tracing::warn;

use crate::subscriptions::WorldMap;
use crate::trace_packet;
use crate::utils::{GLOBAL_WORLD, sanitize_world_name};

pub(super) fn handle_area_subscribe(
    peer: Uuid,
    request: AreaSubscribeRequest,
    world_map: &mut WorldMap,
) -> Result<()> {
    trace_packet!("{:?}", &request);

    // Ignore global world
    if request.world_name == GLOBAL_WORLD {
        return Ok(());
    }

    let world_name = match sanitize_world_name(&request.world_name) {
        Ok(world_name) => world_name,
        Err(error) => {
            warn!(
                "peer {} sent invalid world name: {} ({})",
                &peer, &request.world_name, error
            );

            return Ok(());
        }
    };

    let area_map = world_map.get_mut(&world_name);
    area_map.add_subscription(peer, request.position);

    Ok(())
}
