use color_eyre::Result;
use worldql_messages::server_bound::WorldSubscribeRequest;

use crate::subscriptions::WorldMap;

pub(super) async fn handle_world_subscribe(
    request: WorldSubscribeRequest,
    world_map: &mut WorldMap,
) -> Result<()> {
    todo!()
}
