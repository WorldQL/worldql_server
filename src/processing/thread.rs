use color_eyre::Result;
use flume::Receiver;
use tracing::{debug, warn};
use uuid::Uuid;

use super::area_subscribe::handle_area_subscribe as area_subscribe;
use super::area_unsubscribe::handle_area_unsubscribe as area_unsubscribe;
use super::global_message::handle_global_message as global_message;
use super::heartbeat::handle_heartbeat as heartbeat;
use super::local_message::handle_local_message as local_message;
use super::record_create::handle_record_create as record_create;
use super::record_read::handle_record_read as record_read;
use crate::structures::{Instruction, Message};
use crate::subscriptions::WorldMap;
use crate::transport::ThreadPeerMap;
use crate::{trace_packet, DatabaseClient};

pub async fn start_processing_thread(
    mut database_client: DatabaseClient,
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
                handle_message(message, &mut database_client, &peer_map, &mut world_map).await?;
            },

            // Both channels have closed, exit thread
            else => break,
        }
    }

    Ok(())
}

#[inline]
async fn handle_message(
    message: Message,
    database_client: &mut DatabaseClient,
    peer_map: &ThreadPeerMap,
    world_map: &mut WorldMap,
) -> Result<()> {
    match message.instruction {
        // Panic on handshakes, they should never be sent to this thread.
        Instruction::Handshake => panic!("recieved handshake instruction on processing thread"),

        // Handle known instructions
        Instruction::Heartbeat => heartbeat(message, peer_map).await?,
        Instruction::AreaSubscribe => area_subscribe(message, peer_map, world_map)?,
        Instruction::AreaUnsubscribe => area_unsubscribe(message, peer_map, world_map)?,
        Instruction::LocalMessage => local_message(message, peer_map, world_map).await?,
        Instruction::GlobalMessage => global_message(message, peer_map, world_map).await?,
        Instruction::RecordCreate => record_create(message, database_client, peer_map).await?,
        Instruction::RecordRead => record_read(message, database_client, peer_map).await?,

        // Warn on unknown instructions
        Instruction::Unknown => {
            let map = peer_map.read().await;
            let peer = map.get(&message.sender_uuid).unwrap();

            warn!("Unknown Instruction received from {}", peer);
            trace_packet!("{}", message);
        }

        // Emit debug message for unhandled instructions
        _ => debug!("unhandled instruction: {:?}", message.instruction),
    }

    Ok(())
}
