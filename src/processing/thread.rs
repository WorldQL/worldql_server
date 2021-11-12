use color_eyre::Result;
use flume::Receiver;
use tracing::debug;
use uuid::Uuid;

use super::area_subscribe::handle_area_subscribe;
use super::area_unsubscribe::handle_area_unsubscribe;
use super::global_message::handle_global_message;
use super::local_message::handle_local_message;
use crate::structures::{Instruction, Message};
use crate::subscriptions::WorldMap;
use crate::transport::ThreadPeerMap;

pub async fn start_processing_thread(
    peer_map: ThreadPeerMap,
    msg_rx: Receiver<Message>,
    remove_rx: Receiver<Uuid>,
    cube_size: u16,
) -> Result<()> {
    let mut world_map = WorldMap::new(cube_size);

    loop {
        tokio::select! {
            Ok(peer) = remove_rx.recv_async() => {
                world_map.remove_peer(&peer);
            },

            Ok(message) = msg_rx.recv_async() => {
                handle_message(message, &peer_map, &mut world_map).await?;
            },

            else => break,
        }
    }

    Ok(())
}

async fn handle_message(
    message: Message,
    peer_map: &ThreadPeerMap,
    world_map: &mut WorldMap,
) -> Result<()> {
    match message.instruction {
        Instruction::AreaSubscribe => handle_area_subscribe(message, &peer_map, world_map).await?,
        Instruction::AreaUnsubscribe => {
            handle_area_unsubscribe(message, &peer_map, world_map).await?
        }
        Instruction::GlobalMessage => handle_global_message(message, &peer_map).await?,
        Instruction::LocalMessage => handle_local_message(message, &peer_map, world_map).await?,

        _ => debug!("unhandled instruction: {:?}", message.instruction),
    }

    Ok(())
}
