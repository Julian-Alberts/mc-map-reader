use std::collections::HashMap;

use getset::{CopyGetters, Getters};
use jbe::Builder;

use crate::nbt::Tag;

///<a href="https://minecraft.fandom.com/wiki/Entity_format#Entity_Format">minecraft wiki</a>
///
#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct Entity {
    air: i16,
    custom_name: Option<String>,
    custom_name_visible: bool,
    fall_distance: f32,
    fire: i16,
    glowing: bool,
    has_visual_fire: bool,
    id: String,
    invulnerable: bool,
    motion: Vec<f64>,
    no_gravity: bool,
    passengers: Vec<Entity>,
    portal_colldown: i32,
    pos: Vec<f32>,
    rotation: Vec<f32>,
    silent: bool,
    tags: Option<HashMap<String, Tag>>,
    ticks_frozen: i32,
    uuid: Vec<i32>,
}
