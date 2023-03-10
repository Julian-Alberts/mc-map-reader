use std::collections::HashMap;

use crate::quadtree::{Bounded, Bounds};

use super::config::Item;

#[derive(Debug)]
pub struct FoundInventory<'a> {
    pub inventory_type: String,
    pub x: i32,
    pub z: i32,
    pub items: HashMap<usize, FoundItem<'a>>,
}

#[derive(Debug)]
pub struct FoundItem<'a> {
    pub item_config: &'a Item,
    pub count: i16,
}

impl <'a> Bounded for FoundInventory<'a> {
    fn bounds(&self) -> Bounds {
        Bounds {
            x: self.x as f32,
            y: self.z as f32,
            width: 1.,
            height: 1.,
        }
    }
}
