use std::collections::HashSet;
use std::sync::Arc;

use clap::Parser;
use color_eyre::Result;
use tokio::sync::RwLock;
use tokio_postgres::NoTls;
use tracing::{debug, error, info};
use utils::PortRange;

use crate::processing::start_processing_thread;
use crate::transport::{start_websocket_server, start_zeromq_server, PeerMap, ThreadPeerMap};

mod flatbuffers;
mod processing;
mod structures;
mod transport;
mod utils;

// Fail to compile if no transport features are enabled
#[cfg(not(any(feature = "websocket", feature = "zeromq")))]
compile_error!("at least one of `websocket` or `zeromq` features must be enabled!");

#[derive(Debug, Parser)]
#[clap(version)]
struct Args {
    /// PostgreSQL Connection String
    #[clap(short, long = "psql", env = "WQL_POSTGRES_CONNECTION_STRING")]
    psql_conn: String,

    /// Enable Debug Logs
    #[cfg(not(debug_assertions))]
    #[clap(long, env = "WQL_DEBUG")]
    debug: bool,

    /// Enable Verbose Logging
    #[cfg(debug_assertions)]
    #[clap(short, long, parse(from_occurrences))]
    verbose: u8,

    /// WebSocket Server Port
    #[cfg(feature = "websocket")]
    #[clap(short, long, default_value = "8080", env = "WQL_WEBSOCKET_PORT")]
    ws_port: u16,

    /// ZeroMQ Server Port
    #[cfg(feature = "zeromq")]
    #[clap(
        name = "PORT",
        short = 'z',
        long = "zmq-server-port",
        default_value = "5555",
        env = "WQL_ZMQ_SERVER_PORT"
    )]
    zmq_server_port: u16,

    /// ZeroMQ Client Port Range
    #[cfg(feature = "zeromq")]
    #[clap(
        name = "PORT_RANGE",
        short = 'Z',
        long = "zmq-client-ports",
        default_value = "22000..23000",
        env = "WQL_ZMQ_CLIENT_PORTS"
    )]
    zmq_client_ports: PortRange,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();

    // Logger on debug builds
    #[cfg(debug_assertions)]
    let logger = {
        let filter = match args.verbose {
            0 => format!("{}=debug", env!("CARGO_PKG_NAME")),
            1 | 2 => format!("{}=trace", env!("CARGO_PKG_NAME")),
            _ => "trace".into(),
        };

        tracing_subscriber::fmt()
            .with_target(args.verbose >= 2)
            .with_env_filter(filter)
    };

    // Logger on release builds
    #[cfg(not(debug_assertions))]
    let logger = {
        let level = match args.debug {
            true => "debug",
            false => "info",
        };

        let filter = format!("{}={}", env!("CARGO_PKG_NAME"), level);
        tracing_subscriber::fmt()
            .with_target(false)
            .with_env_filter(filter)
    };

    // Init logger for all builds
    logger.init();

    {
        // Check for port clashes
        let mut used_ports = vec![];

        #[cfg(feature = "websocket")]
        used_ports.push(args.ws_port);

        #[cfg(feature = "zeromq")]
        {
            used_ports.push(args.zmq_server_port);
            for client_port in args.zmq_client_ports.inner() {
                used_ports.push(client_port);
            }
        }

        let mut uniq = HashSet::new();
        let unique = used_ports.into_iter().all(move |x| uniq.insert(x));

        if !unique {
            // TODO: Work out which port(s) clash
            error!("configured ports must be unique");
            std::process::exit(1);
        }
    }

    let (psql, psql_conn) = tokio_postgres::connect(&args.psql_conn, NoTls).await?;
    tokio::spawn(async move {
        debug!("spawned postgres read thread");
        if let Err(e) = psql_conn.await {
            error!("PostgreSQL Connection Error: {}", e);
        }
    });

    let client = Arc::new(psql);
    info!("Connected to PostgreSQL");

    let peer_map: ThreadPeerMap = Arc::new(RwLock::new(PeerMap::new()));
    let (msg_tx, msg_rx) = tokio::sync::mpsc::unbounded_channel();

    let ws_handle = tokio::spawn(start_websocket_server(
        peer_map.clone(),
        msg_tx.clone(),
        args.ws_port,
    ));

    let zmq_handle = tokio::spawn(start_zeromq_server(
        peer_map.clone(),
        msg_tx,
        args.zmq_server_port,
        args.zmq_client_ports.into(),
    ));

    let proc_handle = tokio::spawn(start_processing_thread(peer_map, msg_rx));
    let _ = futures_util::join!(ws_handle, zmq_handle, proc_handle);

    Ok(())
}
