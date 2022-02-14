#![forbid(unsafe_code)]
#![deny(private_in_public)]
#![warn(
    clippy::all,
    clippy::dbg_macro,
    clippy::todo,
    clippy::empty_enum,
    clippy::enum_glob_use,
    clippy::unused_self,
    clippy::needless_continue,
    clippy::needless_borrow,
    clippy::match_wildcard_for_single_variants,
    clippy::if_let_mutex,
    clippy::mismatched_target_os,
    clippy::match_on_vec_items,
    clippy::imprecise_flops,
    clippy::suboptimal_flops,
    clippy::lossy_float_literal,
    clippy::fn_params_excessive_bools,
    clippy::inefficient_to_string,
    clippy::macro_use_imports,
    clippy::option_option,
    clippy::unnested_or_patterns,
    clippy::str_to_string,
    clippy::cast_lossless,
    clippy::implicit_clone,
    clippy::unused_async,
    clippy::redundant_closure_for_method_calls,
    rust_2018_idioms,
    future_incompatible,
    nonstandard_style,
    missing_debug_implementations
)]

use std::collections::HashSet;
use std::sync::Arc;

use clap::Parser;
use color_eyre::Result;
use dotenv::dotenv;
use tokio::sync::RwLock;
use tokio_postgres::NoTls;
use tracing::{debug, error, info, warn};

use crate::args::Args;
use crate::database::DatabaseClient;
use crate::processing::start_processing_thread;
#[cfg(feature = "websocket")]
use crate::transport::websocket::start_websocket_server;
use crate::transport::{PeerMap, ThreadPeerMap};

mod args;
mod database;
mod errors;
mod processing;
mod transport;
mod utils;

// Fail to compile if no full transport features are enabled
#[cfg(not(any(feature = "websocket")))]
compile_error!("at least one of `websocket` features must be enabled!");

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    dotenv().ok();
    let args = Args::parse();

    let filter = match args.verbose {
        #[cfg(debug_assertions)]
        0 | 1 | 2 => format!("{}=debug", env!("TRACING_PREFIX")),

        #[cfg(not(debug_assertions))]
        0 => format!("{}=info", env!("TRACING_PREFIX")),
        #[cfg(not(debug_assertions))]
        1 | 2 => format!("{}=debug", env!("TRACING_PREFIX")),

        3 => format!("{}=trace", env!("TRACING_PREFIX")),
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
    }

    // Validate args
    let valid = args.validate();
    if !valid {
        std::process::exit(1);
    }

    if let Some(token) = &args.auth_token {
        if token.is_empty() {
            error!("Server Auth Token cannot be an empty string!");
            std::process::exit(1);
        }

        let obscured = "*".repeat(token.len());
        info!("Server Auth Token set to {}", obscured);

        if token.len() < 10 {
            warn!("Auth Token is less than 10 characters, consider using a more secure token")
        }
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
    let db = DatabaseClient::new(
        client,
        args.db_region_x_size,
        args.db_region_y_size,
        args.db_region_z_size,
        args.db_table_size,
        args.db_cache_size,
    );

    // Init database
    if let Err(error) = db.init_database().await {
        error!("Failed to create database tables!");
        error!("{}", error);

        std::process::exit(1);
    };

    let (msg_tx, msg_rx) = flume::unbounded();
    let (remove_tx, remove_rx) = flume::unbounded();

    let peer_map: ThreadPeerMap = Arc::new(RwLock::new(PeerMap::new(remove_tx)));
    let mut handles = vec![];

    #[cfg(feature = "http")]
    {
        let http_handle = tokio::spawn(start_http_server(
            msg_tx.clone(),
            args.http_host,
            args.http_port,
            args.http_auth_token,
        ));

        handles.push(http_handle);
    }

    #[cfg(feature = "websocket")]
    {
        let ws_handle = tokio::spawn(start_websocket_server(
            peer_map.clone(),
            msg_tx.clone(),
            args.auth_token.clone(),
            args.ws_host,
            args.ws_port,
        ));

        handles.push(ws_handle);
    }

    let proc_handle = tokio::spawn(start_processing_thread(
        peer_map,
        db,
        msg_rx,
        remove_rx,
        args.sub_region_size,
    ));

    handles.push(proc_handle);

    // Run all threads
    let _ = futures_util::future::join_all(handles).await;

    Ok(())
}
