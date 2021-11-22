use color_eyre::Result;
use tokio_postgres::Client;

use crate::structures::{Record, Vector3};

pub struct DatabaseClient {
    pub(super) client: Client,

    region_x_size: u16,
    region_y_size: u16,
    region_z_size: u16,
    table_size: u32,
}

impl DatabaseClient {
    pub fn new(client: Client, region_x_size: u16, region_y_size: u16, region_z_size: u16, table_size: u32) -> Self {
        Self {
            client,

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

    pub async fn create_record(record: Record) -> Result<()> {
        todo!()
    }

    pub async fn records_in_region(world_name: &str, point_inside_region: Vector3) -> Result<()> {
        todo!()
    }
}
