use std::net::SocketAddr;

use async_trait::async_trait;
use bytes::Bytes;
use futures_util::stream::SplitSink;
use futures_util::SinkExt;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message as WsMessage;
use tokio_tungstenite::WebSocketStream;
use uuid::Uuid;
use worldql_messages::client_bound::ClientMessage;
use worldql_messages::serialization::SerializeBinary;

use crate::transport::{Peer, SendError};

type WebSocketConnection = SplitSink<WebSocketStream<TcpStream>, WsMessage>;

pub struct WebSocketPeer {
    addr: SocketAddr,
    uuid: Uuid,
    token: String,
    connection: WebSocketConnection,
}

impl WebSocketPeer {
    pub fn new(
        addr: SocketAddr,
        uuid: Uuid,
        token: String,
        connection: WebSocketConnection,
    ) -> Self {
        Self {
            addr,
            uuid,
            token,
            connection,
        }
    }

    pub fn set_uuid(&mut self, uuid: Uuid) {
        self.uuid = uuid
    }
}

#[async_trait]
impl Peer for WebSocketPeer {
    #[inline(always)]
    fn type_string(&self) -> &'static str {
        "WebSocket"
    }

    #[inline]
    fn addr(&self) -> SocketAddr {
        self.addr
    }

    #[inline]
    fn uuid(&self) -> Uuid {
        self.uuid
    }

    #[inline]
    fn token(&self) -> &str {
        &self.token
    }

    #[inline(always)]
    fn update_heartbeat(&mut self) {
        // No-op
    }

    #[inline(always)]
    fn is_stale(&self, _: &std::time::Instant, _: &std::time::Duration) -> bool {
        // No-op
        false
    }

    async fn send_message(&mut self, message: &ClientMessage) -> Result<(), SendError> {
        let bytes = message.serialize_binary()?;
        self.send_bytes(&bytes).await?;

        Ok(())
    }

    async fn send_bytes(&mut self, bytes: &Bytes) -> Result<(), SendError> {
        let message = WsMessage::Binary(bytes.to_vec());
        self.connection.send(message).await?;

        Ok(())
    }
}
