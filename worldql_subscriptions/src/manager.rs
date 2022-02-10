use uuid::Uuid;

use crate::Area;

#[derive(Debug)]
pub struct SubscriptionManager {}

impl SubscriptionManager {
    #[must_use]
    pub fn is_subscribed_world(&self, peer: Uuid, world: &str) -> bool {
        todo!()
    }

    #[must_use]
    pub fn is_subscribed_area(&self, peer: Uuid, world: &str, area: Area) -> bool {
        todo!()
    }

    // #[must_use]
    // pub fn get_subscribed_to_world(&self, world: &str) -> impl Iterator<Item = Uuid> + '_ {
    //     todo!()
    // }

    // #[must_use]
    // pub fn get_subscribed_to_area(&self, world: &str, area: Area) -> impl Iterator<Item = Uuid> + '_ {
    //     todo!()
    // }

    pub fn subscribe_to_world(&mut self, peer: Uuid, world: impl Into<String>) {
        todo!()
    }

    pub fn unsubscribe_from_world(&mut self, peer: Uuid, world: &str) {
        todo!()
    }

    pub fn subscribe_to_area(&mut self, peer: Uuid, world: impl Into<String>, area: Area) {
        todo!()
    }

    pub fn unsubscribe_from_area(&mut self, peer: Uuid, world: &str, area: Area) {
        todo!()
    }

    pub fn remove_peer(&mut self, peer: Uuid) -> bool {
        todo!()
    }
}
