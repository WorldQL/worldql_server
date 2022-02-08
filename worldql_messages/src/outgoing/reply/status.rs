use serde::{Deserialize, Serialize};

use crate::outgoing::Error;

/// Status for an message reply
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Status<T> {
    /// Wraps a struct containing the reply data
    Ok(T),

    /// Wraps the [`Error`] struct to indicate an error occurred
    Error(Error),
}

impl<T> Status<T> {
    /// Returns `true` if the result is [`Ok`](Status::Ok)
    #[must_use]
    #[inline]
    pub const fn is_ok(&self) -> bool {
        matches!(*self, Self::Ok(_))
    }

    /// Returns `true` if the result is [`Error`](Status::Error).
    #[must_use]
    #[inline]
    pub const fn is_error(&self) -> bool {
        !self.is_ok()
    }
}
