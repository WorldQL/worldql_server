use serde::{Deserialize, Serialize};

pub use self::heartbeat_reply::HeartbeatReply;
use crate::macros::{impl_into_message, impl_into_super};

mod heartbeat_reply;

/// Responses to [`crate::incoming::IncomingMessage`] requests
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "reply", rename_all = "snake_case")]
pub enum OutgoingMessageReply {
    /// Heartbeat
    Heartbeat(HeartbeatReply),

    // TODO
}

impl_into_super!(Heartbeat, Reply, OutgoingMessageReply);

impl_into_message!(Heartbeat, Reply, crate::outgoing::OutgoingMessage);
