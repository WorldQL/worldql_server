use color_eyre::Result;
use futures_util::StreamExt;
use tokio::sync::mpsc::UnboundedSender;
use tracing::{info, trace};

use super::ThreadPeerMap;
use crate::structures::Message;
use crate::utils::PortRange;

pub async fn start_zeromq_server(
    peer_map: ThreadPeerMap,
    msg_tx: UnboundedSender<Message>,
    server_port: u16,
    client_ports: PortRange,
) -> Result<()> {
    let ctx = tmq::Context::new();

    let pull_addr = format!("tcp://127.0.0.1:{}", server_port);
    let mut pull_socket = tmq::pull(&ctx).bind(&pull_addr)?;
    info!("ZeroMQ PULL Server listening on port {}", server_port);

    loop {
        let msg = pull_socket.next().await;
        match msg {
            None => break,
            Some(msg) => {
                let msg = msg?;
                if msg.len() != 1 {
                    trace!("dropping multipart zmq message");
                    continue;
                }

                // Take first element from multipart
                let data = msg.into_iter().next().unwrap();
                let message_result = Message::deserialize(&data);

                let message = match message_result {
                    Ok(m) => m,
                    Err(error) => {
                        trace!("dropping invalid zmq message: deserialize error");

                        #[cfg(debug_assertions)]
                        tracing::error!("{}", error);

                        continue;
                    }
                };

                // Forward messages with known UUIDs
                {
                    let map = peer_map.read().await;
                    if map.contains_key(&message.sender_uuid) {
                        msg_tx.send(message)?;
                        continue;
                    }
                }

                // TODO: Handle handshakes
                dbg!(&message);
            }
        }
    }

    Ok(())
}
