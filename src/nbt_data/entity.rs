use std::collections::HashMap;

use getset::{CopyGetters, Getters};
use jbe::Builder;

use crate::nbt::{Array, List, Tag};

///<a href="https://minecraft.fandom.com/wiki/Entity_format#Entity_Format">minecraft wiki</a>
///
#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct Entity {
    air: i16,
    custom_name: Option<String>,
    custom_name_visible: bool,
    fall_distance: f32,
    fire: i16,
    glowing: bool,
    has_visual_fire: bool,
    id: Option<String>,
    invulnerable: bool,
    motion: List<f64>,
    no_gravity: bool,
    on_ground: bool,
    passengers: List<Entity>,
    portal_colldown: i32,
    pos: List<f32>,
    rotation: List<f32>,
    silent: bool,
    tags: Option<HashMap<String, Tag>>,
    ticks_frozen: Option<i32>,
    uuid: Array<i32>,
}
