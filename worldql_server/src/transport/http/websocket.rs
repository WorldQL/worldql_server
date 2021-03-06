use std::net::{IpAddr, SocketAddr};

use color_eyre::Result;
use flume::Sender;
use futures_util::StreamExt;
use tokio::net::{TcpListener, TcpStream};
use tracing::{debug, info, trace};
use uuid::Uuid;

use crate::structures::{Instruction, Message};
use crate::transport::{Peer, ThreadPeerMap};

pub async fn start_websocket_server(
    peer_map: ThreadPeerMap,
    msg_tx: Sender<Message>,
    ws_host: IpAddr,
    ws_port: u16,
) -> Result<()> {
    let addr = SocketAddr::new(ws_host, ws_port);
    let listener = TcpListener::bind(&addr).await?;
    info!("WebSocket Server listening on {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let addr = stream.peer_addr()?;
        debug!("websocket peer address: {}", addr);

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
    msg_tx: Sender<Message>,
    addr: SocketAddr,
    raw_stream: TcpStream,
) -> Result<()> {
    let stream = tokio_tungstenite::accept_async(raw_stream).await?;
    debug!("websocket connection established: {}", &addr);

    scopeguard::defer! {
        debug!("websocket connection closed: {}", &addr);
    }

    let uuid = Uuid::new_v4();
    let (outgoing, mut incoming) = stream.split();

    let mut peer = Peer::new_ws(addr, uuid, outgoing);
    trace!("new peer: {}", &peer);

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
                debug!("peer {} did not send a handshake message", &addr);
                return Ok(());
            }

            // Only lock for as long as we need
            {
                let mut map = peer_map.write().await;
                map.insert(uuid, peer).await;
            }
        }
    }

    // Handle all other messages
    loop {
        let msg = incoming.next().await;
        match msg {
            None => {
                info!("websocket handle_connection loop exiting.");
                break;
            }
            Some(Err(error)) => {
                debug!("websocket error: {} = \"{:?}\"", &addr, error);
                break;
            }
            Some(Ok(msg)) => {
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
                if let Err(error) = msg_tx.send_async(message).await {
                    debug!("websocket error: {} = \"{}\"", &addr, error);
                    break;
                }
            }
        }
    }

    // Thread is ending, remove from peer map
    {
        let mut map = peer_map.write().await;
        map.remove(&uuid).await;
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
    let message = match Message::deserialize(&data) {
        Ok(m) => m,
        Err(error) => {
            debug!("deserialize error from peer: {}", addr);

            #[cfg(debug_assertions)]
            tracing::error!("{:?}", error);

            return ParseResult::Ignore;
        }
    };

    if message.sender_uuid != *uuid {
        debug!(
            "peer uuid is incorrect: expected {}, got {}",
            uuid, &message.sender_uuid
        );

        return ParseResult::Close;
    }

    ParseResult::Message(message)
}
