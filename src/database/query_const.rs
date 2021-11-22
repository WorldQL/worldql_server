// region: Init
pub(super) const CREATE_TABLE_NAVIGATION: &str = "
CREATE TABLE IF NOT EXISTS table_navigation
(
   min_x        integer,
   max_x        integer,
   min_y        integer,
   max_y        integer,
   min_z        integer,
   max_z        integer,
   world_name   varchar(32),
   table_suffix serial
);
";

pub(super) const CREATE_REGION_NAVIGATION: &str = "
CREATE TABLE IF NOT EXISTS region_navigation
(
   min_x      integer,
   max_x      integer,
   min_y      integer,
   max_y      integer,
   min_z      integer,
   max_z      integer,
   world_name varchar(32),
   region_id  serial
);
";
// endregion
