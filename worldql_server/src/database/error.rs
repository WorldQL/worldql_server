use worldql_messages::client_bound::Error;

use crate::errors::err_generic_database_error;

pub(super) trait IntoClientError {
    #[must_use]
    fn err(self) -> Error;
}

impl IntoClientError for sqlx::Error {
    #[inline]
    fn err(self) -> Error {
        let message = self.to_string();
        err_generic_database_error(message)
    }
}
