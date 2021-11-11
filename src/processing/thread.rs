use color_eyre::Result;
use flume::Receiver;
use tracing::debug;

use crate::structures::Message;
use crate::transport::ThreadPeerMap;

pub async fn start_processing_thread(
    peer_map: ThreadPeerMap,
    msg_rx: Receiver<Message>,
) -> Result<()> {
    while let Ok(message) = msg_rx.recv_async().await {
        debug!("{:?}", message.instruction);

        // Re-broadcast all messages
        // TODO: Process messages properly
        let uuid = message.sender_uuid;
        let mut map = peer_map.write().await;
        let _ = map.broadcast_except(message, uuid).await;
    }

    Ok(())
}
