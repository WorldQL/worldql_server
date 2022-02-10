use color_eyre::Result;
use uuid::Uuid;
use worldql_messages::server_bound::RecordSetRequest;

use crate::trace_packet;

pub(super) async fn handle_record_set(sender: Uuid, request: RecordSetRequest) -> Result<()> {
    trace_packet!("{:?}", &request);

    todo!()
}
