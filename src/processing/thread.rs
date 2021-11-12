use color_eyre::Result;
use flume::Receiver;
use tracing::debug;

use super::global_message::handle_global_message as global_message;
use super::local_message::handle_local_message as local_message;
use crate::structures::{Instruction, Message};
use crate::subscriptions::AreaMap;
use crate::transport::ThreadPeerMap;

pub async fn start_processing_thread(
    peer_map: ThreadPeerMap,
    msg_rx: Receiver<Message>,
    cube_size: u16,
) -> Result<()> {
    let mut area_map = AreaMap::new(cube_size);

    while let Ok(message) = msg_rx.recv_async().await {
        match message.instruction {
            Instruction::LocalMessage => local_message(message, &peer_map, &mut area_map).await?,
            Instruction::GlobalMessage => global_message(message, &peer_map).await?,

            _ => debug!("unhandled instruction: {:?}", message.instruction),
        }
    }

    Ok(())
}
