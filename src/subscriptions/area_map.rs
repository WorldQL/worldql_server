use std::collections::{HashMap, HashSet};

use uuid::Uuid;

use super::CubeArea;
use crate::structures::Vector3;

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

    pub fn is_peer_subscribed(&self, uuid: &Uuid, cube: Vector3) -> bool {
        let cube = CubeArea::from_vector3(cube, self.cube_size);
        let entry = self.map.get(&cube);

        match entry {
            None => false,
            Some(set) => set.contains(uuid)
        }
    }

    pub fn get_subscribed_peers(&self, cube: Vector3) -> Vec<&Uuid> {
        let cube = CubeArea::from_vector3(cube, self.cube_size);
        let entry = self.map.get(&cube);

        match entry {
            None => vec![],
            Some(set) => set.iter().collect::<Vec<_>>(),
        }
    }

    /// If the subscription was added, `true` is returned.
    ///
    /// If the subscription was already present, `false` is returned
    pub fn add_subscription(&mut self, uuid: Uuid, cube: Vector3) -> bool {
        let cube = CubeArea::from_vector3(cube, self.cube_size);
        let entry = self.map.entry(cube).or_insert(Default::default());

        entry.insert(uuid)
    }

    /// Returns whether the value was registered.
    pub fn remove_subscription(&mut self, uuid: &Uuid, cube: Vector3) -> bool {
        let cube = CubeArea::from_vector3(cube, self.cube_size);
        let entry = self.map.entry(cube).or_insert(Default::default());

        entry.remove(uuid)
    }
}
