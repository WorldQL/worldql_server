use color_eyre::Result;
use uuid::Uuid;
use worldql_messages::server_bound::WorldSubscribeRequest;

use crate::subscriptions::WorldMap;
use crate::trace_packet;

pub(super) async fn handle_world_subscribe(
    peer: Uuid,
    request: WorldSubscribeRequest,
    world_map: &mut WorldMap,
) -> Result<()> {
    trace_packet!("{:?}", &request);

    todo!()
}
