use std::fmt::Display;
use std::net::SocketAddr;
use std::time::{Duration, Instant};

use bytes::Bytes;
use derive_getters::Getters;
#[cfg(feature = "zeromq")]
use flume::Sender;
#[cfg(feature = "websocket")]
use futures_util::stream::SplitSink;
#[cfg(feature = "websocket")]
use futures_util::SinkExt;
use thiserror::Error;
#[cfg(feature = "websocket")]
use tokio::net::TcpStream;
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
type ZmqConnection = Sender<ZmqOutgoingPair>;

#[derive(Debug, Getters)]
pub struct Peer {
    addr: SocketAddr,
    uuid: Uuid,

    #[getter(skip)]
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

    #[cfg(feature = "zeromq")]
    pub fn new_zmq(addr: SocketAddr, uuid: Uuid, zmq_tx: ZmqConnection) -> Self {
        Self {
            addr,
            uuid,
            connection: PeerConnection::ZeroMQ((zmq_tx, Instant::now())),
        }
    }

    /// Returns `true` if the duration between the last recieved heartbeat is greater than `max_duration`
    pub fn is_stale(&self, now: &Instant, max_duration: &Duration) -> bool {
        match self.connection {
            #[cfg(feature = "websocket")]
            PeerConnection::WebSocket(_) => false,
            #[cfg(feature = "zeromq")]
            PeerConnection::ZeroMQ((_, last_heartbeat)) => {
                let duration = *now - last_heartbeat;
                duration > *max_duration
            }
        }
    }

    /// Update the Last Received [`Instant`] to the current time.
    #[cfg(feature = "zeromq")]
    #[inline]
    pub fn update_last_heartbeat(&mut self) {
        self.connection.update_last_heartbeat()
    }

    /// Send a [`Message`] to this peer.
    #[inline]
    pub async fn send(&mut self, message: Message) -> Result<(), SendError> {
        self.connection.send(self.uuid, message).await
    }

    /// Send a raw byte array to this peer.
    #[inline]
    pub async fn send_raw(&mut self, bytes: Bytes) -> Result<(), SendError> {
        self.connection.send_raw(self.uuid, bytes).await
    }
}

impl PartialEq for Peer {
    #[inline]
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
    ZeroMQ((ZmqConnection, Instant)),
}

impl PeerConnection {
    /// Update the Last Received [`Instant`] to the current time.
    #[cfg(feature = "zeromq")]
    #[inline]
    fn update_last_heartbeat(&mut self) {
        #[cfg(feature = "zeromq")]
        if let PeerConnection::ZeroMQ((_, last_recv)) = self {
            // Set the last received instant to now
            *last_recv = Instant::now()
        }
    }

    /// Send a [`Message`] to this connection.
    #[inline]
    async fn send(&mut self, uuid: Uuid, message: Message) -> Result<(), SendError> {
        let bytes = message.serialize();
        self.send_raw(uuid, bytes).await?;

        Ok(())
    }

    /// Send a raw byte array to this connection.
    #[inline]
    async fn send_raw(&mut self, uuid: Uuid, bytes: Bytes) -> Result<(), SendError> {
        match self {
            #[cfg(feature = "websocket")]
            PeerConnection::WebSocket(conn) => {
                let message = WsMessage::Binary(bytes.to_vec());
                conn.send(message).await?;

                Ok(())
            }
            #[cfg(feature = "zeromq")]
            PeerConnection::ZeroMQ((tx, _)) => {
                tx.send_async((bytes, uuid)).await?;

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
    ZmqError(#[from] flume::SendError<ZmqOutgoingPair>),
}
