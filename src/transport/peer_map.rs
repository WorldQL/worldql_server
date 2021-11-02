use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::RwLock;
use tracing::trace;
use uuid::Uuid;

use super::peer::Peer;
use super::SendError;
use crate::structures::Message;

pub type ThreadPeerMap = Arc<RwLock<PeerMap>>;

#[derive(Debug)]
pub struct PeerMap(HashMap<Uuid, Peer>);

impl PeerMap {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn contains_key(&self, uuid: &Uuid) -> bool {
        self.0.contains_key(uuid)
    }

    pub fn insert(&mut self, uuid: Uuid, peer: Peer) -> Option<Peer> {
        trace!("inserting peer {} into map", &peer);
        self.0.insert(uuid, peer)
    }

    pub fn remove(&mut self, uuid: &Uuid) -> Option<Peer> {
        trace!("trying to remove peer id {} from map", &uuid);
        let result = self.0.remove(uuid);

        if result.is_some() {
            trace!("removed peer id {} from map", &uuid);
        }

        result
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
