use std::collections::{HashMap, HashSet};
use std::fmt::Display;

use tracing::trace;
use uuid::Uuid;

use super::{CubeArea, ToCubeArea};

#[derive(Debug)]
pub struct AreaMap {
    cube_size: u16,
    world_name: String,

    map: HashMap<CubeArea, HashSet<Uuid>>,
    subscribed_peers: HashSet<Uuid>,
    empty_set: HashSet<Uuid>,
}

impl AreaMap {
    pub fn new(cube_size: u16, world_name: String) -> Self {
        Self {
            cube_size,
            world_name,

            map: HashMap::new(),
            subscribed_peers: HashSet::new(),
            empty_set: HashSet::new(),
        }
    }

    /// Returns `true` if the [`crate::transport::Peer`] corresponding to the given UUID
    /// is subscribed to the given area.
    pub fn is_peer_subscribed(&self, uuid: &Uuid, cube: impl ToCubeArea) -> bool {
        let cube = cube.to_cube_area(self.cube_size);
        let entry = self.map.get(&cube);

        match entry {
            None => false,
            Some(set) => set.contains(uuid),
        }
    }

    /// Returns `true` if the [`crate::transport::Peer`] corresponding to the given UUID
    /// is subscribed to this world.
    #[inline]
    pub fn is_peer_subscribed_any(&self, uuid: &Uuid) -> bool {
        self.subscribed_peers.contains(uuid)
    }

    /// Returns a vector of [`crate::transport::Peer`] structs which are subscribed to the
    /// given area.
    pub fn get_subscribed_peers(&self, cube: impl ToCubeArea) -> impl Iterator<Item = Uuid> + '_ {
        let cube = cube.to_cube_area(self.cube_size);
        let entry = self.map.get(&cube);

        match entry {
            None => self.empty_set.iter().copied(),
            Some(set) => set.iter().copied(),
        }
    }

    /// Returns a vector of [`crate::transport::Peer`] structs which are subscribed to
    /// this world.
    #[inline]
    pub fn get_subscribed_any_peers(&self) -> impl Iterator<Item = Uuid> + '_ {
        self.subscribed_peers.iter().copied()
    }

    /// If the subscription was added, `true` is returned.
    ///
    /// If the subscription was already present, `false` is returned
    pub fn add_subscription(&mut self, uuid: Uuid, cube: impl ToCubeArea) -> bool {
        let cube = cube.to_cube_area(self.cube_size);
        let entry = self.map.entry(cube).or_insert_with(Default::default);

        trace!(
            "peer {} subscribed to region {} in world \"{}\"",
            &uuid,
            &cube,
            &self.world_name
        );

        self.subscribed_peers.insert(uuid);
        entry.insert(uuid)
    }

    /// Returns whether the subscription was removed.
    pub fn remove_subscription(&mut self, uuid: &Uuid, cube: impl ToCubeArea) -> bool {
        let cube = cube.to_cube_area(self.cube_size);

        // Early return if no subscriptions are present
        if !self.map.contains_key(&cube) {
            return false;
        }

        trace!(
            "peer {} unsubscribed from region {} in world \"{}\"",
            &uuid,
            &cube,
            &self.world_name
        );

        // Remove from HashSet
        let entry = self.map.entry(cube).or_insert_with(Default::default);
        let removed = entry.remove(uuid);

        // Remove HashSet from HashMap if empty
        if entry.is_empty() {
            self.map.remove(&cube);
        }

        // Remove from subscriptions set if no subscriptions are left
        let has_other_subs = self.map.values().any(|set| set.contains(uuid));
        if !has_other_subs {
            self.subscribed_peers.remove(uuid);
        }

        removed
    }

    /// Completely removes a [`crate::transport::Peer`] from the map.
    ///
    /// Used in the event of a disconnect.
    pub fn remove_peer(&mut self, uuid: &Uuid) -> bool {
        self.subscribed_peers.remove(uuid);

        let mut removed = false;
        for peers in self.map.values_mut() {
            if peers.remove(uuid) {
                removed = true;
            }
        }

        removed
    }
}

