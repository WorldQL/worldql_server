use color_eyre::Result;
use tracing::{trace, warn};
use uuid::Uuid;

use crate::structures::Message;
use crate::trace_packet;
use crate::transport::ThreadPeerMap;

pub(super) async fn handle_heartbeat(message: Message, peer_map: &ThreadPeerMap) -> Result<()> {
    trace_packet!("{}", &message);

    let uuid = message.sender_uuid;
    let mut map = peer_map.write().await;

    trace!(
        "received heartbeat: total number of clients = {}",
        map.size()
    );

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
    #[cfg(feature = "zeromq")]
    peer.update_last_heartbeat();

    // Echo back heartbeat
    let message = Message {
        sender_uuid: Uuid::nil(),
        ..message
    };

    peer.send(message).await?;
    Ok(())
}
