use tokio_postgres::Error;

use super::world_region::WorldRegion;
use super::DatabaseClient;
use crate::database::{INSERT_TABLE_SUFFIX, LOOKUP_TABLE_SUFFIX};
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
        // TODO: Add traces

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
                let table_suffix: i32 = row.try_get("table_suffix")?;
                table_suffix
            }

            // No suffix found, create and return
            None => {
                let table_size = i64::from(self.table_size());
                let (min_x, max_x) = region.x_bounds(table_size);
                let (min_y, max_y) = region.y_bounds(table_size);
                let (min_z, max_z) = region.z_bounds(table_size);

                // Insert new values into DB
                let row = self
                    .client
                    .query_one(
                        INSERT_TABLE_SUFFIX,
                        &[
                            &min_x,
                            &max_x,
                            &min_y,
                            &max_y,
                            &min_z,
                            &max_z,
                            region.world_name(),
                        ],
                    )
                    .await?;

                let table_suffix: i32 = row.try_get("table_suffix")?;
                table_suffix
            }
        };

        // Insert into cache and return
        self.table_cache.insert(region.clone(), table_suffix);
        Ok(table_suffix)
    }

    async fn get_region_id(&mut self, region: &WorldRegion) -> Result<i32, Error> {
        // Early return for cached value
        if let Some(id) = self.region_cache.get(region) {
            return Ok(*id);
        }

        todo!()
    }
}
