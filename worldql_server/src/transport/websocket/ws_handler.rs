use std::net::{IpAddr, SocketAddr};

use color_eyre::Result;
use flume::Sender;
use futures_util::StreamExt;
use tokio::net::{TcpListener, TcpStream};
use tracing::{debug, info};
use uuid::Uuid;
use worldql_messages::incoming::{IncomingMessage, IncomingMessagePayload};
use worldql_messages::outgoing::{HandshakeReply, OutgoingMessageReply, Status};
use worldql_messages::serialization::SerializeBinary;

use crate::transport::errors::{ERR_DUPLICATE_UUID, ERR_HANDSHAKE_REQUIRED, ERR_INVALID_MESSAGE};
use crate::transport::websocket::WebSocketPeer;
use crate::transport::{Peer, ThreadPeerMap};

pub async fn start_websocket_server(
    peer_map: ThreadPeerMap,
    msg_tx: Sender<IncomingMessage>,
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
    msg_tx: Sender<IncomingMessage>,
    addr: SocketAddr,
    stream: TcpStream,
) -> Result<()> {
    let stream = tokio_tungstenite::accept_async(stream).await?;
    debug!("websocket connection established: {}", &addr);

    scopeguard::defer! {
        debug!("websocket connection closed: {}", &addr);
    }

    let (outgoing, mut incoming) = stream.split();

    // TODO: Generate this from random bytes
    let auth_token = String::new();
    let mut peer = WebSocketPeer::new(addr, Uuid::nil(), auth_token.clone(), outgoing);

    // Handle hanshake message
    match incoming.next().await {
        None => return Ok(()),
        Some(message) => {
            let message = message?;

            // Require binary message for handshake
            if !message.is_binary() {
                return Ok(());
            }

            let data = message.into_data();
            let incoming = IncomingMessage::deserialize_binary(data.into());

            let reply: Status<HandshakeReply> = match incoming {
                Err(error) => {
                    debug!("invalid websocket message: \"{}\"", error);
                    ERR_INVALID_MESSAGE.clone().into()
                },

                Ok(msg) => {
                    // Overwrite peer UUID with incoming UUID
                    peer.set_uuid(msg.sender);

                    // Check that the UUID is not already in use
                    let map = peer_map.read().await;
                    if map.contains_key(&msg.sender) {
                        ERR_DUPLICATE_UUID.clone().into()
                    } else {
                        match msg.payload {
                            IncomingMessagePayload::Handshake(_) => {
                                // TODO: Check server auth
                                // TODO: Check options

                                let reply = HandshakeReply::new(auth_token);
                                reply.into()
                            }

                            _ => ERR_HANDSHAKE_REQUIRED.clone().into(),
                        }
                    }
                }
            };

            let is_error = reply.is_error();
            let reply = OutgoingMessageReply::Handshake(reply);
            peer.send_message(&reply.into()).await?;

            // Return (disconnect) if the reply was an error
            if is_error {
                return Ok(());
            }
        }
    }

    loop {
        let msg = incoming.next().await;
        match msg {
            None => {
                debug!("websocket handler loop exiting");
                break;
            }

            Some(Err(error)) => {
                debug!("websocket error: {} = \"{:?}\"", &addr, error);
                break;
            }

            Some(Ok(message)) => {
                if message.is_close() {
                    break;
                }

                if !message.is_binary() {
                    continue;
                }

                let data = message.into_data();
                let incoming = IncomingMessage::deserialize_binary(data.into());

                // Handle decode errors
                if let Err(_) = incoming {
                    // TODO: Figure out nice way of sending generic errors
                    continue;
                }

                // Send message to processing thread
                let msg = incoming.unwrap();
                if let Err(error) = msg_tx.send_async(msg).await {
                    debug!("websocket error: {} = \"{}\"", &addr, error);
                    break;
                }
            }
        }
    }

    // Remove peer from map if the ID is not nil (unset from handshake)
    if !peer.uuid().is_nil() {
        let mut map = peer_map.write().await;
        map.remove(&peer.uuid()).await;
    }

    Ok(())
}
