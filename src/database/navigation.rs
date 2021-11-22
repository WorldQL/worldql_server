use super::world_region::WorldRegion;
use super::DatabaseClient;
use crate::structures::Vector3;

impl DatabaseClient {
    /// Lookup both `table_id` and `region_id` in a single function.
    ///
    /// Returned tuple has the form `(table_id, region_id)`
    pub(super) async fn lookup_ids(&mut self, world_name: &str, point: Vector3) -> (i32, i32) {
        let world_region = self.world_region(world_name, point);
        let table_id = self.get_table_id(&world_region).await;
        let region_id = self.get_region_id(&world_region).await;

        (table_id, region_id)
    }

    async fn get_table_id(&mut self, region: &WorldRegion) -> i32 {
        // Early return for cached value
        if let Some(id) = self.table_cache.get(region) {
            return *id;
        }

        todo!()
    }

    async fn get_region_id(
        &mut self,
        region: &WorldRegion,
    ) -> i32 {
        // Early return for cached value
        if let Some(id) = self.region_cache.get(region) {
            return *id;
        }

        todo!()
    }
}
