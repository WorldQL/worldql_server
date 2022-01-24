use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use self::area_subscribe_request::AreaSubscribeRequest;
pub use self::area_unsubscribe_request::AreaUnsubscribeRequest;
pub use self::global_message_request::GlobalMessageRequest;
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
mod heartbeat_request;
mod local_message_request;
mod record_clear_request;
mod record_delete_request;
mod record_get_request;
mod record_set_request;
mod world_subscribe_request;
mod world_unsubscribe_request;

// region: IncomingMessage
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IncomingMessage {
    /// UUID of the Client that sent this message
    pub sender: Uuid,

    /// Unique token to verify the client's UUID is correct and not being spoofed
    pub token: String,

    /// Message payload
    pub payload: IncomingMessagePayload,
}

impl IncomingMessage {
    pub fn new(sender: Uuid, token: String, payload: IncomingMessagePayload) -> Self {
        Self {
            sender,
            token,
            payload,
        }
    }
}
// endregion

// region: IncomingMessagePayload
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "request", rename_all = "snake_case")]
pub enum IncomingMessagePayload {
    Heartbeat(HeartbeatRequest),
    GlobalMessage(GlobalMessageRequest),
    LocalMessage(LocalMessageRequest),
    WorldSubscribe(WorldSubscribeRequest),
    WorldUnsubscribe(WorldUnsubscribeRequest),
    AreaSubscribe(AreaSubscribeRequest),
    AreaUnsubscribe(AreaUnsubscribeRequest),
    RecordGet(RecordGetRequest),
    RecordSet(RecordSetRequest),
    RecordDelete(RecordDeleteRequest),
    RecordClear(RecordClearRequest),
}

impl_into_super!(Heartbeat, Request, IncomingMessagePayload);
impl_into_super!(GlobalMessage, Request, IncomingMessagePayload);
impl_into_super!(LocalMessage, Request, IncomingMessagePayload);
impl_into_super!(WorldSubscribe, Request, IncomingMessagePayload);
impl_into_super!(WorldUnsubscribe, Request, IncomingMessagePayload);
impl_into_super!(AreaSubscribe, Request, IncomingMessagePayload);
impl_into_super!(AreaUnsubscribe, Request, IncomingMessagePayload);
impl_into_super!(RecordGet, Request, IncomingMessagePayload);
impl_into_super!(RecordSet, Request, IncomingMessagePayload);
impl_into_super!(RecordDelete, Request, IncomingMessagePayload);
impl_into_super!(RecordClear, Request, IncomingMessagePayload);

// region: IntoIncomingMessage Trait
pub trait IntoIncomingMessage {
    /// Convert an [`IncomingMessagePayload`] into an [`IncomingMessage`]
    #[must_use]
    fn into_incoming_message(self, sender: Uuid, token: String) -> IncomingMessage;
}

impl<T: Into<IncomingMessagePayload>> IntoIncomingMessage for T {
    #[inline]
    fn into_incoming_message(self, sender: Uuid, token: String) -> IncomingMessage {
        IncomingMessage {
            sender,
            token,
            payload: self.into(),
        }
    }
}
// endregion
