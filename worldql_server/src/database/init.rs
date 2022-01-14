use color_eyre::Result;

use super::client::DatabaseClient;
use super::{
    CREATE_REGION_NAVIGATION, CREATE_SCHEMA_NAVIGATION, CREATE_TABLE_NAVIGATION,
    CREATE_TABLE_NAVIGATION_INDEX,
};

impl DatabaseClient {
    pub async fn init_database(&self) -> Result<()> {
        let queries = vec![
            // Create schema
            CREATE_SCHEMA_NAVIGATION,
            // Create tables
            CREATE_TABLE_NAVIGATION,
            CREATE_REGION_NAVIGATION,
            // Create index
            CREATE_TABLE_NAVIGATION_INDEX,
        ];

        // Execute
        let query = format!("{};", queries.join(";"));
        self.client.batch_execute(&query).await?;

        Ok(())
    }
}
