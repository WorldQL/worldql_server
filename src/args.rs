use clap::{AppSettings, Parser};

#[derive(Debug, Parser)]
#[clap(version, global_setting = AppSettings::DeriveDisplayOrder)]
pub struct Args {
    // region: Global Flags
    /// PostgreSQL connection string
    #[clap(short = 'p', long = "psql", env = "WQL_POSTGRES_CONNECTION_STRING")]
    pub psql_conn: String,

    /// Side length of region cubes
    ///
    /// A value of 0 is invalid
    #[clap(long, default_value = "16", env = "WQL_SUBSCRIPTION_REGION_CUBE_SIZE")]
    pub sub_region_size: u16,

    // TODO: Add arg docs
    #[clap(long, default_value = "16", env = "WQL_DB_REGION_X_SIZE")]
    pub db_region_x_size: u16,

    // TODO: Add arg docs
    #[clap(long, default_value = "256", env = "WQL_DB_REGION_Y_SIZE")]
    pub db_region_y_size: u16,

    // TODO: Add arg docs
    #[clap(long, default_value = "16", env = "WQL_DB_REGION_Z_SIZE")]
    pub db_region_z_size: u16,

    // TODO: Add arg docs
    #[clap(long, default_value = "1024", env = "WQL_DB_TABLE_SIZE")]
    pub db_table_size: u32,
    // endregion

    // region: WebSocket
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
    #[clap(short = 'T', long, default_value = "10", env = "WQL_ZMQ_TIMEOUT_SECS")]
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

// TODO
// // Validate region size arg
// if args.sub_region_size == 0 {
//     error!("A region size of 0 is invalid!");
//     std::process::exit(1);
// } else if args.sub_region_size < 10 {
//     warn!("Region sizes less than 10 might impact lookup performance")
// }

// // TODO: Validate db size args

// // Validate ZeroMQ Timeout arg
// #[cfg(feature = "zeromq")]
// if args.zmq_timeout_secs < 10 {
//     error!("A ZeroMQ timeout of less than 10 seconds is invalid!");
//     std::process::exit(1);
// }
