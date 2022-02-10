use color_eyre::Result;
use uuid::Uuid;
use worldql_messages::server_bound::RecordClearRequest;

use crate::trace_packet;

pub(super) async fn handle_record_clear(peer: Uuid, request: RecordClearRequest) -> Result<()> {
    trace_packet!("{:?}", &request);

    todo!()
}
