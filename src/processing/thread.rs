use color_eyre::Result;
use flume::{Receiver, Sender};
use tracing::warn;
use uuid::Uuid;

use super::area_subscribe::handle_area_subscribe as area_subscribe;
use super::area_unsubscribe::handle_area_unsubscribe as area_unsubscribe;
use super::global_message::handle_global_message as global_message;
use super::heartbeat::handle_heartbeat as heartbeat;
use super::local_message::handle_local_message as local_message;
use super::record_create::handle_record_create as record_create;
use super::record_delete::handle_record_delete as record_delete;
use super::record_read::handle_record_read as record_read;
use crate::structures::{Instruction, Message};
use crate::subscriptions::WorldMap;
use crate::transport::ThreadPeerMap;
use crate::{trace_packet, DatabaseClient};

pub async fn start_processing_thread(
    database_client: DatabaseClient,
    peer_map: ThreadPeerMap,
    msg_rx: Receiver<Message>,
    remove_rx: Receiver<Uuid>,
    cube_size: u16,
) -> Result<()> {
    let (sub_tx, sub_rx) = flume::unbounded();
    let (db_tx, db_rx) = flume::unbounded();

    let mut db = tokio::spawn(handle_db_messages(db_rx, peer_map.clone(), database_client));
    let mut sub = tokio::spawn(handle_sub_messages(
        sub_rx,
        remove_rx,
        peer_map.clone(),
        cube_size,
    ));

    loop {
        tokio::select! {
            // Handle incoming messages
            Ok(message) = msg_rx.recv_async() => {
                handle_message(&sub_tx, &db_tx, &peer_map, message).await?;
            },

            // Exit early if sub processing errors
            Ok(Err(error)) = &mut sub => {
                return Err(error);
            },

            // Exit early if DB processing errors
            Ok(Err(error)) = &mut db => {
                return Err(error);
            },

            // Both channels have closed, exit thread
            else => break,
        }
    }

    Ok(())
}

#[inline]
async fn handle_message(
    sub_tx: &Sender<Message>,
    db_tx: &Sender<Message>,
    peer_map: &ThreadPeerMap,
    message: Message,
) -> Result<()> {
    match message.instruction {
        // Panic on handshakes, they should never be sent to this thread.
        Instruction::Handshake => panic!("recieved handshake instruction on processing thread"),

        // Panic on incoming client-bound instructions
        Instruction::PeerConnect | Instruction::PeerDisconnect | Instruction::RecordReply => {
            panic!("received incoming client-bound instruction")
        }

        // Instantly handle heartbeats
        Instruction::Heartbeat => heartbeat(message, peer_map).await?,

        // Handle subscription messages
        Instruction::AreaSubscribe
        | Instruction::AreaUnsubscribe
        | Instruction::GlobalMessage
        | Instruction::LocalMessage => {
            sub_tx.send_async(message).await?;
        }

        // Handle database messages
        Instruction::RecordCreate
        | Instruction::RecordRead
        | Instruction::RecordUpdate
        | Instruction::RecordDelete => {
            db_tx.send_async(message).await?;
        }

        // Warn on unknown instructions
        Instruction::Unknown => {
            let map = peer_map.read().await;
            let peer = map.get(&message.sender_uuid).unwrap();

            warn!("Unknown Instruction received from {}", peer);
            trace_packet!("{}", message);
        }
    }

    Ok(())
}

async fn handle_sub_messages(
    msg_rx: Receiver<Message>,
    remove_rx: Receiver<Uuid>,
    peer_map: ThreadPeerMap,
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
                match message.instruction {
                    Instruction::AreaSubscribe => area_subscribe(message, &peer_map, &mut world_map)?,
                    Instruction::AreaUnsubscribe => area_unsubscribe(message, &peer_map, &mut world_map)?,
                    Instruction::LocalMessage => local_message(message, &peer_map, &world_map).await?,
                    Instruction::GlobalMessage => global_message(message, &peer_map, &world_map).await?,

                    _ => panic!("invalid message type"),
                }
            },

            // Both channels have closed, exit thread
            else => break,
        }
    }

    Ok(())
}

async fn handle_db_messages(
    msg_rx: Receiver<Message>,
    peer_map: ThreadPeerMap,
    mut database_client: DatabaseClient,
) -> Result<()> {
    loop {
        let message = msg_rx.recv_async().await?;
        match message.instruction {
            Instruction::RecordCreate => {
                record_create(message, &mut database_client, &peer_map).await?
            }

            Instruction::RecordRead => {
                record_read(message, &mut database_client, &peer_map).await?
            }

            Instruction::RecordUpdate => {
                todo!()
            }

            Instruction::RecordDelete => {
                record_delete(message, &mut database_client, &peer_map).await?;
            }

            _ => panic!("invalid message type"),
        }
    }
}
