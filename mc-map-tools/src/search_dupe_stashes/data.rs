use std::{collections::HashMap, fmt::Display};

use qutree::Point;

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

impl From<Position> for Point<i32> {
    fn from(pos: Position) -> Self {
        (pos.x, pos.z).into()
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
        Position, PotentialStashLocation, PotentialStashLocations,
        PotentialStashLocationsByGroup,
    };
    use qutree::Point;
    use test_case::test_case;

    #[test_case(Position { x: 0, y: 0, z: 0 } => Point::from((0, 0)) )]
    #[test_case(Position { x: 2, y: 0, z: 4 } => Point::from((2, 4) ))]
    fn position_to_point(position: Position) -> Point<i32> {
        position.into()
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
