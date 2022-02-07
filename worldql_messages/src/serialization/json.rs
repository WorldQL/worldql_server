pub use serde_json::Error;

use crate::incoming::IncomingMessage;
use crate::outgoing::OutgoingMessage;

/// Serialize and Deserialize to/from a JSON string
pub trait SerializeJson {
    /// Serialize to a minified JSON string
    fn serialize_json(&self) -> Result<String, Error>;

    /// Serialize to a pretty-printed JSON string
    fn serialize_json_pretty(&self) -> Result<String, Error>;

    /// Deserialize from a JSON string
    fn deserialize_json(string: &str) -> Result<Self, Error>
    where
        Self: Sized;
}

macro_rules! impl_json {
    ($target:ty) => {
        impl SerializeJson for $target {
            #[inline]
            fn serialize_json(&self) -> Result<String, Error> {
                let string = serde_json::to_string(self)?;
                Ok(string)
            }

            #[inline]
            fn serialize_json_pretty(&self) -> Result<String, Error> {
                let string = serde_json::to_string_pretty(self)?;
                Ok(string)
            }

            #[inline]
            fn deserialize_json(string: &str) -> Result<Self, Error> {
                let message = serde_json::from_str(string)?;
                Ok(message)
            }
        }
    };
}

impl_json!(IncomingMessage);
impl_json!(OutgoingMessage);
