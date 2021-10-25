/// What protocol the client is connected with.
/// ZeroMQ is used only when the WorldQL client is another game server.
/// For communicating directly with clients, WebSocket or UDP/TCP should be used.
/// Used with [ConnectedClient]
pub enum ClientSocketType {
    ZeroMQ,
    WebSocket
}

/// Represents a connected client. Stored in a hashmap relating UUID to ConnectedClient.
pub struct ConnectedClient {
    client_type: ClientSocketType,
    uuid: String,
    /// Only used for some client types like ZeroMQ.
    port: Option<i32>,
    /// In milliseconds.
    last_heartbeat: i128
}