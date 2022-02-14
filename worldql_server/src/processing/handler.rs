use color_eyre::Result;
use flume::{Receiver, Sender};
use tracing::{debug, warn};
use uuid::Uuid;
use worldql_messages::server_bound::{ServerMessage, ServerMessagePayload};
use worldql_subscriptions::SubscriptionManager;

use super::area_subscribe::handle_area_subscribe;
use super::area_unsubscribe::handle_area_unsubscribe;
use super::global_message::handle_global_message;
use super::heartbeat::handle_heartbeat;
use super::local_message::handle_local_message;
use super::record_clear::handle_record_clear;
use super::record_delete::handle_record_delete;
use super::record_get::handle_record_get;
use super::record_set::handle_record_set;
use super::world_subscribe::handle_world_subscribe;
use super::world_unsubscribe::handle_world_unsubscribe;
use crate::database::DatabaseClient;
use crate::transport::ThreadPeerMap;

pub async fn start_processing_thread(
    peer_map: ThreadPeerMap,
    db: DatabaseClient,
    msg_rx: Receiver<ServerMessage>,
    remove_rx: Receiver<Uuid>,
    cube_size: u16,
) -> Result<()> {
    let (sub_tx, sub_rx) = flume::unbounded();
    let (db_tx, db_rx) = flume::unbounded();

    let mut db = tokio::spawn(handle_database(peer_map.clone(), db, db_rx));
    let mut sub = tokio::spawn(handle_subscriptions(
        peer_map.clone(),
        sub_rx,
        remove_rx,
        cube_size,
    ));

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
    // Verify message UUID / token pair is valid
    {
        let map = peer_map.read().await;
        if let Some(peer) = map.get(&incoming.sender) {
            let authed = peer.verify_token(&incoming.token);
            if !authed {
                warn!(
                    "Peer {} sent a message with an invalid auth token",
                    &incoming.sender
                );
                return Ok(());
            }
        } else {
            warn!("Peer {} sent a message, but is unknown", &incoming.sender);
            return Ok(());
        }
    }

    let peer = incoming.sender;
    match incoming.payload {
        // Ignore handshakes, they should not be resent
        ServerMessagePayload::Handshake(_) => {
            warn!(
                "Received Handshake Request from already authenticated Peer: {}",
                &incoming.sender
            );
        }

        // Instantly handle heartbeats
        ServerMessagePayload::Heartbeat(request) => {
            handle_heartbeat(peer, request, peer_map).await?
        }

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
    mut peer_map: ThreadPeerMap,
    msg_rx: Receiver<ServerMessage>,
    remove_rx: Receiver<Uuid>,
    cube_size: u16,
) -> Result<()> {
    let mut manager = SubscriptionManager::new(cube_size);

    loop {
        tokio::select! {
            // Handle incoming peer IDs to be removed
            Ok(peer) = remove_rx.recv_async() => {
                manager.remove_peer(peer);
            },

            // Handle incoming messages
            Ok(incoming) = msg_rx.recv_async() => {
                let peer = incoming.sender;
                match incoming.payload {
                    ServerMessagePayload::LocalMessage(request) => handle_local_message(peer, &mut peer_map, request, &mut manager).await?,
                    ServerMessagePayload::GlobalMessage(request) => handle_global_message(peer, &mut peer_map, request, &mut manager).await?,
                    ServerMessagePayload::WorldSubscribe(request) => handle_world_subscribe(peer, &mut peer_map, request, &mut manager).await?,
                    ServerMessagePayload::WorldUnsubscribe(request) => handle_world_unsubscribe(peer, &mut peer_map, request, &mut manager).await?,
                    ServerMessagePayload::AreaSubscribe(request) => handle_area_subscribe(peer, &mut peer_map, request, &mut manager).await?,
                    ServerMessagePayload::AreaUnsubscribe(request) => handle_area_unsubscribe(peer, &mut peer_map, request, &mut manager).await?,

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

    Ok(())
}

async fn handle_database(
    mut peer_map: ThreadPeerMap,
    mut db: DatabaseClient,
    msg_rx: Receiver<ServerMessage>,
) -> Result<()> {
    loop {
        let incoming = msg_rx.recv_async().await?;
        let peer = incoming.sender;

        match incoming.payload {
            ServerMessagePayload::RecordGet(request) => {
                handle_record_get(peer, &mut peer_map, request, &mut db).await?
            }

            ServerMessagePayload::RecordSet(request) => {
                handle_record_set(peer, &mut peer_map, request, &mut db).await?
            }

            ServerMessagePayload::RecordDelete(request) => {
                handle_record_delete(peer, &mut peer_map, request, &mut db).await?
            }

            ServerMessagePayload::RecordClear(request) => {
                handle_record_clear(peer, &mut peer_map, request, &mut db).await?
            }

            _ => panic!("invalid message type"),
        }
    }
}
