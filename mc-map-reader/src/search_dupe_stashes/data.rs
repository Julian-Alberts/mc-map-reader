use std::collections::HashMap;

use crate::quadtree::{Bounded, Bounds};

#[derive(Debug)]
pub struct FoundInventory {
    pub inventory_type: String,
    pub x: i32,
    pub z: i32,
    pub items: HashMap<String, FoundItem>,
}

#[derive(Debug)]
pub struct FoundItem {
    pub id: String,
    pub count: i16,
}

impl Bounded for FoundInventory {
    fn bounds(&self) -> Bounds {
        Bounds {
            x: self.x as f32,
            y: self.z as f32,
            width: 1.,
            height: 1.,
        }
    }
}
