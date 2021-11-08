use color_eyre::Result;
use futures_util::StreamExt;
use tmq::push::Push;
use tokio::sync::mpsc::UnboundedSender;
use tracing::{info, trace, warn};
use crate::outgoing_zeromq_owner::MessageAndClientUUID;


use super::ThreadPeerMap;
use crate::structures::{Instruction, Message};
use crate::utils::PortRange;

pub async fn start_zeromq_server(
    peer_map: ThreadPeerMap,
    msg_tx: UnboundedSender<Message>,
    server_port: u16,
    zmq_outgoing_tx: UnboundedSender<MessageAndClientUUID>,
    ctx: tmq::Context,
) -> Result<()> {

    let pull_addr = format!("tcp://127.0.0.1:{}", server_port);
    let mut pull_socket = tmq::pull(&ctx.clone()).bind(&pull_addr)?;
    info!("ZeroMQ PULL Server listening on port {}", server_port);

    loop {
        let msg = pull_socket.next().await;
        match msg {
            None => break,
            Some(msg) => {
                let msg = msg?;
                if msg.len() != 1 {
                    warn!("Dropping multipart zmq message. Clients should not send multipart messages to WorldQL.");
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
                        tracing::error!("{:?}", error);

                        continue;
                    }
                };

                // Forward messages with known UUIDs
                {
                    let map = peer_map.read().await;
                    if map.contains_key(&message.sender_uuid) {
                        // Only forward non-handshake messages
                        if message.instruction != Instruction::Handshake {
                            msg_tx.send(message)?;
                        }

                        continue;
                    }
                }

                // Ignore non-handshake messages from unknown clients
                if message.instruction != Instruction::Handshake {
                    continue;
                }

                // TODO: Handle handshakes


                if message.instruction == Instruction::Handshake {
                    // Forward the message to the OutgoingZeroMQOwner
                    let sender_uuid = message.sender_uuid;
                    let m = MessageAndClientUUID {
                        message: message,
                        // ignored for handshakes
                        client: sender_uuid,
                    };
                    zmq_outgoing_tx.send(m);
                }
            }
        }
    }

    Ok(())
}
