use std::fmt::{Display, Formatter};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use futures_util::stream::SplitSink;
use futures_util::SinkExt;
use thiserror::Error;
#[cfg(feature = "zeromq")]
use tmq::push::Push;
use tokio::net::TcpStream;
#[cfg(feature = "websocket")]
use tokio_tungstenite::tungstenite::Message as WsMessage;
#[cfg(feature = "websocket")]
use tokio_tungstenite::WebSocketStream;
use uuid::Uuid;

use crate::structures::Message;

#[cfg(feature = "websocket")]
type WebSocketConnection = SplitSink<WebSocketStream<TcpStream>, WsMessage>;

#[derive(Debug)]
pub struct Peer {
    addr: SocketAddr,
    uuid: Uuid,
    connection: PeerConnection,
}

impl Peer {
    #[cfg(feature = "websocket")]
    pub fn new_ws(addr: SocketAddr, uuid: Uuid, ws_conn: WebSocketConnection) -> Self {
        Self {
            addr,
            uuid,
            connection: PeerConnection::WebSocket(ws_conn),
        }
    }

    pub async fn send(&mut self, message: Message) -> Result<(), SendError> {
        self.connection.send(message).await
    }

    pub async fn send_raw(&mut self, bytes: Vec<u8>) -> Result<(), SendError> {
        self.connection.send_raw(bytes).await
    }
}

impl PartialEq for Peer {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

impl Display for Peer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{type={}, addr={}, uuid={}}}",
            self.connection, self.addr, self.uuid
        )
    }
}

#[derive(Debug)]
pub enum PeerConnection {
    #[cfg(feature = "websocket")]
    WebSocket(WebSocketConnection),
    #[cfg(feature = "zeromq")]
    ZeroMQ,
}

impl PeerConnection {
    async fn send(&mut self, message: Message) -> Result<(), SendError> {
        let bytes = message.serialize();
        self.send_raw(bytes).await?;

        Ok(())
    }

    async fn send_raw(&mut self, bytes: Vec<u8>) -> Result<(), SendError> {
        match self {
            #[cfg(feature = "websocket")]
            PeerConnection::WebSocket(conn) => {
                let message = WsMessage::Binary(bytes);
                conn.send(message).await?;

                Ok(())
            }
            #[cfg(feature = "zeromq")]
            PeerConnection::ZeroMQ => {
                // TODO
                todo!()
            }
        }
    }
}

impl Display for PeerConnection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(feature = "websocket")]
            PeerConnection::WebSocket(_) => write!(f, "WebSocket"),
            #[cfg(feature = "zeromq")]
            PeerConnection::ZeroMQ => write!(f, "ZeroMQ"),
        }
    }
}

#[derive(Debug, Error)]
pub enum SendError {
    #[cfg(feature = "websocket")]
    #[error(transparent)]
    WsError(#[from] tokio_tungstenite::tungstenite::Error),
}
