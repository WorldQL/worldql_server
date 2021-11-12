use color_eyre::Result;
use flume::Receiver;

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
            Instruction::LocalMessage => {
                // TODO: Use the area subscription lookup table.
                let uuid = message.sender_uuid;
                let mut map = peer_map.write().await;
                let _ = map.broadcast_except(message, uuid).await;
            }

            Instruction::GlobalMessage => {
                let uuid = message.sender_uuid;
                let mut map = peer_map.write().await;
                let _ = map.broadcast_except(message, uuid).await;
            }

            _ => (),
        }
    }

    Ok(())
}
