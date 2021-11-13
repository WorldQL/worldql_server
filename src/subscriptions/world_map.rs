use std::collections::HashMap;
use std::fmt::Display;

use tracing::trace;
use uuid::Uuid;

use super::AreaMap;

#[derive(Debug)]
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
        for area_map in self.map.values_mut() {
            if area_map.remove_peer(uuid) {
                removed = true;
            }
        }

        removed
    }
}

impl Display for WorldMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;

        let len = self.map.len();
        for (i, area_map) in self.map.values().enumerate() {
            write!(f, "{}", area_map)?;
            if i + 1 != len {
                write!(f, ", ")?;
            }
        }

        write!(f, "]")
    }
}
