use std::env;
use std::sync::{Arc, Mutex, RwLock};
use tmq::{Context};
use tokio_postgres::NoTls;
use log::{info, trace, warn, error};

mod zmq_main_server;
mod connected_client;
mod worldql_instruction;

#[tokio::main]
async fn main() {
    if let Err(_) = env::var("RUST_LOG") {
        env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();

    let mut handles = vec![];
    let zmq_context = Context::new();

    //let push_sockets = Arc::new(Mutex::new(vec![]));


    let sql_connection_string = match env::var_os("WQL_POSTGRES_CONNECTION_STRING") {
        Some(v) => v.into_string().unwrap(),
        None => panic!("$WQL_POSTGRES_CONNECTION_STRING is not set")
    };
    info!("Connecting to PostgreSQL with connection string {}", sql_connection_string);
    let (client, connection) =
        tokio_postgres::connect(&*sql_connection_string, NoTls).await.unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            error!("connection error: {}", e);
        }
    });

    handles.push(zmq_main_server::start_main_server(zmq_context.clone()));

    futures::future::join_all(handles).await;
}