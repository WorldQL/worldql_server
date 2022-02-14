use color_eyre::Result;
use lru::LruCache;
use tokio_postgres::Client;
use worldql_messages::common::{PartialRecord, Record, Vector3};
use worldql_messages::client_bound::Error;

use super::world_region::WorldRegion;

pub struct DatabaseClient {
    pub(super) client: Client,
    pub(super) table_cache: LruCache<WorldRegion, i32>,
    pub(super) region_cache: LruCache<WorldRegion, i32>,

    region_x_size: u16,
    region_y_size: u16,
    region_z_size: u16,
    table_size: u32,
}

impl DatabaseClient {
    pub fn new(
        client: Client,
        region_x_size: u16,
        region_y_size: u16,
        region_z_size: u16,
        table_size: u32,
        cache_size: usize,
    ) -> Self {
        let (table_cache, region_cache) = if cache_size == 0 {
            (LruCache::unbounded(), LruCache::unbounded())
        } else {
            (LruCache::new(cache_size), LruCache::new(cache_size))
        };

        Self {
            client,
            table_cache,
            region_cache,

            region_x_size,
            region_y_size,
            region_z_size,
            table_size,
        }
    }

    // region: Getters
    #[inline]
    pub(super) fn region_x_size(&self) -> u16 {
        self.region_x_size
    }

    #[inline]
    pub(super) fn region_y_size(&self) -> u16 {
        self.region_y_size
    }

    #[inline]
    pub(super) fn region_z_size(&self) -> u16 {
        self.region_z_size
    }

    #[inline]
    pub(super) fn table_size(&self) -> u32 {
        self.table_size
    }
    // endregion

    // region: Methods
    pub async fn get_records_by_area(
        world_name: &str,
        position: Vector3,
    ) -> Result<Vec<Record>, Error> {
        todo!()
    }

    pub async fn get_records_by_id(
        records: Vec<PartialRecord>,
    ) -> Result<Vec<Record>, Error> {
        todo!()
    }

    pub async fn set_records(records: Vec<Record>) -> Result<(u32, u32), Error> {
        todo!()
    }

    pub async fn delete_records(records: Vec<PartialRecord>) -> Result<u32, Error> {
        todo!()
    }

    pub async fn clear_records_in_world(world_name: &str) -> Result<u32, Error> {
        todo!()
    }

    pub async fn clear_records_in_area(
        world_name: &str,
        position: Vector3,
    ) -> Result<u32, Error> {
        todo!()
    }
    // endregion
}
