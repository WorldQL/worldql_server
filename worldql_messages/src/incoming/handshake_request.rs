use serde::{Deserialize, Serialize};

/// Initiate a connection to the WorldQL Server
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HandshakeRequest {
    /// Optional server auth token
    ///
    /// Used if the WorldQL server requires authentication
    pub server_auth: Option<String>,

    /// Additional transport-specific options
    pub options: HandshakeOptions,
}

/// Options specific to each transport
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "options", rename_all = "snake_case")]
pub enum HandshakeOptions {
    /// Options specific to WebSocket connections
    WebSocket {
        // WebSocket connections currently have no additional options
    },

    /// Options specific to ZeroMQ connections
    ZeroMQ {
        /// The host string where the ZeroMQ client can be reached
        push_host: String,
    },
}

impl HandshakeRequest {
    /// Create a new [`HandshakeRequest`] for a WebSocket connection
    #[inline]
    #[must_use]
    pub fn new_websocket(server_auth: Option<String>) -> Self {
        let options = HandshakeOptions::WebSocket {};

        Self {
            server_auth,
            options,
        }
    }

    /// Create a new [`HandshakeRequest`] for a ZeroMQ connection
    #[inline]
    #[must_use]
    pub fn new_zeromq(server_auth: Option<String>, push_host: impl Into<String>) -> Self {
        let options = HandshakeOptions::ZeroMQ {
            push_host: push_host.into(),
        };

        Self {
            server_auth,
            options,
        }
    }
}
