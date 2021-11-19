use color_eyre::Result;
use futures_util::future;

use super::client::DatabaseClient;

const CREATE_TABLE_NAVIGATION: &str = "
CREATE TABLE IF NOT EXISTS table_navigation
(
   min_x      integer,
   max_x      integer,
   min_y      integer,
   max_y      integer,
   min_z      integer,
   max_z      integer,
   table_name varchar(32)
);
";

const CREATE_REGION_NAVIGATION: &str = "
CREATE TABLE IF NOT EXISTS region_navigation
(
   min_x     integer,
   max_x     integer,
   min_y     integer,
   max_y     integer,
   min_z     integer,
   max_z     integer,
   region_id serial
);
";

impl DatabaseClient {
    pub async fn init_database(&self) -> Result<()> {
        // Create tables
        future::try_join(
            self.client.execute(CREATE_TABLE_NAVIGATION, &[]),
            self.client.execute(CREATE_REGION_NAVIGATION, &[]),
        )
        .await?;

        Ok(())
    }
}
