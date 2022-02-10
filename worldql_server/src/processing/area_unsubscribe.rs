use color_eyre::Result;
use uuid::Uuid;
use worldql_messages::server_bound::AreaUnsubscribeRequest;

use crate::subscriptions::WorldMap;
use crate::trace_packet;

pub(super) async fn handle_area_unsubscribe(
    peer: Uuid,
    request: AreaUnsubscribeRequest,
    world_map: &mut WorldMap,
) -> Result<()> {
    trace_packet!("{:?}", &request);

    todo!()
}
