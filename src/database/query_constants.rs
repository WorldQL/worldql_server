// region: Init
pub(super) const CREATE_TABLE_NAVIGATION: &str = "
    CREATE TABLE IF NOT EXISTS table_navigation
    (
        min_x        bigint,
        max_x        bigint,
        min_y        bigint,
        max_y        bigint,
        min_z        bigint,
        max_z        bigint,
        world_name   varchar(32),
        table_suffix serial
    );
";

pub(super) const CREATE_TABLE_NAVIGATION_INDEX: &str = "
    CREATE UNIQUE INDEX IF NOT EXISTS
    table_navigation_table_suffix_uindex
    ON table_navigation (table_suffix)
";

pub(super) const CREATE_REGION_NAVIGATION: &str = "
    CREATE TABLE IF NOT EXISTS region_navigation
    (
        min_x      bigint,
        max_x      bigint,
        min_y      bigint,
        max_y      bigint,
        min_z      bigint,
        max_z      bigint,
        world_name varchar(32),
        region_id  serial
    );
";
// endregion

// region: Lookups
pub(super) const QUERY_LOOKUP_TABLE_SUFFIX: &str = "
    SELECT table_suffix FROM table_navigation
    WHERE world_name = $1 AND
    $2 >= min_x AND $2 < max_x AND
    $3 >= min_y AND $3 < max_y AND
    $4 >= min_z AND $4 < max_z
";

pub(super) const QUERY_INSERT_TABLE_SUFFIX: &str = "
    INSERT INTO table_navigation (min_x, max_x, min_y, max_y, min_z, max_z, world_name)
    VALUES ($1, $2, $3, $4, $5, $6, $7)
    RETURNING table_suffix
";

pub(super) const QUERY_LOOKUP_REGION_ID: &str = "
    SELECT region_id FROM region_navigation
    WHERE world_name = $1 AND
    $2 >= min_x AND $2 < max_x AND
    $3 >= min_y AND $3 < max_y AND
    $4 >= min_z AND $4 < max_z
";

pub(super) const QUERY_INSERT_REGION_ID: &str = "
    INSERT INTO region_navigation (min_x, max_x, min_y, max_y, min_z, max_z, world_name)
    VALUES ($1, $2, $3, $4, $5, $6, $7)
    RETURNING region_id
";
// endregion

// region: Create World Table
pub(super) fn query_create_world(world_name: &str, suffix: i32) -> String {
    let query = format!(
        "
        CREATE TABLE {}_{}
        (
            last_modified timestamp NOT NULL DEFAULT NOW(),
            region_id     integer,
            x             double precision,
            y             double precision,
            z             double precision,
            uuid          uuid,
            data          varchar,
            flex          bytea
        )
        ",
        world_name, suffix
    );

    query
}

pub(super) fn query_create_world_index(world_name: &str, suffix: i32) -> String {
    let query = format!(
        "
        CREATE INDEX {0}_{1}_region_id_index
        ON {0}_{1} USING btree (region_id);
        ",
        world_name, suffix
    );

    query
}
// endregion

// region: Record Manipulation
pub(super) fn query_insert_record(world_name: &str, suffix: i32) -> String {
    let query = format!(
        "
        INSERT INTO {0}_{1}
        (region_id, x, y, z, uuid, data, flex)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        ",
        world_name, suffix
    );

    query
}

pub(super) fn query_insert_record_many(world_name: &str, suffix: i32, count: usize) -> String {
    let mut query = format!(
        "
        INSERT INTO {0}_{1}
        (region_id, x, y, z, uuid, data, flex)
        VALUES",
        world_name, suffix
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

#[cfg(test)]
mod tests {
    #[test]
    fn abc() {
        let q = super::query_insert_record_many("world", 1, 2);
        dbg!(q);
    }
}

pub(super) fn query_select_records(world_name: &str, suffix: i32) -> String {
    let query = format!(
        "
        SELECT x, y, z, uuid, data, flex FROM {0}_{1}
        WHERE region_id = $1
        ",
        world_name, suffix
    );

    query
}
// endregion
