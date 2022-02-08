use serde::{Deserialize, Serialize};

use crate::client_bound::Error;

/// Emitted when the server needs to send a message to any clients
///
/// Might be sent globally or might be sent directly to a single client
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "message", rename_all = "snake_case")]
pub enum SystemMessageEvent {
    /// Indicates that an unknown error occurred, outside of any request / reply pairs
    UnknownError(Error),

    /// Indicates the connection is about to be terminated
    Disconnect {
        /// Reason for terminating the connection
        reason: String,
    },
}

impl SystemMessageEvent {
    /// Create a new [`SystemMessageEvent`] with the reason set to [`UnknownError`](SystemMessageEvent::UnknownError)
    #[inline]
    #[must_use]
    pub fn new_unknown_error(error: Error) -> Self {
        Self::UnknownError(error)
    }

    /// Create a new [`SystemMessageEvent`] with the reason set to [`Disconnect`](SystemMessageEvent::Disconnect)
    #[inline]
    #[must_use]
    pub fn new_disconnect(reason: impl Into<String>) -> Self {
        Self::Disconnect {
            reason: reason.into(),
        }
    }
}

impl From<Error> for SystemMessageEvent {
    #[inline]
    fn from(error: Error) -> Self {
        Self::UnknownError(error)
    }
}

impl From<Error> for crate::client_bound::OutgoingMessage {
    #[inline]
    fn from(error: Error) -> Self {
        let event: SystemMessageEvent = error.into();
        event.into()
    }
}
