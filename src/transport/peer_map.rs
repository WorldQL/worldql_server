use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use tokio::sync::RwLock;
use tracing::{debug, trace};
use uuid::Uuid;

use super::peer::Peer;
use super::SendError;
use crate::structures::Message;

pub type ThreadPeerMap = Arc<RwLock<PeerMap>>;

#[derive(Debug)]
pub struct PeerMap(HashMap<Uuid, Peer>);

macro_rules! broadcast_to {
    ($message: expr, $peers: expr) => {{
        let bytes = $message.serialize();

        let mut jobs = vec![];
        for peer in $peers {
            jobs.push(peer.send_raw(bytes.clone()));
        }

        for result in futures_util::future::join_all(jobs).await {
            if let Err(error) = result {
                // TODO: Remove peers that error
                debug!("broadcast error: {:?}", error);
            }
        }

        Ok(())
    }};
}

impl PeerMap {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn contains_key(&self, uuid: &Uuid) -> bool {
        self.0.contains_key(uuid)
    }

    pub fn get(&self, uuid: &Uuid) -> Option<&Peer> {
        self.0.get(uuid)
    }

    pub fn get_mut(&mut self, uuid: &Uuid) -> Option<&mut Peer> {
        self.0.get_mut(uuid)
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
        broadcast_to!(message, self.0.values_mut())
    }

    pub async fn broadcast_to(
        &mut self,
        message: Message,
        peers: impl Iterator<Item = Uuid>,
    ) -> Result<(), SendError> {
        let peers = peers.collect::<HashSet<_>>();
        let peers = self
            .0
            .values_mut()
            .filter(|peer| peers.contains(peer.uuid()));

        broadcast_to!(message, peers)
    }

    pub async fn broadcast_except(
        &mut self,
        message: Message,
        except: Uuid,
    ) -> Result<(), SendError> {
        let peers = self.0.values_mut().filter(|peer| *peer.uuid() != except);
        broadcast_to!(message, peers)
    }
}
