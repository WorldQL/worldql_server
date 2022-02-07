use color_eyre::Result;
use worldql_messages::incoming::LocalMessageRequest;

use crate::subscriptions::WorldMap;

pub(super) async fn handle_local_message(
    request: LocalMessageRequest,
    world_map: &mut WorldMap,
) -> Result<()> {
    todo!()
}
