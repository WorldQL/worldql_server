//! Client-bound messages

use serde::{Deserialize, Serialize};

pub use self::error::*;
pub use self::event::*;
pub use self::reply::*;

mod error;
mod event;
mod reply;

/// Outgoing replies or events
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientMessage {
    /// Replies
    Reply(ClientMessageReply),

    /// Events
    Event(ClientMessageEvent),
}

impl From<ClientMessageReply> for ClientMessage {
    #[inline]
    fn from(reply: ClientMessageReply) -> Self {
        Self::Reply(reply)
    }
}

impl From<ClientMessageEvent> for ClientMessage {
    #[inline]
    fn from(event: ClientMessageEvent) -> Self {
        Self::Event(event)
    }
}
