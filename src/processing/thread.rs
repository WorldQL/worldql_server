use color_eyre::Result;
use tokio::sync::mpsc::UnboundedReceiver;

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

        let message = message.unwrap();

        // Re-broadcast all messages
        // TODO: Process messages properly
        let mut map = peer_map.write().await;
        let _ = map.broadcast(message).await;
    }

    Ok(())
}
