use serde::{Deserialize, Serialize};

mod event;
mod reply;

pub use event::*;
pub use reply::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum OutgoingMessage {
    Reply(OutgoingMessageReply),
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
