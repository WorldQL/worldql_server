use std::net::{IpAddr, SocketAddr};

use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use color_eyre::Result;
use serde::Deserialize;
use tracing::info;
use uuid::Uuid;

use crate::structures::{Instruction, Message, Replication};

pub async fn start_http_server(host: IpAddr, port: u16) -> Result<()> {
    let addr = SocketAddr::new(host, port);
    info!("HTTP Server listening on {}", addr);

    let app = Router::new().route("/global_message", post(post_global_message));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

#[derive(Debug, Deserialize)]
struct PartialGlobalMessage {
    parameter: String,
    world_name: String,
}

impl From<PartialGlobalMessage> for Message {
    fn from(partial: PartialGlobalMessage) -> Self {
        Self {
            instruction: Instruction::GlobalMessage,
            parameter: Some(partial.parameter),
            sender_uuid: Uuid::nil(),
            world_name: partial.world_name,
            replication: Replication::ExceptSelf,
            records: vec![],
            entities: vec![],
            position: None,
            flex: None,
        }
    }
}

async fn post_global_message(
    Json(partial_message): Json<PartialGlobalMessage>,
) -> impl IntoResponse {
    // TODO
}
