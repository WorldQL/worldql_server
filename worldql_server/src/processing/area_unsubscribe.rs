use color_eyre::Result;
use worldql_messages::server_bound::AreaUnsubscribeRequest;

use crate::subscriptions::WorldMap;

pub(super) async fn handle_area_unsubscribe(
    request: AreaUnsubscribeRequest,
    world_map: &mut WorldMap,
) -> Result<()> {
    todo!()
}
