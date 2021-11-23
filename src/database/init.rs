use color_eyre::Result;
use futures_util::future;

use super::client::DatabaseClient;
use super::{CREATE_REGION_NAVIGATION, CREATE_TABLE_NAVIGATION, CREATE_TABLE_NAVIGATION_INDEX};

impl DatabaseClient {
    pub async fn init_database(&self) -> Result<()> {
        // Create tables
        future::try_join(
            self.client.prepare(CREATE_TABLE_NAVIGATION),
            self.client.prepare(CREATE_REGION_NAVIGATION),
        )
        .await?;

        // Create index
        self.client
            .execute(CREATE_TABLE_NAVIGATION_INDEX, &[])
            .await?;

        Ok(())
    }
}
