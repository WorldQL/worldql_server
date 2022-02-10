use color_eyre::Result;
use uuid::Uuid;
use worldql_messages::server_bound::AreaSubscribeRequest;

use crate::subscriptions::WorldMap;
use crate::trace_packet;

pub(super) async fn handle_area_subscribe(
    peer: Uuid,
    request: AreaSubscribeRequest,
    world_map: &mut WorldMap,
) -> Result<()> {
    trace_packet!("{:?}", &request);

    todo!()
}
