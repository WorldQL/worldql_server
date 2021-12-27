use std::net::IpAddr;

use color_eyre::Result;
use flume::Sender;
use futures_util::StreamExt;
use tracing::{debug, info, warn};

use super::ThreadPeerMap;
use crate::structures::{Instruction, Message};

pub async fn start_zeromq_incoming(
    peer_map: ThreadPeerMap,
    msg_tx: Sender<Message>,
    handshake_tx: Sender<Message>,
    server_host: IpAddr,
    server_port: u16,
    ctx: tmq::Context,
) -> Result<()> {
    let pull_addr = format!("tcp://{}:{}", &server_host, &server_port);
    let mut pull_socket = tmq::pull(&ctx.clone()).bind(&pull_addr)?;
    info!(
        "ZeroMQ PULL Server listening on {}:{}",
        server_host, server_port
    );

    loop {
        let msg = pull_socket.next().await;
        match msg {
            None => break,
            Some(msg) => {
                let msg = msg?;
                if msg.len() != 1 {
                    warn!("Dropping multipart ZeroMQ message. Clients should not send multipart messages to WorldQL.");
                    continue;
                }

                // Take first element from multipart
                let data = msg.into_iter().next().unwrap();
                let message_result = Message::deserialize(&data);

                let message = match message_result {
                    Ok(m) => m,
                    Err(error) => {
                        debug!("dropping invalid zmq message: deserialize error");

                        #[cfg(debug_assertions)]
                        tracing::error!("{:?}", error);

                        continue;
                    }
                };

                // Run in new scope to avoid blocking PeerMap Lock
                {
                    let map = peer_map.read().await;
                    if map.contains_key(&message.sender_uuid) {
                        // Only forward non-handshake messages
                        if message.instruction != Instruction::Handshake {
                            msg_tx.send_async(message).await?;
                        }

                        continue;
                    }
                }

                if message.instruction != Instruction::Handshake || message.parameter.is_none() {
                    // Ignore message
                    // TODO: Drop connection
                    continue;
                }

                // Send handshake message to ZeroMQ Outgoing Thread
                handshake_tx.send_async(message).await?;
            }
        }
    }

    Ok(())
}
