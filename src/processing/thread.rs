use color_eyre::Result;
use tokio::sync::mpsc::UnboundedReceiver;
use uuid::Uuid;

use crate::structures::Message;
use crate::transport::ThreadPeerMap;

pub type MessagePair = (Uuid, Message);

pub async fn start_processing_thread(
    peer_map: ThreadPeerMap,
    mut msg_rx: UnboundedReceiver<MessagePair>,
) -> Result<()> {
    loop {
        let message = msg_rx.recv().await;
        if message.is_none() {
            break;
        }

        // TODO: Process incoming messages
        dbg!(message);
    }

    Ok(())
}
