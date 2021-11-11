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

    // region: Lookups and Getters
    /// Returns `true` if the map contains a [`Peer`] for the specified [`Uuid`].
    #[inline]
    pub fn contains_key(&self, uuid: &Uuid) -> bool {
        self.0.contains_key(uuid)
    }

    /// Returns a reference to the [`Peer`] corresponding to the [`Uuid`].
    #[inline]
    pub fn get(&self, uuid: &Uuid) -> Option<&Peer> {
        self.0.get(uuid)
    }

    /// Returns a mutable reference to the [`Peer`] corresponding to the [`Uuid`].
    #[inline]
    pub fn get_mut(&mut self, uuid: &Uuid) -> Option<&mut Peer> {
        self.0.get_mut(uuid)
    }

    /// Returns an iterator of [`Uuid`] items for each contained [`Peer`].
    pub fn peers_iter(&self) -> impl Iterator<Item = Uuid> + '_ {
        self.0.keys().copied()
    }
    // endregion

    // region: Map Modifiers
    /// Inserts a [`Peer`] into the map.
    ///
    /// If the map did not have this key present, [`None`] is returned.
    #[inline]
    pub fn insert(&mut self, uuid: Uuid, peer: Peer) -> Option<Peer> {
        trace!("inserting peer {} into map", &peer);
        self.0.insert(uuid, peer)
    }

    /// Removes a [`Peer`] from the map, returning the [`Peer`] at for the
    /// given [`Uuid`] if the it was previously in the map.
    #[inline]
    pub fn remove(&mut self, uuid: &Uuid) -> Option<Peer> {
        trace!("trying to remove peer id {} from map", &uuid);
        let result = self.0.remove(uuid);

        if result.is_some() {
            trace!("removed peer id {} from map", &uuid);
        }

        result
    }
    // endregion

    // region: Broadcast Functions
    /// Broadcast a [`Message`] to all peers in the map.
    pub async fn broadcast_all(&mut self, message: Message) -> Result<(), SendError> {
        broadcast_to!(message, self.0.values_mut())
    }

    /// Broadcast a [`Message`] to all peers that correspond to the [`Uuid`] iterator.
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

    /// Broadcast a [`Message`] to every peer except one, usually the one who triggered the
    /// broadcast.
    pub async fn broadcast_except(
        &mut self,
        message: Message,
        except: Uuid,
    ) -> Result<(), SendError> {
        let peers = self.0.values_mut().filter(|peer| *peer.uuid() != except);
        broadcast_to!(message, peers)
    }
    // endregion
}
