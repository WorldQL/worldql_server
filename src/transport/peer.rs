use futures_util::stream::SplitSink;
use futures_util::SinkExt;
use thiserror::Error;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message as WsMessage;
use tokio_tungstenite::WebSocketStream;

use crate::structures::Message;

#[cfg(feature = "websocket")]
type WebSocketConnection = SplitSink<WebSocketStream<TcpStream>, WsMessage>;

#[derive(Debug)]
pub struct Peer {
    connection: PeerConnection,
}

impl Peer {
    #[cfg(feature = "websocket")]
    pub fn new_ws(ws_conn: WebSocketConnection) -> Self {
        Self {
            connection: PeerConnection::WebSocket(ws_conn),
        }
    }

    pub async fn send(&mut self, message: Message) -> Result<(), SendError> {
        self.connection.send(message).await
    }
}

#[derive(Debug)]
pub enum PeerConnection {
    #[cfg(feature = "websocket")]
    WebSocket(WebSocketConnection),
    #[cfg(feature = "zeromq")]
    ZeroMQ,
}

impl PeerConnection {
    pub async fn send(&mut self, message: Message) -> Result<(), SendError> {
        let bytes = message.serialize();
        self.send_raw(bytes).await?;

        Ok(())
    }

    async fn send_raw(&mut self, bytes: Vec<u8>) -> Result<(), SendError> {
        match self {
            #[cfg(feature = "websocket")]
            PeerConnection::WebSocket(conn) => {
                let message = WsMessage::Binary(bytes);
                conn.send(message).await?;

                Ok(())
            }
            #[cfg(feature = "zeromq")]
            PeerConnection::ZeroMQ => {
                // TODO
                todo!()
            }
        }
    }
}

#[derive(Debug, Error)]
pub enum SendError {
    #[cfg(feature = "websocket")]
    #[error(transparent)]
    WsError(#[from] tokio_tungstenite::tungstenite::Error),
}
