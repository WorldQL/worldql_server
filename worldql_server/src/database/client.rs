use std::convert::Into;
use std::ops::Deref;

use color_eyre::Result;
use sea_query::{ColumnRef, Cond, Expr, PostgresQueryBuilder, Query};
use sea_query_driver_postgres::{bind_query, bind_query_as};
use sqlx::{Pool, Postgres};
use worldql_messages::client_bound::Error;
use worldql_messages::common::{PartialRecord, Record, Vector3};

use crate::database::error::ResultExt;
use crate::database::sql_record::{RecordIden, SqlRecord};

sea_query::sea_query_driver_postgres!();

pub struct DatabaseClient {
    pub(super) pool: Pool<Postgres>,
}

impl DatabaseClient {
    #[inline]
    #[must_use]
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    // region: Methods
    pub async fn get_records_by_area(
        &self,
        world_name: &str,
        pos_1: Vector3,
        pos_2: Vector3,
    ) -> Result<Vec<Record>, Error> {
        let [x_1, y_1, z_1]: [f64; 3] = pos_1.into();
        let [x_2, y_2, z_2]: [f64; 3] = pos_2.into();

        let x_min = f64::min(x_1, x_2);
        let y_min = f64::min(y_1, y_2);
        let z_min = f64::min(z_1, z_2);

        let x_max = f64::max(x_1, x_2);
        let y_max = f64::max(y_1, y_2);
        let z_max = f64::max(z_1, z_2);

        let (sql, values) = Query::select()
            .column(ColumnRef::Asterisk)
            .from(RecordIden::Table)
            .and_where(Expr::col(RecordIden::WorldName).eq(world_name))
            // Min Bounds
            .and_where(Expr::col(RecordIden::X).gte(x_min))
            .and_where(Expr::col(RecordIden::Y).gte(y_min))
            .and_where(Expr::col(RecordIden::Z).gte(z_min))
            // Max Bounds
            .and_where(Expr::col(RecordIden::X).lt(x_max))
            .and_where(Expr::col(RecordIden::Y).lt(y_max))
            .and_where(Expr::col(RecordIden::Z).lt(z_max))
            .build(PostgresQueryBuilder);

        let rows = bind_query_as(sqlx::query_as::<_, SqlRecord>(&sql), &values)
            .fetch_all(&self.pool)
            .await
            .or_client_err()?
            .into_iter()
            .map(Into::into)
            .collect();

        Ok(rows)
    }

    pub async fn get_records_by_id(
        &self,
        records: Vec<PartialRecord>,
    ) -> Result<Vec<Record>, Error> {
        let (sql, values) = {
            let mut builder = Query::select();
            builder.column(ColumnRef::Asterisk);
            builder.from(RecordIden::Table);

            let mut cond = Cond::any();
            for PartialRecord { uuid, world_name } in records {
                cond = cond.add(
                    Expr::col(RecordIden::Uuid)
                        .eq(uuid)
                        .and(Expr::col(RecordIden::WorldName).eq(world_name)),
                );
            }

            builder.cond_where(cond);
            builder.build(PostgresQueryBuilder)
        };

        let rows = bind_query_as(sqlx::query_as::<_, SqlRecord>(&sql), &values)
            .fetch_all(&self.pool)
            .await
            .or_client_err()?
            .into_iter()
            .map(Into::into)
            .collect();

        Ok(rows)
    }

    pub async fn set_records(&self, records: Vec<Record>) -> Result<(u32, u32), Error> {
        // Delete
        let deleted = {
            let (sql, values) = {
                let mut builder = Query::delete();
                builder.from_table(RecordIden::Table);

                let mut cond = Cond::any();
                for record in &records {
                    let Record {
                        uuid,
                        world_name,
                        position: _,
                        data: _,
                    } = record;

                    cond = cond.add(
                        Expr::col(RecordIden::Uuid)
                            .eq(*uuid)
                            .and(Expr::col(RecordIden::WorldName).eq(world_name.deref())),
                    );
                }

                builder.cond_where(cond);
                builder.build(PostgresQueryBuilder)
            };

            bind_query(sqlx::query(&sql), &values)
                .execute(&self.pool)
                .await
                .or_client_err()?
                .rows_affected()
        };

        let inserted = {
            let (sql, values) = {
                let mut builder = Query::insert();
                builder.into_table(RecordIden::Table);
                builder.columns(vec![
                    RecordIden::Uuid,
                    RecordIden::WorldName,
                    RecordIden::X,
                    RecordIden::Y,
                    RecordIden::Z,
                    RecordIden::Data,
                ]);

                let records = records.into_iter().map(Into::into);
                for record in records {
                    let SqlRecord {
                        uuid,
                        world_name,
                        x,
                        y,
                        z,
                        data,
                    } = record;

                    builder.values_panic(vec![
                        uuid.into(),
                        world_name.into(),
                        x.into(),
                        y.into(),
                        z.into(),
                        data.into(),
                    ]);
                }

                builder.build(PostgresQueryBuilder)
            };

            bind_query(sqlx::query(&sql), &values)
                .execute(&self.pool)
                .await
                .or_client_err()?
                .rows_affected()
        };

        let created = (inserted - deleted) as u32;
        let updated = deleted as u32;

        Ok((created, updated))
    }

    pub async fn delete_records(&self, records: Vec<PartialRecord>) -> Result<u32, Error> {
        todo!()
    }

    pub async fn clear_records_in_world(&self, world_name: &str) -> Result<u32, Error> {
        todo!()
    }

    pub async fn clear_records_in_area(
        &self,
        world_name: &str,
        pos_1: Vector3,
        pos_2: Vector3,
    ) -> Result<u32, Error> {
        todo!()
    }
    // endregion
}
