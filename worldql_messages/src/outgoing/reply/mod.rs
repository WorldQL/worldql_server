use serde::{Deserialize, Serialize};

pub use self::area_subscribe_reply::AreaSubscribeReply;
pub use self::area_unsubscribe_reply::AreaUnsubscribeReply;
pub use self::handshake_reply::HandshakeReply;
pub use self::heartbeat_reply::HeartbeatReply;
pub use self::record_clear_reply::RecordClearReply;
pub use self::record_delete_reply::RecordDeleteReply;
pub use self::record_get_reply::RecordGetReply;
pub use self::record_set_reply::RecordSetReply;
pub use self::status::Status;
pub use self::world_subscribe_reply::WorldSubscribeReply;
pub use self::world_unsubscribe_reply::WorldUnsubscribeReply;
use crate::macros::{impl_into_message, impl_into_status, impl_into_super};

mod area_subscribe_reply;
mod area_unsubscribe_reply;
mod handshake_reply;
mod heartbeat_reply;
mod record_clear_reply;
mod record_delete_reply;
mod record_get_reply;
mod record_set_reply;
mod status;
mod world_subscribe_reply;
mod world_unsubscribe_reply;

/// Responses to [`crate::incoming::IncomingMessage`] requests
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "reply", rename_all = "snake_case")]
pub enum OutgoingMessageReply {
    /// Handshake
    Handshake(Status<HandshakeReply>),

    /// Heartbeat
    Heartbeat(HeartbeatReply),

    /// World Subscribe
    WorldSubscribe(Status<WorldSubscribeReply>),

    /// World Unsubscribe
    WorldUnsubscribe(Status<WorldUnsubscribeReply>),

    /// Area Subscribe
    AreaSubscribe(Status<AreaSubscribeReply>),

    /// Area Unsubscribe
    AreaUnsubscribe(Status<AreaUnsubscribeReply>),

    /// Record Get
    RecordGet(Status<RecordGetReply>),

    /// Record Set
    RecordSet(Status<RecordSetReply>),

    /// Record Delete
    RecordDelete(Status<RecordDeleteReply>),

    /// Record Clear
    RecordClear(Status<RecordClearReply>),
}

impl_into_super!(Heartbeat, Reply, OutgoingMessageReply);

impl_into_status!(Handshake, Reply);
impl_into_status!(WorldSubscribe, Reply);
impl_into_status!(WorldUnsubscribe, Reply);
impl_into_status!(AreaSubscribe, Reply);
impl_into_status!(AreaUnsubscribe, Reply);
impl_into_status!(RecordGet, Reply);
impl_into_status!(RecordSet, Reply);
impl_into_status!(RecordDelete, Reply);
impl_into_status!(RecordClear, Reply);

impl_into_message!(Handshake, Reply, crate::outgoing::OutgoingMessage);
impl_into_message!(Heartbeat, Reply, crate::outgoing::OutgoingMessage);
impl_into_message!(WorldSubscribe, Reply, crate::outgoing::OutgoingMessage);
impl_into_message!(WorldUnsubscribe, Reply, crate::outgoing::OutgoingMessage);
impl_into_message!(AreaSubscribe, Reply, crate::outgoing::OutgoingMessage);
impl_into_message!(AreaUnsubscribe, Reply, crate::outgoing::OutgoingMessage);
impl_into_message!(RecordGet, Reply, crate::outgoing::OutgoingMessage);
impl_into_message!(RecordSet, Reply, crate::outgoing::OutgoingMessage);
impl_into_message!(RecordDelete, Reply, crate::outgoing::OutgoingMessage);
impl_into_message!(RecordClear, Reply, crate::outgoing::OutgoingMessage);
