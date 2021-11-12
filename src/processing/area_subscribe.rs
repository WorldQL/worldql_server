use color_eyre::Result;

use crate::structures::Message;
use crate::subscriptions::AreaMap;
use crate::transport::ThreadPeerMap;

pub async fn handle_area_subscribe(
    message: Message,
    peer_map: &ThreadPeerMap,
    area_map: &mut AreaMap,
) -> Result<()> {
    // TODO
    todo!()
}
