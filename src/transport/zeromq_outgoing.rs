use std::collections::HashMap;

use color_eyre::Result;
use flume::{Receiver, Sender};
use futures_util::SinkExt;
use tmq::push::Push;
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
) -> Result<()> {
    let mut sockets: SocketMap = HashMap::new();
    info!("Started ZeroMQ PUSH Manager");

    loop {
        tokio::select! {
            Ok(pair) = msg_rx.recv_async() => {
                handle_message(&peer_map, &mut sockets, pair).await?
            },
            Ok(message) = handshake_rx.recv_async() => {
                handle_handshake(&peer_map, msg_tx.clone(), &ctx, &mut sockets, message).await?
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
            map.remove(&uuid);
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
        sender_uuid: Uuid::nil(),
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
        map.insert(message.sender_uuid, peer);
    }

    Ok(())
}
