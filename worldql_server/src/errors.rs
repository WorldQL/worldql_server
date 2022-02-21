#![allow(clippy::unusual_byte_groupings)]

use once_cell::sync::Lazy;
use worldql_messages::client_bound::Error;

macro_rules! error {
    ($name: ident, $code:expr, $message:expr) => {
        pub static $name: Lazy<Error> = Lazy::new(|| Error::new($code, $message));
    };
}

macro_rules! error_func {
    ($name: ident, $code:expr) => {
        #[inline]
        #[must_use]
        pub fn $name(message: impl Into<String>) -> Error {
            Error::new($code, message)
        }
    };
}

// region: Generic Transport
#[rustfmt::skip]
error!(ERR_INVALID_MESSAGE, 0x00_001, "invalid message");
#[rustfmt::skip]
error!(ERR_HANDSHAKE_REQUIRED, 0x00_002, "first message must be a handshake");
#[rustfmt::skip]
error!(ERR_DUPLICATE_UUID, 0x00_003, "uuid already in use");
#[rustfmt::skip]
error!(ERR_AUTH_FAILED_NO_TOKEN, 0x00_004, "authentication failed, no auth token given");
#[rustfmt::skip]
error!(ERR_AUTH_FAILED_INCORRECT, 0x00_005, "authentication failed, incorrect token");
// endregion

// region: Generic Processing
error_func!(err_invalid_world_name, 0x01_001);
// endregion

// region: Subscription Management
#[rustfmt::skip]
error!(ERR_WORLD_SUB_GLOBAL_WORLD, 0x02_001, "cannot subscribe to the global world");
#[rustfmt::skip]
error!(ERR_WORLD_UNSUB_GLOBAL_WORLD, 0x02_002, "cannot unsubscribe from the global world");
#[rustfmt::skip]
error!(ERR_AREA_SUB_GLOBAL_WORLD, 0x02_003, "cannot subscribe to an area inside the global world");
#[rustfmt::skip]
error!(ERR_AREA_UNSUB_GLOBAL_WORLD, 0x02_004, "cannot unsubscribe from an area inside the global world");
// endregion

// region: Global and Local Messages
#[rustfmt::skip]
error!(ERR_LOCAL_MESSAGE_GLOBAL_WORLD, 0x02_001, "cannot send local messages to the global world");
// endregion
