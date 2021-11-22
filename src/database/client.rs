use color_eyre::Result;
use tokio_postgres::Client;

use crate::structures::{Record, Vector3};

pub struct DatabaseClient {
    pub(super) client: Client,
}

impl DatabaseClient {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn create_record(record: Record) -> Result<()> {
        todo!()
    }

    pub async fn records_in_region(world_name: &str, point_inside_region: Vector3) -> Result<()> {
        todo!()
    }
}
