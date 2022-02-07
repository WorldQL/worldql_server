use once_cell::sync::Lazy;
use worldql_messages::outgoing::Error;

macro_rules! error {
    ($name: ident, $code:expr, $message:expr) => {
        pub static $name: Lazy<Error> = Lazy::new(|| Error::new($code, $message));
    };
}

error!(ERR_INVALID_MESSAGE, 0x00001, "invalid message");
#[rustfmt::skip]
error!(ERR_HANDSHAKE_REQUIRED, 0x00002, "first message must be a handshake");
error!(ERR_DUPLICATE_UUID, 0x00003, "uuid already in use");
