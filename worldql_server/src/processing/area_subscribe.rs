use color_eyre::Result;
use worldql_messages::incoming::AreaSubscribeRequest;

use crate::subscriptions::WorldMap;

pub(super) async fn handle_area_subscribe(
    request: AreaSubscribeRequest,
    world_map: &mut WorldMap,
) -> Result<()> {
    todo!()
}
