use std::fmt::Display;
use std::net::SocketAddr;

use bytes::Bytes;
use futures_util::stream::SplitSink;
use futures_util::SinkExt;
use thiserror::Error;
use tokio::net::TcpStream;
#[cfg(feature = "zeromq")]
use tokio::sync::mpsc::UnboundedSender;
#[cfg(feature = "websocket")]
use tokio_tungstenite::tungstenite::Message as WsMessage;
#[cfg(feature = "websocket")]
use tokio_tungstenite::WebSocketStream;
use uuid::Uuid;

use crate::structures::Message;

#[cfg(feature = "websocket")]
type WebSocketConnection = SplitSink<WebSocketStream<TcpStream>, WsMessage>;

#[cfg(feature = "zeromq")]
pub type ZmqOutgoingPair = (Bytes, Uuid);
#[cfg(feature = "zeromq")]
type ZmqConnection = UnboundedSender<ZmqOutgoingPair>;

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

    #[cfg(feature = "websocket")]
    pub fn new_zmq(addr: SocketAddr, uuid: Uuid, zmq_tx: ZmqConnection) -> Self {
        Self {
            addr,
            uuid,
            connection: PeerConnection::ZeroMQ(zmq_tx),
        }
    }

    pub fn addr(&self) -> &SocketAddr {
        &self.addr
    }

    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    pub async fn send(&mut self, message: Message) -> Result<(), SendError> {
        self.connection.send(self.uuid, message).await
    }

    pub async fn send_raw(&mut self, bytes: Bytes) -> Result<(), SendError> {
        self.connection.send_raw(self.uuid, bytes).await
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
    ZeroMQ(ZmqConnection),
}

impl PeerConnection {
    async fn send(&mut self, uuid: Uuid, message: Message) -> Result<(), SendError> {
        let bytes = message.serialize();
        self.send_raw(uuid, bytes).await?;

        Ok(())
    }

    async fn send_raw(&mut self, uuid: Uuid, bytes: Bytes) -> Result<(), SendError> {
        match self {
            #[cfg(feature = "websocket")]
            PeerConnection::WebSocket(conn) => {
                let message = WsMessage::Binary(bytes.to_vec());
                conn.send(message).await?;

                Ok(())
            }
            #[cfg(feature = "zeromq")]
            PeerConnection::ZeroMQ(tx) => {
                tx.send((bytes, uuid))?;

                Ok(())
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
            PeerConnection::ZeroMQ(_) => write!(f, "ZeroMQ"),
        }
    }
}

#[derive(Debug, Error)]
pub enum SendError {
    #[cfg(feature = "websocket")]
    #[error(transparent)]
    WsError(#[from] tokio_tungstenite::tungstenite::Error),

    #[cfg(feature = "zeromq")]
    #[error(transparent)]
    ZmqError(#[from] tokio::sync::mpsc::error::SendError<ZmqOutgoingPair>),
}
