use std::collections::{HashMap, HashSet};

use uuid::Uuid;

use super::{CubeArea, ToCubeArea};

pub struct AreaMap {
    cube_size: u16,
    map: HashMap<CubeArea, HashSet<Uuid>>,
}

impl AreaMap {
    pub fn new(cube_size: u16) -> Self {
        Self {
            cube_size,
            map: HashMap::new(),
        }
    }

    pub fn is_peer_subscribed(&self, uuid: &Uuid, cube: impl ToCubeArea) -> bool {
        let cube = cube.to_cube_area(self.cube_size);
        let entry = self.map.get(&cube);

        match entry {
            None => false,
            Some(set) => set.contains(uuid),
        }
    }

    pub fn get_subscribed_peers(&self, cube: impl ToCubeArea) -> Vec<&Uuid> {
        let cube = cube.to_cube_area(self.cube_size);
        let entry = self.map.get(&cube);

        match entry {
            None => vec![],
            Some(set) => set.iter().collect::<Vec<_>>(),
        }
    }

    /// If the subscription was added, `true` is returned.
    ///
    /// If the subscription was already present, `false` is returned
    pub fn add_subscription(&mut self, uuid: Uuid, cube: impl ToCubeArea) -> bool {
        let cube = cube.to_cube_area(self.cube_size);
        let entry = self.map.entry(cube).or_insert(Default::default());

        entry.insert(uuid)
    }

    /// Returns whether the value was registered.
    pub fn remove_subscription(&mut self, uuid: &Uuid, cube: impl ToCubeArea) -> bool {
        let cube = cube.to_cube_area(self.cube_size);
        let entry = self.map.entry(cube).or_insert(Default::default());

        // Remove from HashSet
        let removed = entry.remove(uuid);

        // Remove HashSet from HashMap if empty
        if entry.len() == 0 {
            self.map.remove(&cube);
        }

        removed
    }
}
