use bytes::Bytes;
use sea_query::Iden;
use uuid::Uuid;
use worldql_messages::common::{Record, Vector3};

#[derive(Iden)]
pub(super) enum RecordIden {
    #[iden(rename = "records")]
    Table,
    Uuid,
    WorldName,
    X,
    Y,
    Z,
    Data,
}

#[derive(Debug, sqlx::FromRow)]
pub(super) struct SqlRecord {
    pub uuid: Uuid,
    pub world_name: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub data: Option<Vec<u8>>,
}

// region: Conversion
impl From<Record> for SqlRecord {
    fn from(record: Record) -> Self {
        let Record {
            uuid,
            world_name,
            position,
            data,
        } = record;

        let [x, y, z]: [f64; 3] = position.into();
        let data = data.map(|bytes| bytes.to_vec());

        Self {
            uuid,
            world_name,
            x,
            y,
            z,
            data,
        }
    }
}

impl From<SqlRecord> for Record {
    fn from(record: SqlRecord) -> Self {
        let SqlRecord {
            uuid,
            world_name,
            x,
            y,
            z,
            data,
        } = record;

        let position = Vector3::new(x, y, z);
        let data = data.map(Bytes::from);

        Self {
            uuid,
            world_name,
            position,
            data,
        }
    }
}
// endregion
