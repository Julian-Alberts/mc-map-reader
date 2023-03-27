use std::collections::HashMap;

use jbe::Builder;

use crate::nbt::{Array, List, Tag};

use super::item::Item;

///<a href="https://minecraft.fandom.com/wiki/Entity_format#Entity_Format">minecraft wiki</a>
#[derive(Debug, Builder, Clone, PartialEq)]
pub struct Entity {
    pub air: Option<i16>,
    pub custom_name: Option<String>,
    pub custom_name_visible: Option<bool>,
    pub fall_distance: Option<f32>,
    #[builder({default 0})]
    pub fire: i16,
    #[builder({default false})]
    pub glowing: bool,
    #[builder({default false})]
    pub has_visual_fire: bool,
    pub id: Option<String>,
    #[builder({default false})]
    pub invulnerable: bool,
    pub motion: Option<List<f64>>,
    #[builder({default false})]
    pub no_gravity: bool,
    #[builder({default true})]
    pub on_ground: bool,
    pub passengers: Option<List<Entity>>,
    #[builder({default 0})]
    pub portal_colldown: i32,
    pub pos: Option<List<f32>>,
    pub rotation: Option<List<f32>>,
    #[builder({default false})]
    pub silent: bool,
    pub tags: Option<HashMap<String, Tag>>,
    pub ticks_frozen: Option<i32>,
    pub uuid: Option<Array<i32>>,
}

#[derive(Debug, Builder)]
pub struct Mob {
    pub absorption_amount: Option<f32>,
    pub active_effects: Option<List<ActiveEffect>>,
    pub armor_drop_chances: Option<List<f32>>,
    pub armor_items: Option<List<Item>>,
    pub entity: Entity,
    pub attributes: Option<List<HashMap<String, Tag>>>,
    pub brain: Option<HashMap<String, Tag>>,
    pub can_pick_up_loot: Option<bool>,
    pub death_loot_table: Option<String>,
    pub death_loot_table_seed: Option<i64>,
    pub death_time: Option<i16>,
    pub fall_flying: Option<bool>,
    pub health: Option<f32>,
    pub hurt_by_timestamp: Option<i32>,
    pub hurt_time: Option<i16>,
    pub hand_drop_chances: Option<List<f32>>,
    pub hand_items: Option<List<Item>>,
    pub leash: Option<Leash>,
    pub left_handed: Option<bool>,
    pub no_ai: Option<bool>,
    pub persistence_required: Option<bool>,
    pub sleeping_x: Option<i32>,
    pub sleeping_y: Option<i32>,
    pub sleeping_z: Option<i32>,
    pub team: Option<String>,
}

#[derive(Debug, PartialEq)]
pub enum Leash {
    Entity(Array<i32>),
    Position { x: i32, y: i32, z: i32 },
}
#[derive(Debug, Builder, PartialEq)]
pub struct ActiveEffect {
    pub ambient: bool,
    pub amplifier: i8,
    pub duration: i32,
    pub id: i32,
    pub show_icon: bool,
    pub show_particles: bool,
}
