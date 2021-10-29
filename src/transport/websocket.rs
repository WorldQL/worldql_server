use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use color_eyre::Result;
use futures_util::StreamExt;
use tokio::net::{TcpListener, TcpStream};
use tracing::{debug, info};

use crate::structures::Message;

pub async fn start_websocket_server(ws_port: u16) -> Result<()> {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), ws_port);
    let listener = TcpListener::bind(&addr).await?;
    info!("WebSocket Server listening on port {}", ws_port);

    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream.peer_addr()?;
        debug!("peer address: {}", peer);

        tokio::spawn(handle_connection(peer, stream));
    }

    Ok(())
}

async fn handle_connection(peer: SocketAddr, raw_stream: TcpStream) -> Result<()> {
    let stream = tokio_tungstenite::accept_async(raw_stream).await?;
    debug!("websocket connection established: {}", &peer);

    let (mut outgoing, mut incoming) = stream.split();

    loop {
        tokio::select! {
            msg = incoming.next() => {
                match msg {
                    None => break,
                    Some(msg) => {
                        let msg = msg?;
                        if msg.is_close() {
                            break;
                        }

                        if !msg.is_binary() {
                            continue;
                        }

                        let data = msg.into_data();
                        let message_result = Message::deserialize(&data);

                        if message_result.is_err() {
                            debug!("deserialize error from peer: {}", &peer);
                            continue;
                        }

                        // TODO: Handle messages
                        let message = message_result.unwrap();
                        dbg!(&message);
                    }
                }
            }
        }
    }

    debug!("websocket connection closed: {}", &peer);
    Ok(())
}
