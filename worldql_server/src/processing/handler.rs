use color_eyre::Result;
use flume::{Receiver, Sender};
use tracing::{debug, warn};
use uuid::Uuid;
use worldql_messages::server_bound::{ServerMessage, ServerMessagePayload};

use super::area_subscribe::handle_area_subscribe;
use super::area_unsubscribe::handle_area_unsubscribe;
use super::global_message::handle_global_message;
use super::heartbeat::handle_heartbeat;
use super::local_message::handle_local_message;
use super::world_subscribe::handle_world_subscribe;
use super::world_unsubscribe::handle_world_unsubscribe;
use crate::subscriptions::WorldMap;
use crate::transport::ThreadPeerMap;

pub async fn start_processing_thread(
    peer_map: ThreadPeerMap,
    msg_rx: Receiver<ServerMessage>,
    remove_rx: Receiver<Uuid>,
    cube_size: u16,
) -> Result<()> {
    let (sub_tx, sub_rx) = flume::unbounded();
    let (db_tx, db_rx) = flume::unbounded();

    let mut sub = tokio::spawn(handle_subscriptions(sub_rx, remove_rx, cube_size));
    let mut db = tokio::spawn(handle_database(db_rx));

    loop {
        tokio::select! {
            // Handle incoming messages
            Ok(incoming) = msg_rx.recv_async() => {
                handle_message(&sub_tx, &db_tx, &peer_map, incoming).await?;
            },

            // Exit early if subscription processing errors
            Ok(Err(error)) = &mut sub => {
                return Err(error);
            },

            // Exit early if database processing errors
            Ok(Err(error)) = &mut db => {
                return Err(error);
            },

            // All processing channels have closed, exit thread
            else => {
                debug!("start_processing_thread loop exiting");
                break
            },
        }
    }

    Ok(())
}

#[inline]
async fn handle_message(
    sub_tx: &Sender<ServerMessage>,
    db_tx: &Sender<ServerMessage>,
    peer_map: &ThreadPeerMap,
    incoming: ServerMessage,
) -> Result<()> {
    // TODO: Verify message UUID / Token pair is valid

    match &incoming.payload {
        // Ignore handshakes, they should not be resent
        ServerMessagePayload::Handshake(_) => {
            warn!("received handshake from already authenticated peer")
        }

        // Instantly handle heartbeats
        ServerMessagePayload::Heartbeat(request) => handle_heartbeat(request, peer_map).await?,

        // Handle subscription and realtime messages
        ServerMessagePayload::GlobalMessage(_)
        | ServerMessagePayload::LocalMessage(_)
        | ServerMessagePayload::WorldSubscribe(_)
        | ServerMessagePayload::WorldUnsubscribe(_)
        | ServerMessagePayload::AreaSubscribe(_)
        | ServerMessagePayload::AreaUnsubscribe(_) => {
            sub_tx.send_async(incoming).await?;
        }

        // Handle database messages
        ServerMessagePayload::RecordGet(_)
        | ServerMessagePayload::RecordSet(_)
        | ServerMessagePayload::RecordDelete(_)
        | ServerMessagePayload::RecordClear(_) => {
            db_tx.send_async(incoming).await?;
        }
    }

    Ok(())
}

async fn handle_subscriptions(
    msg_rx: Receiver<ServerMessage>,
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
            Ok(incoming) = msg_rx.recv_async() => {
                match incoming.payload {
                    ServerMessagePayload::LocalMessage(request) => handle_local_message(request, &mut world_map).await?,
                    ServerMessagePayload::GlobalMessage(request) => handle_global_message(request, &mut world_map).await?,
                    ServerMessagePayload::WorldSubscribe(request) => handle_world_subscribe(request, &mut world_map).await?,
                    ServerMessagePayload::WorldUnsubscribe(request) => handle_world_unsubscribe(request, &mut world_map).await?,
                    ServerMessagePayload::AreaSubscribe(request) => handle_area_subscribe(request, &mut world_map).await?,
                    ServerMessagePayload::AreaUnsubscribe(request) => handle_area_unsubscribe(request, &mut world_map).await?,

                    _ => panic!("invalid message type"),
                }
            },

            // Both channels have closed, exit thread
            else => {
                debug!("handle_subscriptions loop exiting");
                break
            },
        }
    }
    todo!()
}

async fn handle_database(msg_rx: Receiver<ServerMessage>) -> Result<()> {
    // TODO
    Ok(())
}
