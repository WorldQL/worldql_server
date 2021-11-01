use std::ops::RangeInclusive;

use color_eyre::Result;
use tokio::sync::mpsc::UnboundedSender;
use tracing::info;

use super::ThreadPeerMap;
use crate::structures::Message;

pub async fn start_zeromq_server(
    peer_map: ThreadPeerMap,
    msg_tx: UnboundedSender<Message>,
    server_port: u16,
    client_ports: RangeInclusive<u16>,
) -> Result<()> {
    let ctx = tmq::Context::new();

    let pull_addr = format!("tcp://127.0.0.1:{}", server_port);
    let pull_socket = tmq::pull(&ctx).bind(&pull_addr)?;
    info!("ZeroMQ PULL Server listening on port {}", server_port);

    // TODO
    Ok(())
}
