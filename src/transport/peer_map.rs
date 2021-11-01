use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::RwLock;

use super::peer::Peer;
use super::SendError;
use crate::structures::Message;

pub type ThreadPeerMap = Arc<RwLock<PeerMap>>;

#[derive(Debug)]
pub struct PeerMap(HashMap<String, Peer>);

impl PeerMap {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub async fn broadcast(&mut self, message: Message) -> Result<(), SendError> {
        let bytes = message.serialize();

        for peer in self.0.values_mut() {
            // TODO: Run in parallel, don't terminate on single failure
            peer.send_raw(bytes.clone()).await?;
        }

        Ok(())
    }
}
