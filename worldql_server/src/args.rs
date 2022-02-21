use std::net::IpAddr;
use std::num::ParseIntError;
use std::time::Duration;

use clap::{AppSettings, Parser};
use once_cell::sync::Lazy;
use thiserror::Error;
use tracing::{error, warn};

static VERSION: Lazy<String> = Lazy::new(|| {
    let mut version = format!("v{}", env!("CARGO_PKG_VERSION"));
    if let Some(hash) = option_env!("GIT_SHORT_HASH") {
        version += &format!(" ({})", hash);
    }

    version
});

// region: Args Struct
#[derive(Debug, Parser)]
#[clap(version = &VERSION[..], global_setting = AppSettings::DeriveDisplayOrder)]
pub struct Args {
    // region: Global Flags
    /// Server Authentication Token
    ///
    /// If set, clients will be required to pass this token in handshakes to authenticate
    #[clap(short = 'a', long, env = "WQL_SERVER_AUTH_TOKEN")]
    pub auth_token: Option<String>,

    /// PostgreSQL connection string
    #[clap(short = 'p', long = "psql", env = "WQL_POSTGRES_CONNECTION_STRING")]
    pub psql_conn: String,

    /// PostgreSQL maxiumum connections in pool
    ///
    /// A value of 0 is invalid
    #[clap(long, env = "WQL_POSTGRES_MAX_CONNECTIONS", default_value = "5", parse(try_from_str = parse_non_zero_8))]
    pub psql_max_connections: u8,

    /// PostgreSQL connection timeout (seconds)
    ///
    /// A value of 0 is invalid
    #[clap(long, env = "WQL_POSTGRES_TIMEOUT_SECS", default_value = "5", parse(try_from_str = parse_non_zero_16))]
    pub psql_timeout_secs: u16,

    /// Side length of subscription region cubes
    ///
    /// A value of 0 is invalid
    #[clap(long, default_value = "16", env = "WQL_SUBSCRIPTION_REGION_CUBE_SIZE", parse(try_from_str = parse_non_zero_16))]
    pub sub_region_size: u16,
    // endregion

    // // region: HTTP
    // /// HTTP server host
    // #[cfg(feature = "http")]
    // #[clap(short = 'H', long, default_value = "0.0.0.0", env = "WQL_HTTP_HOST")]
    // pub http_host: IpAddr,

    // /// HTTP server port
    // #[cfg(feature = "http")]
    // #[clap(short = 'h', long, default_value = "8081", env = "WQL_HTTP_PORT")]
    // pub http_port: u16,

    // /// HTTP server port
    // #[cfg(feature = "http")]
    // #[clap(long, env = "WQL_HTTP_AUTH_TOKEN")]
    // pub http_auth_token: Option<String>,
    // // endregion

    // region: WebSocket
    /// WebSocket server host
    #[cfg(feature = "websocket")]
    #[clap(
        short = 'W',
        long,
        default_value = "0.0.0.0",
        env = "WQL_WEBSOCKET_HOST"
    )]
    pub ws_host: IpAddr,

    /// WebSocket server port
    #[cfg(feature = "websocket")]
    #[clap(short = 'w', long, default_value = "8080", env = "WQL_WEBSOCKET_PORT")]
    pub ws_port: u16,
    // endregion

    // region: Other Flags
    /// Verbosity level
    ///
    /// eg: -vvv for very verbose logs
    #[clap(short, long, parse(from_occurrences))]
    pub verbose: u8,
    // endregion
}
// endregion

// region: Flag parsers
#[derive(Debug, Error)]
enum ParseError {
    #[error("must be greater than 0")]
    NonZero,

    // #[error("must be greater than {0}")]
    // GreaterThan(u8),
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
}

fn parse_non_zero_8(src: &str) -> Result<u8, ParseError> {
    let size = src.parse::<u8>()?;
    if size == 0 {
        return Err(ParseError::NonZero);
    }

    Ok(size)
}

fn parse_non_zero_16(src: &str) -> Result<u16, ParseError> {
    let size = src.parse::<u16>()?;
    if size == 0 {
        return Err(ParseError::NonZero);
    }

    Ok(size)
}

// fn parse_non_zero_32(src: &str) -> Result<u32, ParseError> {
//     let size = src.parse::<u32>()?;
//     if size == 0 {
//         return Err(ParseError::NonZero);
//     }

//     Ok(size)
// }

// fn parse_non_zero_sized(src: &str) -> Result<usize, ParseError> {
//     let size = src.parse::<usize>()?;
//     if size == 0 {
//         return Err(ParseError::NonZero);
//     }

//     Ok(size)
// }
// endregion

// region: Whole Arg Validator
impl Args {
    /// Returns `true` if the args are valid
    pub fn validate(&self) -> bool {
        if self.sub_region_size < 10 {
            warn!("Subscription region sizes less than 10 might impact lookup performance")
        }

        true
    }
}
// endregion
