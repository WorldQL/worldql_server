//! Traits used to (de)serialize message structs
//!
//! # Binary
//! Enabling the `msgpack` feature exposes the [`SerializeBinary`] trait, allowing you
//! to (de)serialize messages to and from a byte array.
//!
//! # JSON
//! Enabling the `json` feature exposes the [`SerializeJson`] trait, allowing you to
//! (de)serialize messages to and from a JSON string, with optional pretty printing.

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "msgpack")]
mod msgpack;

#[cfg(feature = "json")]
pub use json::*;
#[cfg(feature = "msgpack")]
pub use msgpack::*;
