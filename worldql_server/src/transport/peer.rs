use std::net::SocketAddr;
use std::time::{Duration, Instant};

use async_trait::async_trait;
use bytes::Bytes;
use thiserror::Error;
use uuid::Uuid;
use worldql_messages::client_bound::ClientMessage;

#[async_trait]
pub trait Peer {
    /// Returns the type connection this peer is using
    #[must_use]
    fn type_string(&self) -> &'static str;

    /// Return the socket address for this peer
    #[must_use]
    fn addr(&self) -> SocketAddr;

    /// Return the unique ID for this peer
    #[must_use]
    fn uuid(&self) -> Uuid;

    /// Return the auth token for this peer
    #[must_use]
    fn token(&self) -> &str;

    /// Returns `true` if the token matches this peer's auth token
    #[must_use]
    #[inline]
    fn verify_token(&self, token: &str) -> bool {
        token == self.token()
    }

    /// Update the last heartbeat time for this connection
    ///
    /// Might be a no-op
    fn update_heartbeat(&mut self);

    /// Return whether this connection is stale based on the last heartbeat
    ///
    /// Might be a no-op
    fn is_stale(&self, now: &Instant, max_duration: &Duration) -> bool;

    /// Send a [`ClientMessage`] to this peer
    async fn send_message(&mut self, message: &ClientMessage) -> Result<(), SendError>;

    /// Send raw [`Bytes`] to this peer
    async fn send_bytes(&mut self, bytes: &Bytes) -> Result<(), SendError>;
}

impl PartialEq for dyn Peer {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.uuid() == other.uuid()
    }
}

#[derive(Debug, Error)]
pub enum SendError {
    #[error(transparent)]
    SerializeError(#[from] worldql_messages::serialization::EncodeError),

    #[cfg(feature = "websocket")]
    #[error(transparent)]
    WsError(#[from] tokio_tungstenite::tungstenite::Error),
}
