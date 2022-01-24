use serde::{Deserialize, Serialize};

pub use self::global_message_event::GlobalMessageEvent;
pub use self::local_message_event::LocalMessageEvent;

mod global_message_event;
mod local_message_event;

/// Events that are not tied to a request/reply pair
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum OutgoingMessageEvent {
    // TODO
    GlobalMessage(GlobalMessageEvent),
    LocalMessage(LocalMessageEvent),
}
