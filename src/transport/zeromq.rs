use color_eyre::Result;
use tokio::sync::mpsc::UnboundedSender;
use tracing::info;

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

    // TODO
    Ok(())
}
