use clap::Parser;
use tracing::metadata::LevelFilter;

mod flatbuffers;
mod structures;

#[derive(Debug, Parser)]
struct Args {
    /// PostgreSQL Connection String
    #[clap(short, long = "psql", env = "WQL_POSTGRES_CONNECTION_STRING")]
    postgres_connection_string: String,

    /// Enable Debug Logs
    #[clap(long, env = "WQL_DEBUG")]
    debug: bool,
}

#[tokio::main]
async fn main() {
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
}
