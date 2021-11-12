use std::collections::HashMap;

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
    pub fn get(&self, world_name: impl Into<String>) -> Option<&AreaMap> {
        self.map.get(&world_name.into())
    }

    /// Gets a mutable [`AreaMap`] for the given world name.
    pub fn get_mut(&mut self, world_name: impl Into<String>) -> &mut AreaMap {
        self.map
            .entry(world_name.into())
            .or_insert(AreaMap::new(self.cube_size))
    }
}
