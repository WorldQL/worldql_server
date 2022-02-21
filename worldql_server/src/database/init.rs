use once_cell::sync::Lazy;
use sea_query::{ColumnDef, Index, PostgresQueryBuilder, Table};

use super::sql_record::RecordIden;
use super::DatabaseClient;

static CREATE_TABLE_RECORDS: Lazy<String> = Lazy::new(|| {
    Table::create()
        .table(RecordIden::Table)
        .if_not_exists()
        .col(
            ColumnDef::new(RecordIden::Uuid)
                .uuid()
                .not_null()
                .primary_key(),
        )
        .col(
            ColumnDef::new(RecordIden::WorldName)
                .not_null()
                .string_len(64),
        )
        .col(ColumnDef::new(RecordIden::X).not_null().double())
        .col(ColumnDef::new(RecordIden::Y).not_null().double())
        .col(ColumnDef::new(RecordIden::Z).not_null().double())
        .col(ColumnDef::new(RecordIden::Data).binary())
        .build(PostgresQueryBuilder)
});

macro_rules! create_index {
    ($name: ident, $coord: tt, $idx: expr) => {
        static $name: Lazy<String> = Lazy::new(|| {
            Index::create()
                .table(RecordIden::Table)
                .col(RecordIden::$coord)
                .name($idx)
                .build(PostgresQueryBuilder)
        });
    };
}

create_index!(CREATE_IDX_WORLD, WorldName, "records_world_name");
create_index!(CREATE_IDX_X_COORD, X, "records_x_coord");
create_index!(CREATE_IDX_Y_COORD, Y, "records_y_coord");
create_index!(CREATE_IDX_Z_COORD, Z, "records_z_coord");

impl DatabaseClient {
    pub async fn init(&self) -> Result<(), sqlx::Error> {
        sqlx::query(&CREATE_TABLE_RECORDS)
            .execute(&self.pool)
            .await?;

        // TODO: Only ignore duplicate index errors
        let _ = sqlx::query(&CREATE_IDX_WORLD).execute(&self.pool).await;
        let _ = sqlx::query(&CREATE_IDX_X_COORD).execute(&self.pool).await;
        let _ = sqlx::query(&CREATE_IDX_Y_COORD).execute(&self.pool).await;
        let _ = sqlx::query(&CREATE_IDX_Z_COORD).execute(&self.pool).await;

        Ok(())
    }
}
