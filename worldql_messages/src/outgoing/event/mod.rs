use serde::{Deserialize, Serialize};

pub use self::global_message_event::GlobalMessageEvent;
pub use self::local_message_event::LocalMessageEvent;
pub use self::peer_connect_event::PeerConnectEvent;
pub use self::peer_disconnect_event::PeerDisconnectEvent;
use crate::macros::impl_into_super;

mod global_message_event;
mod local_message_event;
mod peer_connect_event;
mod peer_disconnect_event;

/// Events that are not tied to a request/reply pair
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum OutgoingMessageEvent {
    PeerConnect(PeerConnectEvent),
    PeerDisconnect(PeerDisconnectEvent),
    GlobalMessage(GlobalMessageEvent),
    LocalMessage(LocalMessageEvent),
}

impl_into_super!(PeerConnect, Event, OutgoingMessageEvent);
impl_into_super!(PeerDisconnect, Event, OutgoingMessageEvent);
impl_into_super!(GlobalMessage, Event, OutgoingMessageEvent);
impl_into_super!(LocalMessage, Event, OutgoingMessageEvent);
