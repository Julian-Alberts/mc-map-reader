use std::collections::HashMap;

use jbe::Builder;

use crate::nbt::{Array, List, Tag};

use super::player_dat::Player;
use crate::data::dimension::Dimension;
pub use crate::data::load::file_format::level_dat::*;

/// https://minecraft.fandom.com/wiki/Java_Edition_level_format#level.dat_format
#[derive(Debug, Builder, PartialEq)]
pub struct LevelDat {
    pub allow_commands: bool,
    pub border_center_x: f64,
    pub border_center_z: f64,
    pub border_damage_per_block: f64,
    pub border_safe_zone: f64,
    pub border_size: f64,
    pub border_size_lerp_target: f64,
    pub border_size_lerp_time: i64,
    pub border_warning_blocks: f64,
    pub border_warning_time: f64,
    pub clear_weather_time: i32,
    pub custom_boss_events: HashMap<String, CustomBossEvent>,
    pub data_packs: DataPacks,
    pub data_version: i32,
    pub day_time: i64,
    pub difficulty: i8,
    pub difficulty_locked: bool,
    pub dimension_data: Option<HashMap<String, HashMap<String, Tag>>>,
    pub game_rules: HashMap<String, String>,
    pub world_gen_settings: WorldGenSettings,
    pub game_type: i32,
    pub generator_name: Option<String>,
    pub generator_options: Option<HashMap<String, Tag>>,
    pub generator_version: Option<i32>,
    pub hardcore: bool,
    pub initialized: bool,
    pub last_played: i64,
    pub level_name: String,
    #[builder({default: true})]
    pub map_features: bool,
    pub player: Option<Player>,
    pub raining: bool,
    pub rain_time: i32,
    /// Not used
    pub random_seed: Option<i64>,
    /// Not used
    pub size_on_disk: Option<i64>,
    pub spawn_x: i32,
    pub spawn_y: i32,
    pub spawn_z: i32,
    pub thundering: bool,
    pub thunder_time: i32,
    pub time: i64,
    pub version: i32,
    pub version_info: Version,
    pub wandering_trader_id: Array<i32>,
    pub wandering_trader_spawn_chance: i32,
    pub wandering_trader_spawn_delay: i32,
    pub was_modded: bool,
}

/// https://minecraft.fandom.com/wiki/Java_Edition_level_format#level.dat_format
#[derive(Debug, Builder, PartialEq)]
pub struct CustomBossEvent {
    pub players: List<Array<i32>>,
    pub color: String,
    pub create_world_fog: bool,
    pub darken_screen: bool,
    pub max: i32,
    pub value: i32,
    pub name: String,
    pub overlay: String,
    pub play_boss_music: bool,
    pub visible: bool,
}

/// https://minecraft.fandom.com/wiki/Java_Edition_level_format#level.dat_format
#[derive(Debug, Builder, PartialEq)]
pub struct DataPacks {
    pub disabled: List<String>,
    pub enabled: List<String>,
}

/// https://minecraft.fandom.com/wiki/Java_Edition_level_format#level.dat_format
#[derive(Debug, Builder, PartialEq)]
pub struct WorldGenSettings {
    pub bonus_chest: bool,
    pub seed: i64,
    pub generate_features: bool,
    pub dimensions: HashMap<String, Dimension>,
}

#[derive(Debug, Builder, PartialEq)]
pub struct Version {
    pub id: i32,
    pub name: String,
    pub series: String,
    pub snapshot: bool,
}
