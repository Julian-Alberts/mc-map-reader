use jbe::Builder;

// https://minecraft.fandom.com/wiki/Custom_dimension
#[derive(Debug, Builder)]
pub struct Dimension {
    pub ultrawarm: bool,
    pub natural: bool,
    pub coordinate_scale: f64,
    pub has_skylight: bool,
    pub has_ceiling: bool,
    pub ambient_light: f32,
    pub fixed_time: i64,
    // TODO monster_spawn_light_level
    pub monster_spawn_block_light_limit: i32,
    pub piglin_safe: bool,
    pub bed_works: bool,
    pub respawn_anchor_works: bool,
    pub has_raids: bool,
    pub logical_height: i32,
    pub min_y: i32,
    pub height: i32,
    pub infiniburn: String,
    pub effects: Option<String>,
}
