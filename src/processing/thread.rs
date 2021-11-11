use color_eyre::Result;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tracing::debug;

use crate::structures::Message;
use crate::transport::ThreadPeerMap;

pub async fn start_processing_thread(
    peer_map: ThreadPeerMap,
    mut msg_rx: UnboundedReceiver<Message>,
) -> Result<()> {
    loop {
        let message = msg_rx.recv().await;
        if message.is_none() {
            break;
        }

        // TODO: Remove this unwrap.
        let message = message.unwrap();

        debug!("{:?}", message.instruction);

        // Re-broadcast all messages
        // TODO: Process messages properly
        let uuid = message.sender_uuid;
        let mut map = peer_map.write().await;
        let _ = map.broadcast_except(message, uuid).await;
    }

    Ok(())
}
