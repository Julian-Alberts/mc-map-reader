use std::collections::HashMap;

use getset::{CopyGetters, Getters};
use jbe::Builder;

use crate::nbt::{Array, List, Tag};

///<a href="https://minecraft.fandom.com/wiki/Entity_format#Entity_Format">minecraft wiki</a>
///
#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct Entity {
    #[get_copy = "pub"]
    air: i16,
    custom_name: Option<String>,
    #[get_copy = "pub"]
    custom_name_visible: bool,
    #[get_copy = "pub"]
    fall_distance: f32,
    #[get_copy = "pub"]
    fire: i16,
    #[get_copy = "pub"]
    glowing: bool,
    #[get_copy = "pub"]
    has_visual_fire: bool,
    id: Option<String>,    
    #[get_copy = "pub"]
    invulnerable: bool,
    #[get = "pub"]
    motion: List<f64>,
    #[get_copy = "pub"]
    no_gravity: bool,
    #[get_copy = "pub"]
    on_ground: bool,
    #[get = "pub"]
    passengers: List<Entity>,
    #[get_copy = "pub"]
    portal_colldown: i32,
    #[get = "pub"]
    pos: List<f32>,
    #[get = "pub"]
    rotation: List<f32>,
    #[get_copy = "pub"]
    silent: bool,
    tags: Option<HashMap<String, Tag>>,
    ticks_frozen: Option<i32>,
    #[get = "pub"]
    uuid: Array<i32>,
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
}
