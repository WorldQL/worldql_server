use color_eyre::Result;
use uuid::Uuid;
use worldql_messages::server_bound::RecordGetRequest;

use crate::trace_packet;

pub(super) async fn handle_record_get(peer: Uuid, request: RecordGetRequest) -> Result<()> {
    trace_packet!("{:?}", &request);

    todo!()
}
