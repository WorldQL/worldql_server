use color_eyre::Result;
use tracing::debug;

use crate::structures::Message;
use crate::subscriptions::AreaMap;
use crate::transport::ThreadPeerMap;

pub async fn handle_local_message(
    message: Message,
    peer_map: &ThreadPeerMap,
    area_map: &mut AreaMap,
) -> Result<()> {
    let cube = match message.position {
        Some(pos) => pos,
        None => {
            debug!("invalid LocalMessage from peer {}, missing position", &message.sender_uuid);
            return Ok(())
        }
    };

    let peers = area_map.get_subscribed_peers(cube);
    let mut map = peer_map.write().await;
    let _ = map.broadcast_to(message, peers).await;

    Ok(())
}
