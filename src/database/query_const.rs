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
pub(super) const LOOKUP_TABLE_SUFFIX: &str = "
    SELECT table_suffix FROM table_navigation
    WHERE world_name = $1 AND
    $2 >= min_x AND $2 < max_x AND
    $3 >= min_y AND $3 < max_y AND
    $4 >= min_z AND $4 < max_z
";

pub(super) const LOOKUP_REGION_ID: &str = "
    SELECT region_id FROM region_navigation
    WHERE world_name = $1 AND
    $2 >= min_x AND $2 < max_x AND
    $3 >= min_y AND $3 < max_y AND
    $4 >= min_z AND $4 < max_z
";
// endregion

// region: Inserts
// TODO
pub(super) const INSERT_TABLE_SUFFIX: &str = "
    INSERT INTO table_navigation (min_x, max_x, min_y, max_y, min_z, max_z, world_name)
    VALUES ($1, $2, $3, $4, $5, $6, $7)
    RETURNING table_suffix
";

// TODO
pub(super) const INSERT_REGION_ID: &str = "
    INSERT INTO region_navigation (min_x, max_x, min_y, max_y, min_z, max_z, world_name)
    VALUES ($1, $2, $3, $4, $5, $6, $7)
    RETURNING region_id
";
// endregion
