//! Common structs used by multiple message types

mod record;
mod replication;
mod vector3;

pub use record::{PartialRecord, Record};
pub use replication::Replication;
pub use vector3::Vector3;
