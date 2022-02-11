use serde::{Deserialize, Serialize};

use crate::server_bound::HeartbeatRequest;

/// Replies to [`HeartbeatRequest`](crate::server_bound::HeartbeatRequest)
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HeartbeatReply {
    /// Copied from the request to ensure data integrity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_once: Option<String>,
}

impl HeartbeatReply {
    /// Create a new [`HeartbeatReply`]
    #[inline]
    #[must_use]
    pub fn new(no_once: Option<String>) -> Self {
        Self { no_once }
    }

    /// Create a new [`HeartbeatReply`] with the `no_once` set to [`None`]
    #[inline]
    #[must_use]
    pub fn empty() -> Self {
        Self { no_once: None }
    }
}

impl From<HeartbeatRequest> for HeartbeatReply {
    fn from(request: HeartbeatRequest) -> Self {
        Self {
            no_once: request.no_once,
        }
    }
}
