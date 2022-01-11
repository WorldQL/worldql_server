use std::net::{IpAddr, SocketAddr};

use axum::extract::Extension;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{AddExtensionLayer, Json, Router};
use color_eyre::Result;
use flume::Sender;
use serde::Deserialize;
use thiserror::Error;
use tracing::info;
use uuid::Uuid;

use crate::structures::{Instruction, Message, Replication};

pub async fn start_http_server(msg_tx: Sender<Message>, host: IpAddr, port: u16) -> Result<()> {
    let addr = SocketAddr::new(host, port);
    info!("HTTP Server listening on {}", addr);

    let app = Router::new()
        .route("/global_message", post(post_global_message))
        .layer(AddExtensionLayer::new(msg_tx));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

#[derive(Debug, Deserialize)]
struct PartialGlobalMessage {
    parameter: Option<String>,
    world_name: String,
}

impl From<PartialGlobalMessage> for Message {
    fn from(partial: PartialGlobalMessage) -> Self {
        Self {
            instruction: Instruction::GlobalMessage,
            parameter: partial.parameter,
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

#[derive(Debug, Error)]
enum AppError {
    #[error(transparent)]
    SendError(#[from] flume::SendError<Message>),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let reply = (
            StatusCode::INTERNAL_SERVER_ERROR,
            self.to_string().into_response(),
        );

        reply.into_response()
    }
}

async fn post_global_message(
    Extension(msg_tx): Extension<Sender<Message>>,
    Json(partial_message): Json<PartialGlobalMessage>,
) -> Result<impl IntoResponse, AppError> {
    msg_tx.send_async(partial_message.into()).await?;

    Ok(StatusCode::NO_CONTENT)
}
