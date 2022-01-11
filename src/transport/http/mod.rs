#[cfg(feature = "http")]
mod http_rest;
#[cfg(feature = "websocket")]
mod websocket;

#[cfg(feature = "http")]
pub use http_rest::start_http_server;
#[cfg(feature = "websocket")]
pub use websocket::start_websocket_server;
