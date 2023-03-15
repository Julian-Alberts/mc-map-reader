use std::collections::HashMap;

use getset::{CopyGetters, Getters};
use jbe::Builder;

use crate::nbt::{Array, List, Tag};

use super::block_entity::Item;

///<a href="https://minecraft.fandom.com/wiki/Entity_format#Entity_Format">minecraft wiki</a>
///
#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct Entity {
    air: Option<i16>,
    custom_name: Option<String>,
    custom_name_visible: Option<bool>,
    fall_distance: Option<f32>,
    #[get_copy = "pub"]
    #[builder({default 0})]
    fire: i16,
    #[get_copy = "pub"]
    #[builder({default false})]
    glowing: bool,
    #[get_copy = "pub"]
    #[builder({default false})]
    has_visual_fire: bool,
    id: Option<String>,
    #[get_copy = "pub"]
    #[builder({default false})]
    invulnerable: bool,
    motion: Option<List<f64>>,
    #[get_copy = "pub"]
    #[builder({default false})]
    no_gravity: bool,
    #[get_copy = "pub"]
    #[builder({default true})]
    on_ground: bool,
    passengers: Option<List<Entity>>,
    #[get_copy = "pub"]
    #[builder({default 0})]
    portal_colldown: i32,
    pos: Option<List<f32>>,
    rotation: Option<List<f32>>,
    #[get_copy = "pub"]
    #[builder({default false})]
    silent: bool,
    tags: Option<HashMap<String, Tag>>,
    ticks_frozen: Option<i32>,
    uuid: Option<Array<i32>>,
}

#[derive(Debug, Builder)]
pub struct Mob {
    absorption_amount: Option<f32>,
    active_effects: Option<List<ActiveEffect>>,
    armor_drop_chances: Option<List<f32>>,
    armor_items: Option<List<Item>>,
    entity: Entity,
    attributes: Option<List<HashMap<String, Tag>>>,
    brain: Option<HashMap<String, Tag>>,
    can_pick_up_loot: Option<bool>,
    death_loot_table: Option<String>,
    death_loot_table_seed: Option<i64>,
    death_time: Option<i16>,
    fall_flying: Option<bool>,
    health: Option<f32>,
    hurt_by_timestamp: Option<i32>,
    hurt_time: Option<i16>,
    hand_drop_chances: Option<List<f32>>,
    hand_items: Option<List<Item>>,
    leash: Option<Leash>,
    left_handed: Option<bool>,
    no_ai: Option<bool>,
    persistence_required: Option<bool>,
    sleeping_x: Option<i32>,
    sleeping_y: Option<i32>,
    sleeping_z: Option<i32>,
    team: Option<String>,
}

#[derive(Debug)]
pub enum Leash {
    Entity(Array<i32>),
    Position { x: i32, y: i32, z: i32 },
}
#[derive(Debug, Builder)]
pub struct ActiveEffect {
    ambient: bool,
    amplifier: i8,
    duration: i32,
    id: i32,
    show_icon: bool,
    show_particles: bool,
}

impl Entity {
    pub fn custom_name(&self) -> Option<&String> {
        self.custom_name.as_ref()
    }

    pub fn id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    pub fn tags(&self) -> Option<&HashMap<String, Tag>> {
        self.tags.as_ref()
    }

    pub fn ticks_frozen(&self) -> Option<i32> {
        self.ticks_frozen
    }

    pub fn air(&self) -> Option<i16> {
        self.air
    }

    pub fn custom_name_visible(&self) -> Option<bool> {
        self.custom_name_visible
    }

    pub fn fall_distance(&self) -> Option<f32> {
        self.fall_distance
    }

    pub fn motion(&self) -> Option<&List<f64>> {
        self.motion.as_ref()
    }

    pub fn passengers(&self) -> Option<&List<Entity>> {
        self.passengers.as_ref()
    }

    pub fn pos(&self) -> Option<&List<f32>> {
        self.pos.as_ref()
    }

    pub fn rotation(&self) -> Option<&List<f32>> {
        self.rotation.as_ref()
    }

    pub fn uuid(&self) -> Option<&Array<i32>> {
        self.uuid.as_ref()
    }
}
