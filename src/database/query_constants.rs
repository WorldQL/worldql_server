// region: Init
pub(super) const CREATE_SCHEMA_NAVIGATION: &str = "
    CREATE SCHEMA IF NOT EXISTS navigation
";

pub(super) const CREATE_TABLE_NAVIGATION: &str = "
    CREATE TABLE IF NOT EXISTS navigation.tables
    (
        min_x        bigint NOT NULL,
        max_x        bigint NOT NULL,
        min_y        bigint NOT NULL,
        max_y        bigint NOT NULL,
        min_z        bigint NOT NULL,
        max_z        bigint NOT NULL,
        world_name   varchar(32) NOT NULL,
        table_suffix serial NOT NULL
    );
";

pub(super) const CREATE_TABLE_NAVIGATION_INDEX: &str = "
    CREATE UNIQUE INDEX IF NOT EXISTS
    table_navigation_table_suffix_uindex
    ON navigation.tables (table_suffix)
";

pub(super) const CREATE_REGION_NAVIGATION: &str = "
    CREATE TABLE IF NOT EXISTS navigation.regions
    (
        min_x      bigint NOT NULL,
        max_x      bigint NOT NULL,
        min_y      bigint NOT NULL,
        max_y      bigint NOT NULL,
        min_z      bigint NOT NULL,
        max_z      bigint NOT NULL,
        world_name varchar(32) NOT NULL,
        region_id  serial NOT NULL
    );
";
// endregion

// region: Lookups
pub(super) const QUERY_LOOKUP_TABLE_SUFFIX: &str = "
    SELECT table_suffix FROM navigation.tables
    WHERE world_name = $1 AND
    $2 >= min_x AND $2 < max_x AND
    $3 >= min_y AND $3 < max_y AND
    $4 >= min_z AND $4 < max_z
";

pub(super) const QUERY_INSERT_TABLE_SUFFIX: &str = "
    INSERT INTO navigation.tables (min_x, max_x, min_y, max_y, min_z, max_z, world_name)
    VALUES ($1, $2, $3, $4, $5, $6, $7)
    RETURNING table_suffix
";

pub(super) const QUERY_LOOKUP_REGION_ID: &str = "
    SELECT region_id FROM navigation.regions
    WHERE world_name = $1 AND
    $2 >= min_x AND $2 < max_x AND
    $3 >= min_y AND $3 < max_y AND
    $4 >= min_z AND $4 < max_z
";

pub(super) const QUERY_INSERT_REGION_ID: &str = "
    INSERT INTO navigation.regions (min_x, max_x, min_y, max_y, min_z, max_z, world_name)
    VALUES ($1, $2, $3, $4, $5, $6, $7)
    RETURNING region_id
";
// endregion

// region: Create World Table
pub(super) fn query_create_world_schema(world_name: &str) -> String {
    let query = format!(
        "
        CREATE SCHEMA IF NOT EXISTS w_{}
        ",
        world_name
    );

    query
}

fn table_name(world_name: &str, suffix: i32) -> String {
    format!("w_{0}.r_{1}", world_name, suffix)
}

pub(super) fn query_create_world(world_name: &str, suffix: i32) -> String {
    let query = format!(
        "
        CREATE TABLE {}
        (
            last_modified timestamp NOT NULL DEFAULT NOW(),
            region_id     integer NOT NULL,
            x             double precision,
            y             double precision,
            z             double precision,
            uuid          uuid NOT NULL,
            data          varchar,
            flex          bytea
        )
        ",
        table_name(world_name, suffix)
    );

    query
}

pub(super) fn query_create_world_index(world_name: &str, suffix: i32) -> String {
    let query = format!(
        "
        CREATE INDEX {0}_{1}_region_id_index
        ON {2} USING btree (region_id);
        ",
        world_name,
        suffix,
        table_name(world_name, suffix)
    );

    query
}
// endregion

// region: Record Manipulation
pub(super) fn query_insert_record(world_name: &str, suffix: i32) -> String {
    let query = format!(
        "
        INSERT INTO {}
        (region_id, x, y, z, uuid, data, flex)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        ",
        table_name(world_name, suffix)
    );

    query
}

pub(super) fn query_insert_record_many(world_name: &str, suffix: i32, count: usize) -> String {
    let mut query = format!(
        "
        INSERT INTO {}
        (region_id, x, y, z, uuid, data, flex)
        VALUES",
        table_name(world_name, suffix)
    );

    for i in 0..count {
        let i = i * 7;
        let prefix = if i == 0 { " " } else { ", " };

        query += &format!(
            "{}(${}, ${}, ${}, ${}, ${}, ${}, ${})",
            prefix,
            i + 1,
            i + 2,
            i + 3,
            i + 4,
            i + 5,
            i + 6,
            i + 7
        );
    }

    query
}

pub(super) fn query_select_records(world_name: &str, suffix: i32) -> String {
    let query = format!(
        "
        SELECT last_modified, x, y, z, uuid, data, flex
        FROM {} WHERE region_id = $1
        ",
        table_name(world_name, suffix)
    );

    query
}

pub(super) fn query_delete_record(world_name: &str, suffix: i32) -> String {
    let query = format!(
        "
        DELETE FROM {} WHERE
        region_id = $1 AND uuid = $2
        ",
        table_name(world_name, suffix)
    );

    query
}

pub(super) fn query_delete_duplictes(world_name: &str, suffix: i32) -> String {
    let query = format!(
        "
        DELETE FROM {} WHERE
        uuid = $1 AND last_modified < $2
        ",
        table_name(world_name, suffix)
    );

    query
}
// endregion
