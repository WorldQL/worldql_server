use color_eyre::Result;
use uuid::Uuid;
use worldql_messages::server_bound::RecordSetRequest;

use crate::database::DatabaseClient;
use crate::trace_packet;
use crate::transport::ThreadPeerMap;

pub(super) async fn handle_record_set(
    sender: Uuid,
    peer_map: &mut ThreadPeerMap,
    request: RecordSetRequest,
    db: &mut DatabaseClient,
) -> Result<()> {
    trace_packet!("{:?}", &request);

    todo!()
}
