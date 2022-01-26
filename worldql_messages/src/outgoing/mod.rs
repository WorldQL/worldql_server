//! Client-bound messages

use serde::{Deserialize, Serialize};

pub use self::event::*;
pub use self::reply::*;

mod event;
mod reply;

/// Outgoing replies or events
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum OutgoingMessage {
    /// Replies
    Reply(OutgoingMessageReply),

    /// Events
    Event(OutgoingMessageEvent),
}

impl From<OutgoingMessageReply> for OutgoingMessage {
    #[inline]
    fn from(reply: OutgoingMessageReply) -> Self {
        Self::Reply(reply)
    }
}

impl From<OutgoingMessageEvent> for OutgoingMessage {
    #[inline]
    fn from(event: OutgoingMessageEvent) -> Self {
        Self::Event(event)
    }
}
