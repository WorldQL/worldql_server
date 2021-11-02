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

                if message_result.is_err() {
                    trace!("dropping invalid zmq message");
                    continue;
                }

                let message = message_result.unwrap();

                // TODO: Handle handshakes and message forwarding
                dbg!(&message);
            }
        }
    }

    Ok(())
}
