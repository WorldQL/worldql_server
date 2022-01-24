use serde::{Deserialize, Serialize};

/// Used to signal to the server that a connection is still alive
///
/// Only used on connection types that don't
/// have their own native heartbeat implementation
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HeartbeatRequest {
    /// No-once
    ///
    /// Will be echoed back to the sender to ensure data integrity
    pub no_once: Option<String>,
}

impl HeartbeatRequest {
    /// Create a new [`HeartbeatRequest`]
    #[inline]
    #[must_use]
    pub fn new(no_once: Option<String>) -> Self {
        Self { no_once }
    }

    /// Create an empty [`HeartbeatRequest`]
    ///
    /// Equivalent to creating a [`HeartbeatRequest`] with `no_once` set to [`None`]
    #[inline]
    #[must_use]
    pub fn empty() -> Self {
        Self { no_once: None }
    }
}
