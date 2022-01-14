use thiserror::Error;

pub(super) trait Encode<T> {
    fn encode(self) -> T;
}

pub(super) trait Decode<T> {
    fn decode(encoded: T) -> Result<Self, DecodeError>
    where
        Self: Sized;
}

#[derive(Debug, Error)]
pub enum DecodeError {
    #[error("missing required field: {0}")]
    MissingRequiredField(String),

    #[error(transparent)]
    InvalidUuid(#[from] uuid::Error),
}
