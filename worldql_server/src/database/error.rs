use worldql_messages::client_bound::Error;

use crate::errors::err_generic_database_error;

pub(super) trait ResultExt<T> {
    fn or_client_err(self) -> Result<T, Error>;
}

impl<T> ResultExt<T> for Result<T, sqlx::Error> {
    #[inline]
    fn or_client_err(self) -> Result<T, Error> {
        self.map_err(|error| {
            let message = error.to_string();
            err_generic_database_error(message)
        })
    }
}
