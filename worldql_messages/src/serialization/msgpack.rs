use bytes::Bytes;
pub use rmp_serde::decode::Error as DecodeError;
pub use rmp_serde::encode::Error as EncodeError;

use crate::incoming::IncomingMessage;
use crate::outgoing::OutgoingMessage;

/// Serialize and Deserialize to/from MessagePack bytes
pub trait SerializeBinary {
    /// Serialize into MessagePack encoded bytes
    fn serialize_binary(&self) -> Result<Bytes, EncodeError>;

    /// Deserialize from MessagePack encoded bytes
    fn deserialize_binary(bytes: Bytes) -> Result<Self, DecodeError>
    where
        Self: Sized;
}

macro_rules! impl_msgpack {
    ($target:ty) => {
        impl SerializeBinary for $target {
            #[inline]
            fn serialize_binary(&self) -> Result<Bytes, EncodeError> {
                let vec = rmp_serde::to_vec_named(self)?;
                Ok(bytes::Bytes::from(vec))
            }

            #[inline]
            fn deserialize_binary(bytes: Bytes) -> Result<Self, DecodeError> {
                let message = rmp_serde::from_slice(&bytes)?;
                Ok(message)
            }
        }
    };
}

impl_msgpack!(IncomingMessage);
impl_msgpack!(OutgoingMessage);
