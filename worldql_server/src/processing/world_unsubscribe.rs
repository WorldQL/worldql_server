use color_eyre::Result;
use uuid::Uuid;
use worldql_messages::server_bound::WorldUnsubscribeRequest;

use crate::subscriptions::WorldMap;
use crate::trace_packet;

pub(super) async fn handle_world_unsubscribe(
    peer: Uuid,
    request: WorldUnsubscribeRequest,
    world_map: &mut WorldMap,
) -> Result<()> {
    trace_packet!("{:?}", &request);

    todo!()
}
