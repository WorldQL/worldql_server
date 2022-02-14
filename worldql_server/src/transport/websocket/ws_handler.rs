use std::net::{IpAddr, SocketAddr};

use color_eyre::Result;
use flume::Sender;
use futures_util::StreamExt;
use tokio::net::{TcpListener, TcpStream};
use tracing::{debug, info};
use uuid::Uuid;
use worldql_messages::client_bound::{
    ClientMessageReply, HandshakeReply, Status, SystemMessageEvent,
};
use worldql_messages::serialization::SerializeBinary;
use worldql_messages::server_bound::{ServerMessage, ServerMessagePayload};

use crate::errors::{ERR_DUPLICATE_UUID, ERR_HANDSHAKE_REQUIRED, ERR_INVALID_MESSAGE};
use crate::transport::auth::authenticate_handshake;
use crate::transport::websocket::WebSocketPeer;
use crate::transport::{Peer, ThreadPeerMap};

pub async fn start_websocket_server(
    peer_map: ThreadPeerMap,
    msg_tx: Sender<ServerMessage>,
    server_token: Option<String>,
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
            server_token.clone(),
            addr,
            stream,
        ));
    }

    Ok(())
}

async fn handle_connection(
    peer_map: ThreadPeerMap,
    msg_tx: Sender<ServerMessage>,
    server_token: Option<String>,
    addr: SocketAddr,
    stream: TcpStream,
) -> Result<()> {
    let stream = tokio_tungstenite::accept_async(stream).await?;
    debug!("websocket connection established: {}", &addr);

    scopeguard::defer! {
        debug!("websocket connection closed: {}", &addr);
    }

    let (outgoing, mut incoming) = stream.split();

    let client_token = crate::utils::crypto_secure_token();
    let mut peer = WebSocketPeer::new(addr, Uuid::nil(), client_token.clone(), outgoing);

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
            let incoming = ServerMessage::deserialize_binary(data.into());

            let reply: Status<HandshakeReply> = match incoming {
                Err(error) => {
                    debug!("invalid websocket message: {} = \"{}\"", &addr, error);
                    ERR_INVALID_MESSAGE.clone().into()
                }

                Ok(msg) => {
                    // Overwrite peer UUID with incoming UUID
                    peer.set_uuid(msg.sender);

                    // Check that the UUID is not already in use
                    let map = peer_map.read().await;
                    if map.contains_key(&msg.sender) {
                        ERR_DUPLICATE_UUID.clone().into()
                    } else {
                        match msg.payload {
                            ServerMessagePayload::Handshake(request) => {
                                // Authenticate client if required
                                let auth_error = authenticate_handshake(server_token, request);

                                match auth_error {
                                    Some(error) => error.into(),
                                    None => {
                                        let reply = HandshakeReply::new(client_token);
                                        reply.into()
                                    }
                                }
                            }

                            _ => ERR_HANDSHAKE_REQUIRED.clone().into(),
                        }
                    }
                }
            };

            let is_error = reply.is_error();
            let reply = ClientMessageReply::Handshake(reply);
            peer.send_message(&reply.into()).await?;

            // Return (disconnect) if the reply was an error
            if is_error {
                let system_message = SystemMessageEvent::new_disconnect("handshake failed");
                peer.send_message(&system_message.into()).await?;

                return Ok(());
            }
        }
    }

    // Handshake complete, add peer to map
    let uuid = peer.uuid();
    {
        let mut map = peer_map.write().await;
        let peer = Box::new(peer);

        map.insert(uuid, peer).await;
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
                let incoming = ServerMessage::deserialize_binary(data.into());

                // Handle decode errors
                if let Err(error) = incoming {
                    debug!("invalid websocket message: {} = \"{}\"", &addr, error);

                    let mut map = peer_map.write().await;
                    if let Some(peer) = map.get_mut(&uuid) {
                        let reply = ERR_INVALID_MESSAGE.clone().into();
                        let _ = peer.send_message(&reply).await;
                    }

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
    if !uuid.is_nil() {
        let mut map = peer_map.write().await;
        map.remove(&uuid).await;
    }

    Ok(())
}
