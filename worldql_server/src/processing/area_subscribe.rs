use color_eyre::Result;
use worldql_messages::server_bound::AreaSubscribeRequest;

use crate::subscriptions::WorldMap;

pub(super) async fn handle_area_subscribe(
    request: AreaSubscribeRequest,
    world_map: &mut WorldMap,
) -> Result<()> {
    todo!()
}
