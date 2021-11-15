use color_eyre::Result;

use crate::trace_packet;
use crate::structures::Message;
use crate::transport::ThreadPeerMap;

pub async fn handle_global_message(message: Message, peer_map: &ThreadPeerMap) -> Result<()> {
    trace_packet!("{}", &message);

    let uuid = message.sender_uuid;
    let mut map = peer_map.write().await;
    let _ = map.broadcast_except(message, uuid).await;

    Ok(())
}
