use serde::{Deserialize, Serialize};

/// Initiate a connection to the WorldQL Server
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HandshakeRequest {
    /// Optional server auth token
    ///
    /// Used if the WorldQL server requires authentication
    pub server_auth: Option<String>,
}

impl HandshakeRequest {
    /// Create a new [`HandshakeRequest`]
    #[inline]
    #[must_use]
    pub fn new(server_auth: Option<String>) -> Self {
        Self { server_auth }
    }
}
