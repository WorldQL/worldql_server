use color_eyre::Result;
use tracing::warn;
use uuid::Uuid;

use crate::structures::Message;
use crate::transport::ThreadPeerMap;

pub async fn handle_heartbeat(message: Message, peer_map: &ThreadPeerMap) -> Result<()> {
    let uuid = message.sender_uuid;
    let mut map = peer_map.write().await;
    let peer = match map.get_mut(&uuid) {
        Some(peer) => peer,
        None => {
            warn!(
                "missing peer: {}\nplease report to worldql developers",
                &uuid
            );

            return Ok(());
        }
    };

    // Update last received time
    peer.update_last_heartbeat();

    // Echo back heartbeat
    let message = Message {
        sender_uuid: Uuid::nil(),
        ..message
    };

    peer.send(message).await?;
    Ok(())
}
