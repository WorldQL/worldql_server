use serde::{Deserialize, Serialize};

pub use self::area_subscribe_reply::AreaSubscribeReply;
pub use self::area_unsubscribe_reply::AreaUnsubscribeReply;
pub use self::heartbeat_reply::HeartbeatReply;
pub use self::status::{Error, Status};
pub use self::world_subscribe_reply::WorldSubscribeReply;
pub use self::world_unsubscribe_reply::WorldUnsubscribeReply;
use crate::macros::{impl_into_message, impl_into_status, impl_into_super};

mod area_subscribe_reply;
mod area_unsubscribe_reply;
mod heartbeat_reply;
mod status;
mod world_subscribe_reply;
mod world_unsubscribe_reply;

/// Responses to [`crate::incoming::IncomingMessage`] requests
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "reply", rename_all = "snake_case")]
pub enum OutgoingMessageReply {
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

    // TODO
}

impl_into_super!(Heartbeat, Reply, OutgoingMessageReply);

impl_into_status!(WorldSubscribe, Reply);
impl_into_status!(WorldUnsubscribe, Reply);
impl_into_status!(AreaSubscribe, Reply);
impl_into_status!(AreaUnsubscribe, Reply);

impl_into_message!(Heartbeat, Reply, crate::outgoing::OutgoingMessage);
impl_into_message!(WorldSubscribe, Reply, crate::outgoing::OutgoingMessage);
impl_into_message!(WorldUnsubscribe, Reply, crate::outgoing::OutgoingMessage);
impl_into_message!(AreaSubscribe, Reply, crate::outgoing::OutgoingMessage);
impl_into_message!(AreaUnsubscribe, Reply, crate::outgoing::OutgoingMessage);
