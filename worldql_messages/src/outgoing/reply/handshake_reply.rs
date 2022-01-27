use serde::{Deserialize, Serialize};

/// Replies to [`HandshakeRequest`](crate::incoming::HandshakeRequest)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HandshakeReply {
    /// Token used to identify the client's UUID
    pub auth_token: String,
}

impl HandshakeReply {
    /// Create a new [`HandshakeReply`]
    #[inline]
    #[must_use]
    pub fn new(auth_token: impl Into<String>) -> Self {
        Self {
            auth_token: auth_token.into(),
        }
    }
}
