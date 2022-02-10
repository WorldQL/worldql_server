use color_eyre::Result;
use uuid::Uuid;
use worldql_messages::server_bound::GlobalMessageRequest;

use crate::subscriptions::WorldMap;
use crate::trace_packet;

pub(super) async fn handle_global_message(
    sender: Uuid,
    request: GlobalMessageRequest,
    world_map: &mut WorldMap,
) -> Result<()> {
    trace_packet!("{:?}", &request);

    todo!()
}
