use std::collections::HashMap;
use std::time::Duration;

use color_eyre::Result;
use flume::{Receiver, Sender};
use futures_util::SinkExt;
use tmq::push::Push;
use tokio::time;
use tracing::{debug, info};
use uuid::Uuid;

use super::{ThreadPeerMap, ZmqOutgoingPair};
use crate::structures::{Instruction, Message};
use crate::transport::Peer;

type SocketMap = HashMap<Uuid, Push>;

pub async fn start_zeromq_outgoing(
    peer_map: ThreadPeerMap,
    msg_tx: Sender<ZmqOutgoingPair>,
    msg_rx: Receiver<ZmqOutgoingPair>,
    handshake_rx: Receiver<Message>,
    ctx: tmq::Context,
    timeout_secs: u8,
) -> Result<()> {
    let mut sockets: SocketMap = HashMap::new();
    info!("Started ZeroMQ PUSH Manager");

    let interval = Duration::from_secs(timeout_secs as u64);
    let mut interval = time::interval(interval);
    interval.set_missed_tick_behavior(time::MissedTickBehavior::Skip);

    loop {
        tokio::select! {
            // Handle outgoing Message Bytes
            Ok(pair) = msg_rx.recv_async() => {
                handle_message(&peer_map, &mut sockets, pair).await?
            },

            // Handle incoming Handshake Messages
            Ok(message) = handshake_rx.recv_async() => {
                handle_handshake(&peer_map, msg_tx.clone(), &ctx, &mut sockets, message).await?
            },

            // Repeating interval, check peers which haven't sent
            // a heartbeat recently and remove
            now = interval.tick() => {
                // TODO
            },

            // Both channels have closed, exit thread
            else => break,
        }
    }

    Ok(())
}

async fn handle_message(
    peer_map: &ThreadPeerMap,
    sockets: &mut SocketMap,
    (bytes, uuid): ZmqOutgoingPair,
) -> Result<()> {
    match sockets.get_mut(&uuid) {
        Some(socket) => {
            let zmq_msg = tmq::Message::from(bytes.as_ref());
            socket.send(zmq_msg).await?;
        }
        None => {
            // Remove sockets from PeerMap if they are not in SocketMap
            let mut map = peer_map.write().await;
            map.remove(&uuid).await;
        }
    }

    Ok(())
}

async fn handle_handshake(
    peer_map: &ThreadPeerMap,
    msg_tx: Sender<ZmqOutgoingPair>,
    ctx: &tmq::Context,
    sockets: &mut SocketMap,
    message: Message,
) -> Result<()> {
    // Check for clashing UUIDs
    {
        let map = peer_map.read().await;
        if map.contains_key(&message.sender_uuid) {
            // UUID already exists, drop handshake
            return Ok(());
        }
    }

    let parameter = message.parameter.unwrap();
    let addr = match parameter.parse() {
        Ok(addr) => addr,
        Err(_) => {
            // Invalid socket address, drop handshake message
            return Ok(());
        }
    };

    let endpoint = format!("tcp://{}", &parameter);
    debug!("zeromq peer address: {}", endpoint);

    let mut socket = tmq::push(ctx).connect(&endpoint)?;
    let handshake_msg = Message {
        instruction: Instruction::Handshake,
        ..Default::default()
    };

    // Directly send handshake message back to socket
    let handshake_data = handshake_msg.serialize();
    let handshake_msg = tmq::Message::from(handshake_data.as_ref());
    socket.send(handshake_msg).await?;

    // Add peer to PeerMap and SocketMap
    {
        let mut map = peer_map.write().await;
        let peer = Peer::new_zmq(addr, message.sender_uuid, msg_tx);

        sockets.insert(message.sender_uuid, socket);
        map.insert(message.sender_uuid, peer).await;
    }

    Ok(())
}
