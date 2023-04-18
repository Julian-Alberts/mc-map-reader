use std::{collections::HashMap, fmt::Display};

use crate::quadtree::{Bounded, Bounds};

#[derive(Debug)]
pub struct FoundInventory<'a> {
    pub inventory_type: String,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub items: HashMap<&'a String, FoundItem<'a>>,
}

#[derive(Debug)]
pub struct FoundItem<'a> {
    pub group_key: &'a str,
    pub count: usize,
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
    pub group_key: &'a str,
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

#[cfg(test)]
mod tests {
    use super::{
        FoundItem, Position, PotentialStashLocation, PotentialStashLocations,
        PotentialStashLocationsByGroup,
    };
    use crate::quadtree::{Bounded, Bounds};
    use test_case::test_case;

    #[test_case(FoundItem { group_key: "test", count: 1, position: Position { x: 0, y: 0, z: 0 } } => Bounds { x: 0., y: 0., width: 1., height: 1. })]
    #[test_case(FoundItem { group_key: "test", count: 1, position: Position { x: 2, y: 0, z: 4 } } => Bounds { x: 2., y: 4., width: 1., height: 1. })]
    fn test_found_item_to_bounds(item: FoundItem) -> Bounds {
        item.bounds()
    }

    #[test]
    fn test_display_potentiol_stash_locations() {
        let locations = PotentialStashLocations(vec![
            PotentialStashLocationsByGroup {
                group_key: "key1",
                locations: vec![
                    PotentialStashLocation {
                        count: 1,
                        position: Position { x: 0, y: 0, z: 0 },
                    },
                    PotentialStashLocation {
                        count: 2,
                        position: Position { x: 1, y: 0, z: 1 },
                    },
                ],
            },
            PotentialStashLocationsByGroup {
                group_key: "key2",
                locations: vec![
                    PotentialStashLocation {
                        count: 3,
                        position: Position { x: 2, y: 0, z: 2 },
                    },
                    PotentialStashLocation {
                        count: 4,
                        position: Position { x: 3, y: 0, z: 3 },
                    },
                ],
            },
        ]);
        let string = format!("{}", locations);
        assert_eq!(
            string,
            r##"Group: key1
  Count: 1, Position: Position { x: 0, y: 0, z: 0 }
  Count: 2, Position: Position { x: 1, y: 0, z: 1 }
Group: key2
  Count: 3, Position: Position { x: 2, y: 0, z: 2 }
  Count: 4, Position: Position { x: 3, y: 0, z: 3 }
"##
        )
    }
}
