use color_eyre::Result;
use sqlx::{Pool, Postgres};
use worldql_messages::client_bound::Error;
use worldql_messages::common::{PartialRecord, Record, Vector3};

pub struct DatabaseClient {
    pub(super) pool: Pool<Postgres>,
}

impl DatabaseClient {
    #[inline]
    #[must_use]
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    // region: Methods
    pub async fn get_records_by_area(
        &mut self,
        world_name: &str,
        position: Vector3,
    ) -> Result<Vec<Record>, Error> {
        todo!()
    }

    pub async fn get_records_by_id(
        &mut self,
        records: Vec<PartialRecord>,
    ) -> Result<Vec<Record>, Error> {
        todo!()
    }

    pub async fn set_records(&mut self, records: Vec<Record>) -> Result<(u32, u32), Error> {
        todo!()
    }

    pub async fn delete_records(&mut self, records: Vec<PartialRecord>) -> Result<u32, Error> {
        todo!()
    }

    pub async fn clear_records_in_world(&mut self, world_name: &str) -> Result<u32, Error> {
        todo!()
    }

    pub async fn clear_records_in_area(
        &mut self,
        world_name: &str,
        position: Vector3,
    ) -> Result<u32, Error> {
        todo!()
    }
    // endregion
}
