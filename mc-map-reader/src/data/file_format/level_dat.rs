use std::collections::HashMap;

use jbe::Builder;

use crate::nbt::{List, Tag, Array};

use super::player_dat::Player;
use crate::data::dimension::Dimension;


/// https://minecraft.fandom.com/wiki/Java_Edition_level_format#level.dat_format
#[derive(Debug, Builder)]
pub struct LevelDat {
    allow_commands: bool,
    border_center_x: f64,
    border_center_z: f64,
    border_damage_per_block: f64,
    border_safe_zone: f64,
    border_size: f64,
    border_size_lerp_target: f64,
    border_size_lerp_time: i64,
    border_warning_blocks: f64,
    border_warning_time: i64,
    clear_weather_time: i64,
    custom_boss_events: Vec<HashMap<String, CustomBossEvent>>,
    data_packs: DataPacks,
    data_version: i32,
    day_time: i64,
    difficulty: i8,
    difficulty_locked: bool,
    dimension_data: HashMap<String, HashMap<String, Tag>>,
    game_rules: HashMap<String, String>,
    world_gen_settings: WorldGenSettings,
    game_type: i32,
    generator_name: String,
    generator_options: HashMap<String, Tag>,
    generator_version: i32,
    hardcore: bool,
    initialized: bool,
    last_played: i64,
    level_name: String,
    map_features: bool,
    player: Player,
    raining: bool,
    rain_time: i32,
    random_seed: i64,
    size_on_disk: i64,
    spawn_x: i32,
    spawn_y: i32,
    spawn_z: i32,
    thundering: bool,
    thunder_time: i32,
    time: i64,
    version: i32,
    version_info: Version,
    wandering_trader_id: Array<i32>,
    wandering_trader_spawn_chance: i32,
    wandering_trader_spawn_delay: i32,
    was_modded: bool,
}

/// https://minecraft.fandom.com/wiki/Java_Edition_level_format#level.dat_format
#[derive(Debug, Builder)]
pub struct CustomBossEvent {
    players: List<Array<i32>>,
    color: String,
    create_world_fog: bool,
    darken_screen: bool,
    max: i32,
    value: i32,
    name: String,
    overlay: String,
    play_boss_music: bool,
    visible: bool,
}

/// https://minecraft.fandom.com/wiki/Java_Edition_level_format#level.dat_format
#[derive(Debug, Builder)]
pub struct DataPacks {
    disabled: List<String>,
    enabled: List<String>,
}

/// https://minecraft.fandom.com/wiki/Java_Edition_level_format#level.dat_format
#[derive(Debug, Builder)]
pub struct WorldGenSettings {
    bonus_chest: bool,
    seed: i64,
    generate_features: bool,
    dimensions: HashMap<String, Dimension>,
}

#[derive(Debug, Builder)]
pub struct Version {
    id: i32,
    name: String,
    series: String,
    snapshot: bool,
}
