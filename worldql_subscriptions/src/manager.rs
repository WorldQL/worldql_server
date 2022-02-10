use ahash::{AHashMap, AHashSet};
use uuid::Uuid;

use crate::Area;

/// Manager for World and Area subscriptions
#[derive(Debug, Default)]
pub struct SubscriptionManager {
    area_size: u16,
    map: AHashMap<String, WorldManager>,
    empty_set: PeerSet,
}

type PeerSet = AHashSet<Uuid>;

#[derive(Debug, Default)]
struct WorldManager {
    world_subscriptions: PeerSet,
    area_subscriptions: AHashMap<Area, PeerSet>,
}

impl SubscriptionManager {
    /// Create a new [`SubscriptionManager`]
    #[inline]
    #[must_use]
    pub fn new(area_size: u16) -> Self {
        let map = AHashMap::new();
        let empty_set = AHashSet::new();

        Self {
            area_size,
            map,
            empty_set,
        }
    }

    /// Returns the configured area size for this manager
    #[inline]
    #[must_use]
    pub fn area_size(&self) -> u16 {
        self.area_size
    }

    /// Returns `true` if the peer is subscribed to the given world, or any area within that world
    #[must_use]
    pub fn is_subscribed_world(&self, peer: Uuid, world: &str) -> bool {
        self.map
            .get(world)
            .map_or(false, |manager| manager.world_subscriptions.contains(&peer))
    }

    /// Returns `true` if the peer is subscribed to the given area
    #[must_use]
    pub fn is_subscribed_area(&self, peer: Uuid, world: &str, area: Area) -> bool {
        self.map.get(world).map_or(false, |manager| {
            manager
                .area_subscriptions
                .get(&area)
                .map_or(false, |peers| peers.contains(&peer))
        })
    }

    /// Returns the number of peers that are subscribed to the given world
    #[must_use]
    pub fn world_subscription_count(&self, world: &str) -> usize {
        match self.map.get(world) {
            None => 0,
            Some(manager) => manager.world_subscriptions.len(),
        }
    }

    /// Returns the number of peers that are subscribed to the given area
    #[must_use]
    pub fn area_subscription_count(&self, world: &str, area: Area) -> usize {
        match self.map.get(world) {
            None => 0,
            Some(manager) => match manager.area_subscriptions.get(&area) {
                None => 0,
                Some(peers) => peers.len(),
            },
        }
    }

    /// Returns an iterator of peers that are subscribed to the given world
    pub fn get_subscribed_to_world(&self, world: &str) -> impl Iterator<Item = Uuid> + '_ {
        match self.map.get(world) {
            None => self.empty_set.iter().copied(),
            Some(manager) => manager.world_subscriptions.iter().copied(),
        }
    }

    /// Returns an iterator of peers that are subscribed to the given area
    pub fn get_subscribed_to_area(
        &self,
        world: &str,
        area: Area,
    ) -> impl Iterator<Item = Uuid> + '_ {
        match self.map.get(world) {
            None => self.empty_set.iter().copied(),
            Some(manager) => match manager.area_subscriptions.get(&area) {
                None => self.empty_set.iter().copied(),
                Some(peers) => peers.iter().copied(),
            },
        }
    }

    /// Subscribe to a world
    ///
    /// Returns `true` if the subscription was not already present
    #[must_use]
    pub fn subscribe_to_world(&mut self, peer: Uuid, world: impl Into<String>) -> bool {
        let manager = self
            .map
            .entry(world.into())
            .or_insert_with(Default::default);

        manager.world_subscriptions.insert(peer)
    }

    /// Unsubscribe from a world
    ///
    /// Will also unsubscribe from all areas within the world
    ///
    /// Returns `true` if the subscription was present before removal
    #[must_use]
    pub fn unsubscribe_from_world(&mut self, peer: Uuid, world: &str) -> bool {
        let manager = self
            .map
            .entry(world.into())
            .or_insert_with(Default::default);

        let modified = manager.world_subscriptions.remove(&peer);

        for peers in manager.area_subscriptions.values_mut() {
            peers.remove(&peer);
        }

        modified
    }

    /// Subscribe to an area within a world
    ///
    /// Will also implicitly subscribe to the world
    ///
    /// Returns `true` if the subscription was not already present
    #[must_use]
    pub fn subscribe_to_area(&mut self, peer: Uuid, world: impl Into<String>, area: Area) -> bool {
        let manager = self
            .map
            .entry(world.into())
            .or_insert_with(Default::default);

        manager.world_subscriptions.insert(peer);

        let peers = manager
            .area_subscriptions
            .entry(area)
            .or_insert_with(Default::default);

        peers.insert(peer)
    }

    /// Unsubscribe from an area within a world
    ///
    /// Will not implicitly remove world subscriptions,
    /// use [`SubscriptionManager::unsubscribe_from_world`] to explicitly unsubscribe
    ///
    /// Returns `true` if the subscription was present before removal
    #[must_use]
    pub fn unsubscribe_from_area(&mut self, peer: Uuid, world: &str, area: Area) -> bool {
        let manager = self
            .map
            .entry(world.into())
            .or_insert_with(Default::default);

        let peers = manager
            .area_subscriptions
            .entry(area)
            .or_insert_with(Default::default);

        peers.remove(&peer)
    }

    /// Completely remove a peer by unsubscribing them from all areas and worlds
    pub fn remove_peer(&mut self, peer: Uuid) {
        for manager in self.map.values_mut() {
            manager.world_subscriptions.remove(&peer);

            for peers in manager.area_subscriptions.values_mut() {
                peers.remove(&peer);
            }
        }
    }
}
