use color_eyre::Result;
use uuid::Uuid;
use worldql_messages::server_bound::RecordClearRequest;

use crate::database::DatabaseClient;
use crate::trace_packet;
use crate::transport::ThreadPeerMap;

pub(super) async fn handle_record_clear(
    sender: Uuid,
    peer_map: &mut ThreadPeerMap,
    request: RecordClearRequest,
    db: &mut DatabaseClient,
) -> Result<()> {
    trace_packet!("{:?}", &request);

    todo!()
}
