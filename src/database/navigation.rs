use tokio_postgres::Error;
use tracing::trace;

use super::world_region::WorldRegion;
use super::{
    DatabaseClient, INSERT_REGION_ID, INSERT_TABLE_SUFFIX, LOOKUP_REGION_ID, LOOKUP_TABLE_SUFFIX,
};
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
        trace!("looking up table_suffix for {}", region);

        // Early return for cached value
        if let Some(id) = self.table_cache.get(region) {
            trace!("region {} has cached table_suffix = {}", region, id);
            return Ok(*id);
        }

        // Query database for table_suffix
        trace!("querying database for {} table_suffix", region);
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
                trace!(
                    "table_suffix for {} returned from db = {}",
                    region,
                    &table_suffix
                );

                table_suffix
            }

            // No suffix found, create and return
            None => {
                trace!("table_suffix for {} not found in db, creating", region);

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
                trace!(
                    "table_suffix for {} returned from db = {}",
                    region,
                    &table_suffix
                );

                table_suffix
            }
        };

        // Insert into cache and return
        self.table_cache.put(region.clone(), table_suffix);
        Ok(table_suffix)
    }

    async fn get_region_id(&mut self, region: &WorldRegion) -> Result<i32, Error> {
        trace!("looking up region_id for {}", region);

        // Early return for cached value
        if let Some(id) = self.region_cache.get(region) {
            trace!("region {} has cached region_id = {}", region, id);
            return Ok(*id);
        }

        // Query database for region_id
        trace!("querying database for {} region_id", region);
        let rows = self
            .client
            .query(
                LOOKUP_REGION_ID,
                &[region.world_name(), region.x(), region.y(), region.z()],
            )
            .await?;

        let region_id = match rows.get(0) {
            // ID found, return
            Some(row) => {
                let region_id: i32 = row.try_get("region_id")?;
                trace!("region_id for {} returned from db = {}", region, &region_id);

                region_id
            }

            // No suffix found, create and return
            None => {
                trace!("region_id for {} not found in db, creating", region);

                let min_x = *region.x();
                let min_y = *region.y();
                let min_z = *region.z();

                let max_x = min_x + i64::from(self.region_x_size());
                let max_y = min_y + i64::from(self.region_y_size());
                let max_z = min_z + i64::from(self.region_z_size());

                // Insert new values into DB
                let row = self
                    .client
                    .query_one(
                        INSERT_REGION_ID,
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

                let region_id: i32 = row.try_get("region_id")?;
                trace!("region_id for {} returned from db = {}", region, &region_id);

                region_id
            }
        };

        // Insert into cache and return
        self.region_cache.put(region.clone(), region_id);
        Ok(region_id)
    }
}
