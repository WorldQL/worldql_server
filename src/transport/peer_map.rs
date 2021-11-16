use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::{Duration, Instant};

use flume::Sender;
use tokio::sync::RwLock;
use tracing::{debug, trace};
use uuid::Uuid;

use super::peer::Peer;
use super::SendError;
use crate::structures::{Instruction, Message};

pub type ThreadPeerMap = Arc<RwLock<PeerMap>>;

#[derive(Debug)]
pub struct PeerMap {
    map: HashMap<Uuid, Peer>,
    on_remove: Sender<Uuid>,
}

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
    pub fn new(on_remove: Sender<Uuid>) -> Self {
        Self {
            map: HashMap::new(),
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
    pub fn get(&self, uuid: &Uuid) -> Option<&Peer> {
        self.map.get(uuid)
    }

    /// Returns a mutable reference to the [`Peer`] corresponding to the [`Uuid`].
    #[inline]
    pub fn get_mut(&mut self, uuid: &Uuid) -> Option<&mut Peer> {
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

    /// Returns an iterator of [`Uuid`] items for each [`Peer`] that is considered stale.
    #[inline]
    pub fn stale_peers_iter(&self, max_duration: Duration) -> impl Iterator<Item = Uuid> + '_ {
        let now = Instant::now();
        self.map
            .values()
            .filter_map(move |peer| match peer.is_stale(&now, &max_duration) {
                false => None,
                true => Some(peer.uuid()),
            })
            .copied()
    }
    // endregion

    // region: Map Modifiers
    /// Inserts a [`Peer`] into the map.
    ///
    /// If the map did not have this key present, [`None`] is returned.
    #[inline]
    pub async fn insert(&mut self, uuid: Uuid, peer: Peer) -> Option<Peer> {
        debug!("inserting peer {} into map", &peer);
        let existing = self.map.insert(uuid, peer);

        let message = Message {
            instruction: Instruction::PeerConnect,
            parameter: Some(uuid.to_string()),
            ..Default::default()
        };

        // Broadcast PeerConnect to all except new Peer
        let _ = self.broadcast_except(message, uuid).await;

        existing
    }

    /// Removes a [`Peer`] from the map, returning the [`Peer`] at for the
    /// given [`Uuid`] if the it was previously in the map.
    #[inline]
    pub async fn remove(&mut self, uuid: &Uuid) -> Option<Peer> {
        trace!("trying to remove peer id {} from map", &uuid);
        let result = self.map.remove(uuid);

        if result.is_some() {
            debug!("removed peer id {} from map", &uuid);

            let message = Message {
                instruction: Instruction::PeerDisconnect,
                parameter: Some(uuid.to_string()),
                ..Default::default()
            };

            // Broadcast PeerDisconnect to all
            let _ = self.broadcast_all(message).await;
        }

        let _ = self.on_remove.send(*uuid);
        result
    }
    // endregion

    // region: Broadcast Functions
    /// Broadcast a [`Message`] to all peers in the map.
    pub async fn broadcast_all(&mut self, message: Message) -> Result<(), SendError> {
        broadcast_to!(message, self.map.values_mut())
    }

    /// Broadcast a [`Message`] to all peers that correspond to the [`Uuid`] iterator.
    pub async fn broadcast_to(
        &mut self,
        message: Message,
        peers: impl Iterator<Item = Uuid>,
    ) -> Result<(), SendError> {
        let peers = peers.collect::<HashSet<_>>();
        let peers = self
            .map
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
        let peers = self.map.values_mut().filter(|peer| *peer.uuid() != except);
        broadcast_to!(message, peers)
    }
    // endregion
}
