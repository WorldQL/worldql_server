use std::net::IpAddr;
use std::num::ParseIntError;

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
    /// PostgreSQL connection string
    #[clap(short = 'p', long = "psql", env = "WQL_POSTGRES_CONNECTION_STRING")]
    pub psql_conn: String,

    /// Side length of subscription region cubes
    ///
    /// A value of 0 is invalid
    #[clap(long, default_value = "16", env = "WQL_SUBSCRIPTION_REGION_CUBE_SIZE", parse(try_from_str = parse_non_zero_16))]
    pub sub_region_size: u16,

    /// TODO: Add arg docs
    ///
    /// A value of 0 is invalid
    #[clap(long, default_value = "16", env = "WQL_DB_REGION_X_SIZE", parse(try_from_str = parse_non_zero_16))]
    pub db_region_x_size: u16,

    /// TODO: Add arg docs
    ///
    /// A value of 0 is invalid
    #[clap(long, default_value = "256", env = "WQL_DB_REGION_Y_SIZE", parse(try_from_str = parse_non_zero_16))]
    pub db_region_y_size: u16,

    /// TODO: Add arg docs
    ///
    /// A value of 0 is invalid
    #[clap(long, default_value = "16", env = "WQL_DB_REGION_Z_SIZE", parse(try_from_str = parse_non_zero_16))]
    pub db_region_z_size: u16,

    /// TODO: Add arg docs
    ///
    /// A value of 0 is invalid
    #[clap(long, default_value = "1024", env = "WQL_DB_TABLE_SIZE", parse(try_from_str = parse_non_zero_32))]
    pub db_table_size: u32,

    /// Maximum number of cached database lookups
    ///
    /// Set to 0 to disable cache eviction
    #[clap(long, default_value = "1024", env = "WQL_DB_CACHE_SIZE", parse(try_from_str = parse_non_zero_sized))]
    pub db_cache_size: usize,
    // endregion

    // region: WebSocket
    /// WebSocket Host
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

    // region: ZeroMQ
    /// ZeroMQ server port
    #[cfg(feature = "zeromq")]
    #[clap(short = 'z', long, default_value = "5555", env = "WQL_ZMQ_SERVER_PORT")]
    pub zmq_server_port: u16,

    /// ZeroMQ connection timeout (seconds)
    ///
    /// It is not recommended to set this to a very large number, values less than 10 are invalid
    #[cfg(feature = "zeromq")]
    #[clap(short = 'T', long, default_value = "25", env = "WQL_ZMQ_TIMEOUT_SECS", parse(try_from_str = parse_zmq_timeout_secs))]
    pub zmq_timeout_secs: u8,
    // endregion

    // region: Other Flags
    /// Set verbosity level
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

    #[error("must be greater than {0}")]
    GreaterThan(u8),

    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
}

fn parse_non_zero_16(src: &str) -> Result<u16, ParseError> {
    let size = src.parse::<u16>()?;
    if size == 0 {
        return Err(ParseError::NonZero);
    }

    Ok(size)
}

fn parse_non_zero_32(src: &str) -> Result<u32, ParseError> {
    let size = src.parse::<u32>()?;
    if size == 0 {
        return Err(ParseError::NonZero);
    }

    Ok(size)
}

fn parse_non_zero_sized(src: &str) -> Result<usize, ParseError> {
    let size = src.parse::<usize>()?;
    if size == 0 {
        return Err(ParseError::NonZero);
    }

    Ok(size)
}

#[cfg(feature = "zeromq")]
fn parse_zmq_timeout_secs(src: &str) -> Result<u8, ParseError> {
    let min = 10;

    let secs = src.parse::<u8>()?;
    if secs < min {
        return Err(ParseError::GreaterThan(min));
    }

    Ok(secs)
}
// endregion

// region: Whole Arg Validator
impl Args {
    /// Returns `true` if the args are valid
    pub fn validate(&self) -> bool {
        if self.sub_region_size < 10 {
            warn!("Subscription region sizes less than 10 might impact lookup performance")
        }

        // TODO: Better error messages
        let mod_x = self.db_table_size % u32::from(self.db_region_x_size);
        if mod_x != 0 {
            error!(
                "--db-table-size ({}) must be divisible by --db-region-x-size ({})",
                self.db_table_size, self.db_region_x_size
            );

            return false;
        }

        let mod_y = self.db_table_size % u32::from(self.db_region_y_size);
        if mod_y != 0 {
            error!(
                "--db-table-size ({}) must be divisible by --db-region-y-size ({})",
                self.db_table_size, self.db_region_y_size
            );

            return false;
        }

        let mod_z = self.db_table_size % u32::from(self.db_region_z_size);
        if mod_z != 0 {
            error!(
                "--db-table-size ({}) must be divisible by --db-region-z-size ({})",
                self.db_table_size, self.db_region_z_size
            );

            return false;
        }

        true
    }
}
// endregion
