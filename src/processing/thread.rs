use color_eyre::Result;
use flume::Receiver;
use tracing::debug;
use uuid::Uuid;

use super::area_subscribe::handle_area_subscribe as area_subscribe;
use super::area_unsubscribe::handle_area_unsubscribe as area_unsubscribe;
use super::global_message::handle_global_message as global_message;
use super::local_message::handle_local_message as local_message;
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
            // Handle incoming peer IDs to be removed
            Ok(peer) = remove_rx.recv_async() => {
                world_map.remove_peer(&peer);
            },

            // Handle incoming messages
            Ok(message) = msg_rx.recv_async() => {
                handle_message(message, &peer_map, &mut world_map).await?;
            },

            // Both channels have closed, exit thread
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
        // Panic on handshakes, they should never be sent to this thread.
        Instruction::Handshake => panic!("recieved handshake instruction on processing thread"),

        // Handle known instructions
        Instruction::AreaSubscribe => area_subscribe(message, peer_map, world_map).await?,
        Instruction::AreaUnsubscribe => area_unsubscribe(message, peer_map, world_map).await?,
        Instruction::GlobalMessage => global_message(message, peer_map).await?,
        Instruction::LocalMessage => local_message(message, peer_map, world_map).await?,

        // Warn on unknown or unhandled instructions
        Instruction::Unknown => debug!("unknown instruction received from {}", message.sender_uuid),
        _ => debug!("unhandled instruction: {:?}", message.instruction),
    }

    Ok(())
}
