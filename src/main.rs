use std::sync::Arc;

use clap::Parser;
use color_eyre::Result;
use tokio_postgres::NoTls;
use tracing::{debug, error, info};

use crate::transport::websocket::start_websocket_server;

mod flatbuffers;
mod structures;
mod transport;

// Fail to compile if no transport features are enabled
#[cfg(not(any(feature = "websocket", feature = "zeromq")))]
compile_error!("at least one of `websocket` or `zeromq` features must be enabled!");

#[derive(Debug, Parser)]
struct Args {
    /// PostgreSQL Connection String
    #[clap(short, long = "psql", env = "WQL_POSTGRES_CONNECTION_STRING")]
    psql_conn: String,

    /// Enable Debug Logs
    #[clap(long, env = "WQL_DEBUG")]
    debug: bool,

    /// Enable Verbose Logging
    #[cfg(debug_assertions)]
    #[clap(long)]
    verbose: bool,

    /// WebSocket Server Port
    #[cfg(feature = "websocket")]
    #[clap(short, long, default_value = "8080", env = "WQL_WEBSOCKET_PORT")]
    ws_port: u16,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();

    // Logger on debug builds
    #[cfg(debug_assertions)]
    let logger = {
        let filter = match args.verbose {
            true => "trace".into(),
            false => format!("{}=trace", env!("CARGO_PKG_NAME")),
        };

        tracing_subscriber::fmt().with_target(args.verbose).with_env_filter(filter)
    };

    // Logger on release builds
    #[cfg(not(debug_assertions))]
    let logger = {
        let level = match args.debug {
            true => "debug",
            false => "info",
        };

        let filter = format!("{}={}", env!("CARGO_PKG_NAME"), level);
        tracing_subscriber::fmt().with_target(false).with_env_filter(filter)
    };

    // Init logger for all builds
    logger.init();

    info!(
        "Connecting to PostgreSQL with connection string: {}",
        &args.psql_conn
    );

    let (psql, psql_conn) = tokio_postgres::connect(&args.psql_conn, NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = psql_conn.await {
            error!("postgres connection error: {}", e);
        }
    });

    let client = Arc::new(psql);
    debug!("connected to postgres");

    let ws_handle = tokio::spawn(start_websocket_server(args.ws_port));

    let _ = futures_util::join!(ws_handle);

    Ok(())
}
