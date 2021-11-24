#![warn(clippy::cast_lossless, clippy::implicit_clone, clippy::unused_async)]

use std::collections::HashSet;
use std::sync::Arc;

use clap::Parser;
use color_eyre::Result;
use tokio::sync::RwLock;
use tokio_postgres::NoTls;
use tracing::{debug, error, info, warn};

use crate::args::Args;
use crate::database::DatabaseClient;
use crate::processing::start_processing_thread;
#[cfg(feature = "websocket")]
use crate::transport::start_websocket_server;
#[cfg(feature = "zeromq")]
use crate::transport::{start_zeromq_incoming, start_zeromq_outgoing};
use crate::transport::{PeerMap, ThreadPeerMap};

mod args;
mod database;
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

    // Validate args
    let valid = args.validate();
    if !valid {
        std::process::exit(1);
    }

    let psql_result = tokio_postgres::connect(&args.psql_conn, NoTls).await;
    if let Err(err) = psql_result {
        error!("PostgreSQL Error: {}", err);
        std::process::exit(1);
    }

    let (client, psql_conn) = psql_result.unwrap();
    tokio::spawn(async move {
        debug!("spawned postgres read thread");
        if let Err(e) = psql_conn.await {
            error!("PostgreSQL Connection Error: {}", e);
        }
    });

    info!("Connected to PostgreSQL");
    let mut client = DatabaseClient::new(
        client,
        args.db_region_x_size,
        args.db_region_y_size,
        args.db_region_z_size,
        args.db_table_size,
        args.db_cache_size,
    );

    // Init database
    if let Err(error) = client.init_database().await {
        error!("Failed to create database tables!");
        error!("{}", error);

        std::process::exit(1);
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
        client,
        peer_map,
        msg_rx,
        remove_rx,
        args.sub_region_size,
    ));

    handles.push(proc_handle);

    // Run all threads
    let _ = futures_util::future::join_all(handles).await;

    Ok(())
}
