use color_eyre::Result;
use worldql_messages::server_bound::WorldUnsubscribeRequest;

use crate::subscriptions::WorldMap;

pub(super) async fn handle_world_unsubscribe(
    request: WorldUnsubscribeRequest,
    world_map: &mut WorldMap,
) -> Result<()> {
    todo!()
}
