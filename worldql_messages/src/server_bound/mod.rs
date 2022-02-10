//! Server-bound messages

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use self::area_subscribe_request::AreaSubscribeRequest;
pub use self::area_unsubscribe_request::AreaUnsubscribeRequest;
pub use self::global_message_request::GlobalMessageRequest;
pub use self::handshake_request::HandshakeRequest;
pub use self::heartbeat_request::HeartbeatRequest;
pub use self::local_message_request::LocalMessageRequest;
pub use self::record_clear_request::RecordClearRequest;
pub use self::record_delete_request::RecordDeleteRequest;
pub use self::record_get_request::RecordGetRequest;
pub use self::record_set_request::RecordSetRequest;
pub use self::world_subscribe_request::WorldSubscribeRequest;
pub use self::world_unsubscribe_request::WorldUnsubscribeRequest;
use crate::macros::impl_into_super;

mod area_subscribe_request;
mod area_unsubscribe_request;
mod global_message_request;
mod handshake_request;
mod heartbeat_request;
mod local_message_request;
mod record_clear_request;
mod record_delete_request;
mod record_get_request;
mod record_set_request;
mod world_subscribe_request;
mod world_unsubscribe_request;

// region: IncomingMessage
/// Incoming Messages
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerMessage {
    /// UUID of the Client that sent this message
    pub sender: Uuid,

    /// Unique token to verify the client's UUID is correct and not being spoofed
    pub token: String,

    /// Message payload
    pub payload: ServerMessagePayload,
}

impl ServerMessage {
    /// Create a new [`ServerMessage`]
    pub fn new(sender: Uuid, token: impl Into<String>, payload: ServerMessagePayload) -> Self {
        Self {
            sender,
            token: token.into(),
            payload,
        }
    }
}
// endregion

// region: IncomingMessagePayload
/// Enum containing message instruction types
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "request", rename_all = "snake_case")]
pub enum ServerMessagePayload {
    /// Handshake
    Handshake(HandshakeRequest),

    /// Heartbeat
    Heartbeat(HeartbeatRequest),

    /// Global Message
    GlobalMessage(GlobalMessageRequest),

    /// Local Message
    LocalMessage(LocalMessageRequest),

    /// World Subscribe
    WorldSubscribe(WorldSubscribeRequest),

    /// World Unsubscribe
    WorldUnsubscribe(WorldUnsubscribeRequest),

    /// Area Subscribe
    AreaSubscribe(AreaSubscribeRequest),

    /// Area Unsubscribe
    AreaUnsubscribe(AreaUnsubscribeRequest),

    /// Record Get
    RecordGet(RecordGetRequest),

    /// Record Set
    RecordSet(RecordSetRequest),

    /// Record Delete
    RecordDelete(RecordDeleteRequest),

    /// Record Clear
    RecordClear(RecordClearRequest),
}

impl_into_super!(Handshake, Request, ServerMessagePayload);
impl_into_super!(Heartbeat, Request, ServerMessagePayload);
impl_into_super!(GlobalMessage, Request, ServerMessagePayload);
impl_into_super!(LocalMessage, Request, ServerMessagePayload);
impl_into_super!(WorldSubscribe, Request, ServerMessagePayload);
impl_into_super!(WorldUnsubscribe, Request, ServerMessagePayload);
impl_into_super!(AreaSubscribe, Request, ServerMessagePayload);
impl_into_super!(AreaUnsubscribe, Request, ServerMessagePayload);
impl_into_super!(RecordGet, Request, ServerMessagePayload);
impl_into_super!(RecordSet, Request, ServerMessagePayload);
impl_into_super!(RecordDelete, Request, ServerMessagePayload);
impl_into_super!(RecordClear, Request, ServerMessagePayload);

// region: IntoServerMessage Trait
/// Convert to an [`ServerMessage`]
pub trait IntoServerMessage {
    /// Perform the conversion
    #[must_use]
    fn into_incoming_message(self, sender: Uuid, token: impl Into<String>) -> ServerMessage;
}

impl<T: Into<ServerMessagePayload>> IntoServerMessage for T {
    #[inline]
    fn into_incoming_message(self, sender: Uuid, token: impl Into<String>) -> ServerMessage {
        ServerMessage {
            sender,
            token: token.into(),
            payload: self.into(),
        }
    }
}
// endregion
