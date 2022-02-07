use serde::{Deserialize, Serialize};

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

/// Used to indicate an error occurred when processing the request
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Error {
    /// Error Code
    pub code: u32,

    /// Error Message
    pub message: String,
}

impl Error {
    /// Create a new [`Error`]
    ///
    /// # Examples
    /// ```
    /// pub use worldql_messages::outgoing::Error;
    ///
    /// let error = Error::new(0xAB, "error");
    ///
    /// assert_eq!(error.code, 0xAB);
    /// assert_eq!(error.message, "error");
    /// ```
    #[inline]
    #[must_use]
    pub fn new(code: u32, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}
