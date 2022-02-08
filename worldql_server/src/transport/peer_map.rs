use std::sync::Arc;

use ahash::{AHashMap, AHashSet};
use flume::Sender;
use tokio::sync::RwLock;
use tracing::{debug, info, trace};
use uuid::Uuid;
use worldql_messages::outgoing::{
    OutgoingMessage, OutgoingMessageEvent, PeerConnectEvent, PeerDisconnectEvent,
};
use worldql_messages::serialization::SerializeBinary;

use super::SendError;
use crate::transport::Peer;

type BoxedPeer = Box<dyn Peer + Sync + Send>;
pub type ThreadPeerMap = Arc<RwLock<PeerMap>>;

pub struct PeerMap {
    map: AHashMap<Uuid, BoxedPeer>,
    on_remove: Sender<Uuid>,
}

macro_rules! broadcast_to {
    ($event: expr, $peers: expr) => {{
        let message: OutgoingMessage = $event.into();
        let bytes = message.serialize_binary().unwrap();

        let mut jobs = vec![];
        for peer in $peers {
            jobs.push(peer.send_bytes(&bytes));
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
    pub fn new(on_remove: Sender<Uuid>) -> Self {
        Self {
            map: AHashMap::new(),
            on_remove,
        }
    }

    // region: Lookups and Getters
    /// Returns `true` if the map contains a [`Peer`] for the specified [`Uuid`].
    #[inline]
    pub fn contains_key(&self, uuid: &Uuid) -> bool {
        self.map.contains_key(uuid)
    }

    /// Returns a reference to the [`Peer`] corresponding to the [`Uuid`].
    #[inline]
    pub fn get(&self, uuid: &Uuid) -> Option<&BoxedPeer> {
        self.map.get(uuid)
    }

    /// Returns a mutable reference to the [`Peer`] corresponding to the [`Uuid`].
    #[inline]
    pub fn get_mut(&mut self, uuid: &Uuid) -> Option<&mut BoxedPeer> {
        self.map.get_mut(uuid)
    }

    /// Returns the number of connected Peers.
    #[inline]
    pub fn size(&self) -> usize {
        self.map.len()
    }

    /// Returns an iterator of [`Uuid`] items for each contained [`Peer`].
    #[inline]
    pub fn peers_iter(&self) -> impl Iterator<Item = Uuid> + '_ {
        self.map.keys().copied()
    }
    // endregion

    // region: Map Modifiers
    /// Inserts a [`Peer`] into the map.
    ///
    /// If the map did not have this key present, [`None`] is returned.
    #[inline]
    pub async fn insert(&mut self, uuid: Uuid, peer: BoxedPeer) -> Option<BoxedPeer> {
        info!("[{}] {} Peer Connected", peer.addr(), peer.type_string());

        let existing = self.map.insert(uuid, peer);

        // Broadcast PeerConnect event to all peers except newly connected
        let event = PeerConnectEvent::new(uuid);
        let _ = self.broadcast_except(event.into(), uuid).await;

        existing
    }

    /// Removes a [`Peer`] from the map, returning the [`Peer`] at for the
    /// given [`Uuid`] if the it was previously in the map.
    #[inline]
    pub async fn remove(&mut self, uuid: &Uuid) -> Option<BoxedPeer> {
        trace!("trying to remove peer id {} from map", &uuid);
        let result = self.map.remove(uuid);

        if let Some(peer) = &result {
            // debug!("removed peer {} from map", peer);
            info!("[{}] {} Peer Disconnected", peer.addr(), peer.type_string());

            // Broadcast PeerDisconnect event to all peers
            // TODO: Implement timeout boolean
            let event = PeerDisconnectEvent::new(peer.uuid(), false);
            let _ = self.broadcast_all(event.into()).await;
        }

        let _ = self.on_remove.send(*uuid);
        result
    }
    // endregion

    // region: Broadcast Functions
    /// Broadcast an [`OutgoingMessage`] to all peers in the map.
    pub async fn broadcast_all(&mut self, event: OutgoingMessageEvent) -> Result<(), SendError> {
        broadcast_to!(event, self.map.values_mut())
    }

    /// Broadcast an [`OutgoingMessageEvent`] to all peers that correspond to the [`Uuid`] iterator.
    pub async fn broadcast_to(
        &mut self,
        event: OutgoingMessageEvent,
        peers: impl Iterator<Item = Uuid>,
    ) -> Result<(), SendError> {
        let peers = peers.collect::<AHashSet<_>>();
        let peers = self
            .map
            .values_mut()
            .filter(|peer| peers.contains(&peer.uuid()));

        broadcast_to!(event, peers)
    }

    /// Broadcast an [`OutgoingMessageEvent`] to every peer except one, usually the one who triggered the
    /// broadcast.
    pub async fn broadcast_except(
        &mut self,
        event: OutgoingMessageEvent,
        except: Uuid,
    ) -> Result<(), SendError> {
        let peers = self.map.values_mut().filter(|peer| peer.uuid() != except);
        broadcast_to!(event, peers)
    }
    // endregion
}