impl Display for AreaMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ world_name = \"{}\", area_count = {} }}",
            self.world_name,
            self.map.len()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structures::Vector3;

    #[test]
    fn area_subscriptions() {
        let uuid = Uuid::new_v4();
        let mut map = AreaMap::new(16, "world".into());

        let cube_1 = CubeArea::new(0, 0, 0);
        let cube_2 = CubeArea::new(16, 16, 16);

        // Equivalent to cube_2
        let vec_1 = Vector3::new(6.3, 1.0, 10.5);

        // No subscriptions yet
        assert!(!map.is_peer_subscribed(&uuid, cube_1));
        assert!(!map.is_peer_subscribed(&uuid, cube_2));
        assert!(!map.is_peer_subscribed(&uuid, vec_1));

        // Subscribe to cube_1
        map.add_subscription(uuid, cube_1);
        assert!(map.is_peer_subscribed(&uuid, cube_1));
        assert!(!map.is_peer_subscribed(&uuid, cube_2));
        assert!(!map.is_peer_subscribed(&uuid, vec_1));

        // Subscribe to cube_2
        map.add_subscription(uuid, cube_2);
        assert!(map.is_peer_subscribed(&uuid, cube_1));
        assert!(map.is_peer_subscribed(&uuid, cube_2));
        assert!(map.is_peer_subscribed(&uuid, vec_1));

        // Unsubscribe from cube_1
        map.remove_subscription(&uuid, cube_1);
        assert!(!map.is_peer_subscribed(&uuid, cube_1));
        assert!(map.is_peer_subscribed(&uuid, cube_2));
        assert!(map.is_peer_subscribed(&uuid, vec_1));

        // Unsubscribe from cube_2
        map.remove_subscription(&uuid, cube_2);
        assert!(!map.is_peer_subscribed(&uuid, cube_1));
        assert!(!map.is_peer_subscribed(&uuid, cube_2));
        assert!(!map.is_peer_subscribed(&uuid, vec_1));

        // Subscribe to vec_1
        map.add_subscription(uuid, vec_1);
        assert!(!map.is_peer_subscribed(&uuid, cube_1));
        assert!(map.is_peer_subscribed(&uuid, cube_2));
        assert!(map.is_peer_subscribed(&uuid, vec_1));

        // Unsubscribe from vec_1
        map.remove_subscription(&uuid, vec_1);
        assert!(!map.is_peer_subscribed(&uuid, cube_1));
        assert!(!map.is_peer_subscribed(&uuid, cube_2));
        assert!(!map.is_peer_subscribed(&uuid, vec_1));
    }

    #[test]
    fn world_subscriptions() {
        let uuid_1 = Uuid::new_v4();
        let uuid_2 = Uuid::new_v4();

        let cube_1 = CubeArea::new(0, 0, 0);
        let cube_2 = CubeArea::new(16, 16, 16);
        let mut map = AreaMap::new(16, "world".into());

        // Neither are subscribed
        assert!(!map.is_peer_subscribed_any(&uuid_1));
        assert!(!map.is_peer_subscribed_any(&uuid_2));

        // Only uuid_1 is subscribed
        map.add_subscription(uuid_1, cube_1);
        assert!(map.is_peer_subscribed_any(&uuid_1));
        assert!(!map.is_peer_subscribed_any(&uuid_2));

        // Only uuid_1 is subscribed
        map.add_subscription(uuid_1, cube_2);
        assert!(map.is_peer_subscribed_any(&uuid_1));
        assert!(!map.is_peer_subscribed_any(&uuid_2));

        // Both are subscribed
        map.add_subscription(uuid_2, cube_2);
        assert!(map.is_peer_subscribed_any(&uuid_1));
        assert!(map.is_peer_subscribed_any(&uuid_2));

        // Both are subscribed
        map.remove_subscription(&uuid_1, cube_1);
        assert!(map.is_peer_subscribed_any(&uuid_1));
        assert!(map.is_peer_subscribed_any(&uuid_2));

        // Only uuid_2 is subscribed
        map.remove_subscription(&uuid_1, cube_2);
        assert!(!map.is_peer_subscribed_any(&uuid_1));
        assert!(map.is_peer_subscribed_any(&uuid_2));

        // Only uuid_2 is subscribed
        map.add_subscription(uuid_2, cube_1);
        assert!(!map.is_peer_subscribed_any(&uuid_1));
        assert!(map.is_peer_subscribed_any(&uuid_2));

        // Neither are subscribed
        map.remove_peer(&uuid_2);
        assert!(!map.is_peer_subscribed_any(&uuid_1));
        assert!(!map.is_peer_subscribed_any(&uuid_2));
    }
}
