use tokio_postgres::Client;

pub struct DatabaseClient {
    pub(super) client: Client,
}

impl DatabaseClient {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}
