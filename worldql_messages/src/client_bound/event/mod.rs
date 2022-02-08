use serde::{Deserialize, Serialize};

pub use self::global_message_event::GlobalMessageEvent;
pub use self::local_message_event::LocalMessageEvent;
pub use self::peer_connect_event::PeerConnectEvent;
pub use self::peer_disconnect_event::PeerDisconnectEvent;
pub use self::system_message::SystemMessageEvent;
use crate::macros::{impl_into_message, impl_into_super};

mod global_message_event;
mod local_message_event;
mod peer_connect_event;
mod peer_disconnect_event;
mod system_message;

/// Events that are not tied to a request/reply pair
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum ClientMessageEvent {
    /// System Message
    SystemMessage(SystemMessageEvent),

    /// Peer Connect
    PeerConnect(PeerConnectEvent),

    /// Peer Disconnect
    PeerDisconnect(PeerDisconnectEvent),

    /// Global Message
    GlobalMessage(GlobalMessageEvent),

    /// Local Message
    LocalMessage(LocalMessageEvent),
}

impl_into_super!(SystemMessage, Event, ClientMessageEvent);
impl_into_super!(PeerConnect, Event, ClientMessageEvent);
impl_into_super!(PeerDisconnect, Event, ClientMessageEvent);
impl_into_super!(GlobalMessage, Event, ClientMessageEvent);
impl_into_super!(LocalMessage, Event, ClientMessageEvent);

impl_into_message!(SystemMessage, Event, super::ClientMessage);
impl_into_message!(PeerConnect, Event, super::ClientMessage);
impl_into_message!(PeerDisconnect, Event, super::ClientMessage);
impl_into_message!(GlobalMessage, Event, super::ClientMessage);
impl_into_message!(LocalMessage, Event, super::ClientMessage);
