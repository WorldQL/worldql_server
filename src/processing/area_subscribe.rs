use color_eyre::Result;
use tracing::debug;

use crate::structures::Message;
use crate::subscriptions::WorldMap;
use crate::trace_packet;
use crate::transport::ThreadPeerMap;

pub(super) fn handle_area_subscribe(
    message: Message,
    peer_map: &ThreadPeerMap,
    world_map: &mut WorldMap,
) -> Result<()> {
    trace_packet!("{}", &message);

    let uuid = message.sender_uuid;
    let cube = match message.position {
        Some(pos) => pos,
        None => {
            // TODO: Disconnect peer
            debug!(
                "invalid AreaSubscribe from peer {}, missing position",
                &uuid
            );

            return Ok(());
        }
    };

    let area_map = world_map.get_mut(&message.world_name);
    area_map.add_subscription(uuid, cube);

    Ok(())
}
