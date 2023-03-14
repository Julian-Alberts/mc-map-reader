use jbe::Builder;

// https://minecraft.fandom.com/wiki/Custom_dimension
#[derive(Debug, Builder)]
pub struct Dimension {
    ultrawarm: bool,
    natural: bool,
    coordinate_scale: f64,
    has_skylight: bool,
    has_ceiling: bool,
    ambient_light: f32,
    fixed_time: i64,
    // TODO monster_spawn_light_level
    monster_spawn_block_light_limit: i32,
    piglin_safe: bool,
    bed_works: bool,
    respawn_anchor_works: bool,
    has_raids: bool,
    logical_height: i32,
    min_y: i32,
    height: i32,
    infiniburn: String,
    effects: Option<String>,
}