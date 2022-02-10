use color_eyre::Result;
use uuid::Uuid;
use worldql_messages::server_bound::RecordDeleteRequest;

use crate::trace_packet;

pub(super) async fn handle_record_delete(sender: Uuid, request: RecordDeleteRequest) -> Result<()> {
    trace_packet!("{:?}", &request);

    todo!()
}
