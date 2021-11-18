use color_eyre::Result;
use tracing::warn;

use crate::structures::{Message, Replication};
use crate::trace_packet;
use crate::transport::ThreadPeerMap;

pub async fn handle_global_message(message: Message, peer_map: &ThreadPeerMap) -> Result<()> {
    trace_packet!("{}", &message);

    let uuid = message.sender_uuid;
    let mut map = peer_map.write().await;

    let _ = match message.replication {
        Replication::ExceptSelf => map.broadcast_except(message, uuid).await,
        Replication::IncludingSelf => map.broadcast_all(message).await,
        Replication::OnlySelf => {
            let peer = map.get_mut(&uuid);
            if peer.is_none() {
                warn!("Missing peer {} for GlobalMessage send!", &uuid);
                return Ok(());
            }

            let peer = peer.unwrap();
            peer.send(message).await
        }
    };

    Ok(())
}
