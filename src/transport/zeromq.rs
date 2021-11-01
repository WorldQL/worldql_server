use color_eyre::Result;
use tokio::sync::mpsc::UnboundedSender;

use super::ThreadPeerMap;
use crate::structures::Message;

pub async fn start_zeromq_server(
    peer_map: ThreadPeerMap,
    msg_tx: UnboundedSender<Message>,
) -> Result<()> {
    // TODO
    Ok(())
}
