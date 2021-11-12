use color_eyre::Result;
use flume::Receiver;
use tracing::debug;

use super::area_subscribe::handle_area_subscribe;
use super::area_unsubscribe::handle_area_unsubscribe;
use super::global_message::handle_global_message;
use super::local_message::handle_local_message;
use crate::structures::{Instruction, Message};
use crate::subscriptions::AreaMap;
use crate::transport::ThreadPeerMap;

pub async fn start_processing_thread(
    peer_map: ThreadPeerMap,
    msg_rx: Receiver<Message>,
    cube_size: u16,
) -> Result<()> {
    // TODO: Lookups per world
    let mut area_map = AreaMap::new(cube_size);

    while let Ok(message) = msg_rx.recv_async().await {
        match message.instruction {
            Instruction::AreaSubscribe => {
                handle_area_subscribe(message, &peer_map, &mut area_map).await?
            }
            Instruction::AreaUnsubscribe => {
                handle_area_unsubscribe(message, &peer_map, &mut area_map).await?
            }
            Instruction::GlobalMessage => handle_global_message(message, &peer_map).await?,
            Instruction::LocalMessage => {
                handle_local_message(message, &peer_map, &mut area_map).await?
            }

            _ => debug!("unhandled instruction: {:?}", message.instruction),
        }
    }

    Ok(())
}
