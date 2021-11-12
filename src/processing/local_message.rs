use color_eyre::Result;

use crate::structures::Message;
use crate::subscriptions::AreaMap;
use crate::transport::ThreadPeerMap;

pub async fn handle_local_message(
    message: Message,
    peer_map: &ThreadPeerMap,
    area_map: &mut AreaMap,
) -> Result<()> {
    // TODO: Use the area subscription lookup table.
    let uuid = message.sender_uuid;
    let mut map = peer_map.write().await;
    let _ = map.broadcast_except(message, uuid).await;

    Ok(())
}
