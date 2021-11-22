use crate::structures::Vector3;

use super::DatabaseClient;

impl DatabaseClient {
    pub(super) async fn get_table_for_point(&mut self, world_name: &str, point_inside_region: Vector3) -> i32 {
        todo!()
    }

    pub(super) async fn get_region_id_for_point(&mut self, world_name: &str, point_inside_region: Vector3) -> i32 {
        todo!()
    }
}
