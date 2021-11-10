use std::collections::HashSet;
use std::sync::Arc;

use clap::Parser;
use color_eyre::Result;
use tokio::sync::RwLock;
use tokio_postgres::NoTls;
use tracing::{debug, error, info};
#[cfg(feature = "zeromq")]
use utils::PortRange;
use crate::outgoing_zeromq_owner::start_outgoing_zeromq_thread;

use crate::processing::start_processing_thread;
#[cfg(feature = "websocket")]
use crate::transport::start_websocket_server;
#[cfg(feature = "zeromq")]
use crate::transport::start_zeromq_server;
use crate::transport::{PeerMap, ThreadPeerMap};

mod flatbuffers;
mod processing;
mod structures;
mod subscriptions;
mod transport;
mod utils;
mod outgoing_zeromq_owner;

// Fail to compile if no transport features are enabled
#[cfg(not(any(feature = "websocket", feature = "zeromq")))]
compile_error!("at least one of `websocket` or `zeromq` features must be enabled!");

#[derive(Debug, Parser)]
#[clap(version)]
struct Args {
    /// PostgreSQL Connection String
    #[clap(short, long = "psql", env = "WQL_POSTGRES_CONNECTION_STRING")]
    psql_conn: String,

    /// Set Verbosity Level.
    /// eg: -vvv to enable very verbose logs
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

    /// Subscription Region Cube Size
    #[clap(long, default_value = "10", env = "WQL_SUBSCRIPTION_REGION_CUBE_SIZE")]
    sub_region_cube_size: u16,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();

    let filter = match args.verbose {
        #[cfg(debug_assertions)]
        0 | 1 => format!("{}=debug", env!("CARGO_PKG_NAME")),
        #[cfg(not(debug_assertions))]
        0 => format!("{}=info", env!("CARGO_PKG_NAME")),
        #[cfg(not(debug_assertions))]
        1 => format!("{}=debug", env!("CARGO_PKG_NAME")),
        2 | 3 => format!("{}=trace", env!("CARGO_PKG_NAME")),
        _ => "trace".into(),
    };

    tracing_subscriber::fmt()
        .with_target(args.verbose >= 3)
        .with_env_filter(filter)
        .init();

    // Check for port clashes
    {
        let mut used_ports = HashSet::new();

        #[cfg(feature = "websocket")]
        {
            if !portpicker::is_free_tcp(args.ws_port) {
                error!("WebSocket Server port {} is already in use!", args.ws_port);
                std::process::exit(1);
            }

            used_ports.insert(args.ws_port);
        }

        #[cfg(feature = "zeromq")]
        {
            let server_inserted = used_ports.insert(args.zmq_server_port);
            if !server_inserted || !portpicker::is_free_tcp(args.zmq_server_port) {
                error!(
                    "ZeroMQ Server port {} is already in use!",
                    args.zmq_server_port
                );

                std::process::exit(1);
            }
        }
    }

    let psql_result = tokio_postgres::connect(&args.psql_conn, NoTls).await;
    if let Err(err) = psql_result {
        error!("PostgreSQL Error: {}", err);
        std::process::exit(1);
    }

    let (psql, psql_conn) = psql_result.unwrap();
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

    // This exists because we cannot share the ZeroMQ PUSH sockets across threads.
    // We transfer ownership of the ZeroMQ threads to a dedicated thread that only sends outgoing messages.
    let (zeromq_outgoing_tx, zeromq_outgoing_rx) = tokio::sync::mpsc::unbounded_channel();

    let mut handles = vec![];

    #[cfg(feature = "websocket")]
    {
        let ws_handle = tokio::spawn(start_websocket_server(
            peer_map.clone(),
            msg_tx.clone(),
            args.ws_port,
        ));

        handles.push(ws_handle);
    }

    let ctx = tmq::Context::new();

    #[cfg(feature = "zeromq")]
    {
        let zmq_handle = tokio::spawn(start_zeromq_server(
            peer_map.clone(),
            msg_tx,
            args.zmq_server_port,
            zeromq_outgoing_tx,
            ctx.clone()
        ));

        handles.push(zmq_handle);
    }

    let proc_handle = tokio::spawn(start_processing_thread(peer_map, msg_rx));
    handles.push(proc_handle);

    handles.push(tokio::spawn(start_outgoing_zeromq_thread(zeromq_outgoing_rx, ctx)));

    // Run all threads
    let _ = futures_util::future::join_all(handles).await;

    Ok(())
}
