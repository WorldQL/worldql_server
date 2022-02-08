use color_eyre::Result;
use worldql_messages::server_bound::GlobalMessageRequest;

use crate::subscriptions::WorldMap;

pub(super) async fn handle_global_message(
    request: GlobalMessageRequest,
    world_map: &mut WorldMap,
) -> Result<()> {
    todo!()
}
