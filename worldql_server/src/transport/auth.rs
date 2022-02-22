use worldql_messages::client_bound::Error;
use worldql_messages::server_bound::HandshakeRequest;

use crate::errors::{ERR_AUTH_FAILED_INCORRECT, ERR_AUTH_FAILED_NO_TOKEN};

#[must_use]
pub(super) fn authenticate_handshake(
    server_token: Option<String>,
    handshake: HandshakeRequest,
) -> Option<Error> {
    let HandshakeRequest { server_auth } = handshake;

    match (server_token, server_auth) {
        // Server auth disabled, ignore
        (None, _) => None,

        // Server auth enabled, client gave no token
        (Some(_), None) => Some(ERR_AUTH_FAILED_NO_TOKEN.clone()),

        // Server auth enabled, client gave a token
        (Some(server_token), Some(token)) => {
            // Check token matches
            if server_token == token {
                None
            } else {
                Some(ERR_AUTH_FAILED_INCORRECT.clone())
            }
        }
    }
}
