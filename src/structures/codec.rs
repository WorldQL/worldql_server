use thiserror::Error;

pub trait Encode<T> {
    fn encode(self) -> T;
}

pub trait Decode<T> {
    fn decode(encoded: T) -> Result<Self, DecodeError>
    where
        Self: Sized;
}

#[derive(Debug, Error)]
pub enum DecodeError {
    #[error("missing required field: {0}")]
    MissingRequiredField(String),
}
