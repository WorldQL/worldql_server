use std::collections::HashMap;

use tracing::trace;
use uuid::Uuid;

use super::AreaMap;

pub struct WorldMap {
    cube_size: u16,
    map: HashMap<String, AreaMap>,
}

impl WorldMap {
    pub fn new(cube_size: u16) -> Self {
        Self {
            cube_size,
            map: HashMap::new(),
        }
    }

    /// Gets an [`AreaMap`] for the given world name.
    pub fn get(&self, world_name: &str) -> Option<&AreaMap> {
        self.map.get(world_name)
    }

    /// Gets a mutable [`AreaMap`] for the given world name.
    pub fn get_mut(&mut self, world_name: &str) -> &mut AreaMap {
        self.map.entry(world_name.to_string()).or_insert_with(|| {
            trace!("creating new world: {}", world_name);
            AreaMap::new(self.cube_size, world_name.to_string())
        })
    }

    /// Completely removes a [`crate::transport::Peer`] from the map.
    ///
    /// Used in the event of a disconnect.
    pub fn remove_peer(&mut self, uuid: &Uuid) -> bool {
        let mut removed = false;
        for (_, peers) in &mut self.map {
            if peers.remove_peer(uuid) {
                removed = true;
            }
        }

        removed
    }
}
