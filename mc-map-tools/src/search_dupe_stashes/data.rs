use std::{collections::HashMap, fmt::Display};

use crate::quadtree::{Bounded, Bounds};

#[derive(Debug)]
pub struct FoundInventory<'a> {
    pub inventory_type: String,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub items: HashMap<String, FoundItem<'a>>,
}

#[derive(Debug)]
pub struct FoundItem<'a> {
    pub group_key: &'a String,
    pub count: i16,
    pub position: Position,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

pub struct PotentialStashLocation {
    pub count: usize,
    pub position: Position,
}

pub struct PotentialStashLocationsByGroup<'a> {
    pub group_key: &'a String,
    pub locations: Vec<PotentialStashLocation>,
}

pub struct PotentialStashLocations<'a>(pub Vec<PotentialStashLocationsByGroup<'a>>);

impl Bounded for FoundItem<'_> {
    fn bounds(&self) -> Bounds {
        self.position.into()
    }
}

impl From<Position> for Bounds {
    fn from(pos: Position) -> Self {
        Bounds {
            x: pos.x as f32,
            y: pos.z as f32,
            width: 1.,
            height: 1.,
        }
    }
}

impl Display for PotentialStashLocations<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for group in &self.0 {
            writeln!(f, "Group: {}", group.group_key)?;
            for location in &group.locations {
                writeln!(
                    f,
                    "  Count: {}, Position: {:?}",
                    location.count, location.position
                )?;
            }
        }
        Ok(())
    }
}
