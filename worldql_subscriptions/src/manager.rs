use uuid::Uuid;

#[derive(Debug)]
pub struct SubscriptionManager {

}

type Area = ();

impl SubscriptionManager {
    pub fn is_subscribed_world(&self, peer: Uuid, world: &str) -> bool {
        todo!()
    }

    pub fn is_subscribed_area(&self, peer: Uuid, world: &str, area: Area) -> bool {
        todo!()
    }

    pub fn get_subscribed_to_world(&self, world: &str) -> impl Iterator<Item = Uuid> + '_ {
        todo!()
    }

    pub fn get_subscribed_to_area(&self, world: &str, area: Area) -> impl Iterator<Item = Uuid> + '_ {
        todo!()
    }

    pub fn subscribe_to_world(&mut self, peer: Uuid, world: impl Into<String>) -> bool {
        todo!()
    }

    pub fn unsubscribe_from_world(&mut self, peer: Uuid, world: &str) -> bool {
        todo!()
    }

    pub fn subscribe_to_area(&mut self, peer: Uuid, world: impl Into<String>, area: Area) -> bool {
        todo!()
    }

    pub fn unsubscribe_from_area(&mut self, peer: Uuid, world: &str, area: Area) -> bool {
        todo!()
    }

    pub fn remove_peer(&mut self, peer: Uuid) -> bool {
        todo!()
    }
}
