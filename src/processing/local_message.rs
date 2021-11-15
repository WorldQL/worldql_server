use color_eyre::Result;
use tracing::debug;

use crate::structures::Message;
use crate::subscriptions::WorldMap;
use crate::trace_packet;
use crate::transport::ThreadPeerMap;

pub async fn handle_local_message(
    message: Message,
    peer_map: &ThreadPeerMap,
    world_map: &mut WorldMap,
) -> Result<()> {
    trace_packet!("{}", &message);

    let cube = match message.position {
        Some(pos) => pos,
        None => {
            // TODO: Disconnect peer
            debug!(
                "invalid LocalMessage from peer {}, missing position",
                &message.sender_uuid
            );

            return Ok(());
        }
    };

    let area_map = world_map.get_mut(&message.world_name);
    let uuid = message.sender_uuid;
    let peers = area_map
        .get_subscribed_peers(cube)
        .filter(|peer| *peer != uuid);

    let mut map = peer_map.write().await;
    let _ = map.broadcast_to(message, peers).await;

    Ok(())
}
