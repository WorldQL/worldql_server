use std::collections::{HashMap, HashSet};

use uuid::Uuid;

use crate::structures::Vector3;

struct AreaMap {
    cube_size: u16,
    map: HashMap<Vector3, HashSet<Uuid>>
}

impl AreaMap {
    pub fn new(cube_size: u16) -> Self {
        Self {
            cube_size,
            map: HashMap::new(),
        }
    }

    pub fn is_peer_subscribed(&self, uuid: Uuid, cube: Vector3) -> bool {
        // TODO
        todo!()
    }

    pub fn get_subscribed_peers(&self, cube: Vector3) -> Vec<Uuid> {
        // TODO
        todo!()
    }

    pub fn add_subscription(&mut self, uuid: Uuid, cube: Vector3) {
        // TODO
        todo!()
    }

    pub fn remove_subscription(&mut self, uuid: Uuid, cube: Vector3) {
        // TODO
        todo!()
    }
}
