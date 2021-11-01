use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use color_eyre::Result;
use futures_util::StreamExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::UnboundedSender;
use tracing::{debug, info, trace};
use uuid::Uuid;

use super::ThreadPeerMap;
use crate::structures::{Instruction, Message};
use crate::transport::Peer;

pub async fn start_websocket_server(
    peer_map: ThreadPeerMap,
    msg_tx: UnboundedSender<Message>,
    ws_port: u16,
) -> Result<()> {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), ws_port);
    let listener = TcpListener::bind(&addr).await?;
    info!("WebSocket Server listening on port {}", ws_port);

    while let Ok((stream, _)) = listener.accept().await {
        let addr = stream.peer_addr()?;
        debug!("peer address: {}", addr);

        tokio::spawn(handle_connection(
            peer_map.clone(),
            msg_tx.clone(),
            addr,
            stream,
        ));
    }

    Ok(())
}

async fn handle_connection(
    peer_map: ThreadPeerMap,
    msg_tx: UnboundedSender<Message>,
    addr: SocketAddr,
    raw_stream: TcpStream,
) -> Result<()> {
    let stream = tokio_tungstenite::accept_async(raw_stream).await?;
    debug!("websocket connection established: {}", &addr);

    scopeguard::defer! {
        debug!("websocket connection closed: {}", &addr);
    }

    let uuid = Uuid::new_v4();
    trace!("peer {} assigned uuid: {}", &addr, &uuid);

    let (outgoing, mut incoming) = stream.split();
    let mut peer = Peer::new_ws(outgoing);

    // Send client-bound handshake message
    peer.send(Message {
        instruction: Instruction::Handshake,
        parameter: Some(uuid.to_string()),
        ..Default::default()
    })
    .await?;

    // Check for handshake message
    match incoming.next().await {
        None => return Ok(()),
        Some(msg) => {
            let msg = msg?;
            let message = match parse_message(msg, &uuid, &addr) {
                ParseResult::Close => return Ok(()),
                ParseResult::Ignore => return Ok(()),
                ParseResult::Message(msg) => msg,
            };

            if message.instruction != Instruction::Handshake {
                trace!("peer {} did not send a handshake message", &addr);
                return Ok(());
            }

            // Only lock for as long as we need
            {
                let mut map = peer_map.write().await;
                map.insert(uuid, peer);
            }

            trace!("peer {} inserted into map", &addr);
        }
    }

    // Handle all other messages
    loop {
        let msg = incoming.next().await;
        match msg {
            None => break,
            Some(msg) => {
                let msg = msg?;
                let message = match parse_message(msg, &uuid, &addr) {
                    ParseResult::Close => break,
                    ParseResult::Ignore => continue,
                    ParseResult::Message(msg) => msg,
                };

                if message.instruction == Instruction::Handshake {
                    // If multiple handshakes are sent, disconnect
                    break;
                }

                // Send message to processing thread
                msg_tx.send(message)?;
            }
        }
    }

    Ok(())
}

enum ParseResult {
    Close,
    Ignore,
    Message(Message),
}

fn parse_message(
    msg: tokio_tungstenite::tungstenite::Message,
    uuid: &Uuid,
    addr: &SocketAddr,
) -> ParseResult {
    if msg.is_close() {
        return ParseResult::Close;
    }

    if !msg.is_binary() {
        return ParseResult::Ignore;
    }

    let data = msg.into_data();
    let message_result = Message::deserialize(&data);

    if message_result.is_err() {
        debug!("deserialize error from peer: {}", addr);

        #[cfg(debug_assertions)]
        tracing::error!("{}", message_result.unwrap_err());

        return ParseResult::Ignore;
    }

    let message = message_result.unwrap();
    if message.sender_uuid != *uuid {
        trace!(
            "peer uuid is incorrect: expected {}, got {}",
            uuid,
            &message.sender_uuid
        );

        return ParseResult::Close;
    }

    ParseResult::Message(message)
}
