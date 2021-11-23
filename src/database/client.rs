use color_eyre::Result;
use lru::LruCache;
use thiserror::Error;
use tokio_postgres::error::SqlState;
use tokio_postgres::Client;

use super::world_region::WorldRegion;
use crate::database::{
    query_create_world, query_create_world_index, query_insert_record, query_select_records,
};
use crate::structures::{Record, Vector3};
use crate::utils::{sanitize_world_name, SanitizeError};

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
    /// Insert a single [`Record`] into the database.
    pub async fn insert_record(&mut self, record: Record) -> Result<(), DatabaseError> {
        // TODO: Handle records without position
        let position = record.position.unwrap();
        let world_name = sanitize_world_name(&record.world_name)?;

        let (table_suffix, region_id) = self.lookup_ids(&world_name, &position).await?;
        let query = query_insert_record(&world_name, table_suffix);

        let result = self
            .client
            .execute(
                &query,
                &[
                    &region_id,
                    position.x(),
                    position.y(),
                    position.z(),
                    &record.uuid,
                    &record.data,
                    &record.flex.as_ref().map(|b| b.to_vec()),
                ],
            )
            .await;

        // Insertion completed without errors, exit early
        if result.is_ok() {
            return Ok(());
        }

        // Handle SQL Error
        let error = result.unwrap_err();
        let db_error = error.as_db_error();

        // If error isn't a database error, re-throw
        if db_error.is_none() {
            return Err(DatabaseError::PostgresError(error));
        }

        // Check for undefined table error, if not then re-throw
        let db_error = db_error.unwrap();
        if *db_error.code() != SqlState::UNDEFINED_TABLE {
            return Err(DatabaseError::PostgresError(error));
        }

        // Create table for world region
        self.client
            .execute(&query_create_world(&world_name, table_suffix), &[])
            .await?;

        // Create index for new table
        self.client
            .execute(&query_create_world_index(&world_name, table_suffix), &[])
            .await?;

        // Retry insertion
        self.client
            .execute(
                &query,
                &[
                    &region_id,
                    position.x(),
                    position.y(),
                    position.z(),
                    &record.uuid,
                    &record.data,
                    &record.flex.as_ref().map(|b| b.to_vec()),
                ],
            )
            .await?;

        Ok(())
    }

    /// Returns a [`Vec`] containing all records found within the region represented
    /// by `point_inside_region`
    pub async fn get_records_in_region(
        &mut self,
        world_name: &str,
        point_inside_region: Vector3,
    ) -> Result<Vec<Record>> {
        let (table_suffix, region_id) = self.lookup_ids(world_name, &point_inside_region).await?;

        let query = query_select_records(world_name, table_suffix);
        let records = self
            .client
            .query(&query, &[&region_id])
            .await?
            .into_iter()
            .map(|row| Record::from_postgres_row(row, world_name))
            .collect::<Vec<Record>>();

        Ok(records)
    }
    // endregion
}

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("world name error: {0}")]
    InvalidWorldName(#[from] SanitizeError),

    #[error(transparent)]
    PostgresError(#[from] tokio_postgres::Error),
}
