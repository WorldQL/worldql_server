use serde::{Deserialize, Serialize};

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