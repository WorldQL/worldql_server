use color_eyre::Result;
use uuid::Uuid;
use worldql_messages::server_bound::LocalMessageRequest;

use crate::subscriptions::WorldMap;
use crate::trace_packet;

pub(super) async fn handle_local_message(
    peer: Uuid,
    request: LocalMessageRequest,
    world_map: &mut WorldMap,
) -> Result<()> {
    trace_packet!("{:?}", &request);

    todo!()
}
