use std::sync::Arc;

use clap::Parser;
use color_eyre::Result;
use tokio_postgres::NoTls;
use tracing::metadata::LevelFilter;
use tracing::{debug, error, info};

mod flatbuffers;
mod structures;
mod transport;

#[derive(Debug, Parser)]
struct Args {
    /// PostgreSQL Connection String
    #[clap(short, long = "psql", env = "WQL_POSTGRES_CONNECTION_STRING")]
    psql_conn: String,

    /// Enable Debug Logs
    #[clap(long, env = "WQL_DEBUG")]
    debug: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();

    // Always enable TRACE level logging on debug builds
    #[cfg(debug_assertions)]
    let filter = LevelFilter::TRACE;
    // Conditionally enable DEBUG/INFO logging on release builds
    #[cfg(not(debug_assertions))]
    let filter = match args.debug {
        true => LevelFilter::DEBUG,
        false => LevelFilter::INFO,
    };

    tracing_subscriber::fmt().with_max_level(filter).init();

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

    Ok(())
}
