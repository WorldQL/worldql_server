use tokio_postgres::Error;

use super::world_region::WorldRegion;
use super::DatabaseClient;
use crate::database::LOOKUP_TABLE_SUFFIX;
use crate::structures::Vector3;

impl DatabaseClient {
    /// Lookup both `table_id` and `region_id` in a single function.
    ///
    /// Returned tuple has the form `(table_id, region_id)`
    pub(super) async fn lookup_ids(
        &mut self,
        world_name: &str,
        point: Vector3,
    ) -> Result<(i32, i32), Error> {
        let world_region = self.world_region(world_name, point);
        let table_id = self.get_table_suffix(&world_region).await?;
        let region_id = self.get_region_id(&world_region).await?;

        Ok((table_id, region_id))
    }

    async fn get_table_suffix(&mut self, region: &WorldRegion) -> Result<i32, Error> {
        // Early return for cached value
        if let Some(id) = self.table_cache.get(region) {
            return Ok(*id);
        }

        // Query database for table_suffix
        let rows = self
            .client
            .query(
                LOOKUP_TABLE_SUFFIX,
                &[region.world_name(), region.x(), region.y(), region.z()],
            )
            .await?;

        let table_suffix = match rows.get(0) {
            // Suffix found, return
            Some(row) => {
                // TODO: Return table_suffix
                todo!()
            }

            // No suffix found, create and return
            None => {
                // TODO:
                todo!()
            }
        };

        // Insert into cache and return
        self.table_cache.insert(region.clone(), table_suffix);
        return Ok(table_suffix)
    }

    async fn get_region_id(&mut self, region: &WorldRegion) -> Result<i32, Error> {
        // Early return for cached value
        if let Some(id) = self.region_cache.get(region) {
            return Ok(*id);
        }

        todo!()
    }
}
