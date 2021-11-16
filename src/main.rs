#![warn(clippy::cast_lossless, clippy::implicit_clone, clippy::unused_async)]

use std::collections::HashSet;
use std::sync::Arc;

use clap::Parser;
use color_eyre::Result;
use tokio::sync::RwLock;
use tokio_postgres::NoTls;
use tracing::{debug, error, info};

use crate::processing::start_processing_thread;
#[cfg(feature = "websocket")]
use crate::transport::start_websocket_server;
#[cfg(feature = "zeromq")]
use crate::transport::{start_zeromq_incoming, start_zeromq_outgoing};
use crate::transport::{PeerMap, ThreadPeerMap};

mod flatbuffers;
mod processing;
mod structures;
mod subscriptions;
mod transport;
mod utils;

// Fail to compile ZeroMQ module on non unix-based systems
#[cfg(all(feature = "zeromq", not(unix)))]
compile_error!("the `zeromq` feature is only supported on unix-based systems");

// Fail to compile if no transport features are enabled
#[cfg(not(any(feature = "websocket", feature = "zeromq")))]
compile_error!("at least one of `websocket` or `zeromq` features must be enabled!");

#[derive(Debug, Parser)]
#[clap(version, global_setting = clap::AppSettings::DeriveDisplayOrder)]
struct Args {
    // region: Global Flags
    /// PostgreSQL connection string
    #[clap(short = 'p', long = "psql", env = "WQL_POSTGRES_CONNECTION_STRING")]
    // TODO: Make required
    psql_conn: Option<String>,

    /// Side length of region cubes
    ///
    /// A value of 0 is invalid
    #[clap(
        short = 's',
        long,
        default_value = "16",
        env = "WQL_SUBSCRIPTION_REGION_CUBE_SIZE"
    )]
    region_size: u16,
    // endregion

    // region: WebSocket
    /// WebSocket server port
    #[cfg(feature = "websocket")]
    #[clap(short = 'w', long, default_value = "8080", env = "WQL_WEBSOCKET_PORT")]
    ws_port: u16,
    // endregion

    // region: ZeroMQ
    /// ZeroMQ server port
    #[cfg(feature = "zeromq")]
    #[clap(short = 'z', long, default_value = "5555", env = "WQL_ZMQ_SERVER_PORT")]
    zmq_server_port: u16,

    /// ZeroMQ connection timeout (seconds)
    ///
    /// It is not recommended to set this to a very large number, values less than 10 are invalid
    #[cfg(feature = "zeromq")]
    #[clap(short = 'T', long, default_value = "10", env = "WQL_ZMQ_TIMEOUT_SECS")]
    zmq_timeout_secs: u8,
    // endregion

    // region: Other Flags
    /// Set verbosity level
    ///
    /// eg: -vvv for very verbose logs
    #[clap(short, long, parse(from_occurrences))]
    verbose: u8,
    // endregion
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();

    let filter = match args.verbose {
        #[cfg(debug_assertions)]
        0 | 1 | 2 => format!("{}=debug", env!("CARGO_PKG_NAME")),

        #[cfg(not(debug_assertions))]
        0 => format!("{}=info", env!("CARGO_PKG_NAME")),
        #[cfg(not(debug_assertions))]
        1 | 2 => format!("{}=debug", env!("CARGO_PKG_NAME")),

        3 => format!("{}=trace", env!("CARGO_PKG_NAME")),
        _ => "trace".into(),
    };

    tracing_subscriber::fmt()
        .with_target(args.verbose >= 2)
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

    // Validate ZeroMQ Timeout Arg
    #[cfg(feature = "zeromq")]
    if args.zmq_timeout_secs < 10 {
        error!("A ZeroMQ timeout of less than 10 seconds is invalid!");
        std::process::exit(1);
    }

    // TODO: Make required
    let client = match args.psql_conn {
        None => None,
        Some(conn) => {
            let psql_result = tokio_postgres::connect(&conn, NoTls).await;
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

            Some(client)
        }
    };

    let (msg_tx, msg_rx) = flume::unbounded();
    let (remove_tx, remove_rx) = flume::unbounded();

    let peer_map: ThreadPeerMap = Arc::new(RwLock::new(PeerMap::new(remove_tx)));
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

    #[cfg(feature = "zeromq")]
    {
        let ctx = tmq::Context::new();
        let (zmq_msg_tx, zmq_msg_rx) = flume::unbounded();
        let (zmq_handshake_tx, zmq_handshake_rx) = flume::unbounded();

        let zmq_incoming_handle = tokio::spawn(start_zeromq_incoming(
            peer_map.clone(),
            msg_tx,
            zmq_handshake_tx,
            args.zmq_server_port,
            ctx.clone(),
        ));

        let zmq_outgoing_handle = tokio::spawn(start_zeromq_outgoing(
            peer_map.clone(),
            zmq_msg_tx,
            zmq_msg_rx,
            zmq_handshake_rx,
            ctx,
            args.zmq_timeout_secs,
        ));

        handles.push(zmq_incoming_handle);
        handles.push(zmq_outgoing_handle);
    }

    let proc_handle = tokio::spawn(start_processing_thread(
        peer_map,
        msg_rx,
        remove_rx,
        args.region_size,
    ));

    handles.push(proc_handle);

    // Run all threads
    let _ = futures_util::future::join_all(handles).await;

    Ok(())
}
