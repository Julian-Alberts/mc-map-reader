use async_std::sync::RwLock;
use std::collections::VecDeque;
use std::path::Path;
use std::sync::Arc;
use std::{collections::HashMap, fmt::Display};

use crate::file::region_inventories::RegionInventories;
use crate::file::FileItemRead;
use qutee::Point;

pub struct RegionInventoryCache<'a> {
    regions: RwLock<VecDeque<RegionInventoryCacheItem>>,
    cache_size: usize,
    base_dir: &'a Path,
}

struct RegionInventoryCacheItem {
    x: i32,
    z: i32,
    inventories: Arc<RegionInventories>,
}

#[derive(Debug)]
pub struct FoundInventory<'a> {
    pub inventory_type: String,
    pub position: Position,
    pub items: HashMap<&'a str, FoundItem>,
}

#[derive(Debug)]
pub struct FoundItem {
    pub count: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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

impl<'a> RegionInventoryCache<'a> {
    pub fn new(base_dir: &'a Path, cache_size: usize) -> Self {
        Self {
            cache_size,
            regions: Default::default(),
            base_dir,
        }
    }

    pub async fn get(&self, x: i32, z: i32) -> std::io::Result<Arc<RegionInventories>> {
        use async_std::fs::File;
        let regions_lock = self.regions.read().await;
        let region = regions_lock.iter().find(|reg| reg.x == x && reg.z == z);

        if let Some(reg) = region {
            return Ok(reg.inventories.clone());
        }

        drop(regions_lock);

        let region = self.base_dir.join(format!("region_{x}_{z}.mtri"));
        let mut file = File::open(region).await?;

        let inventories = Arc::new(RegionInventories::read(&mut file).await?);
        let mut regions_lock = self.regions.write().await;
        regions_lock.push_back(RegionInventoryCacheItem {
            x,
            z,
            inventories: Arc::clone(&inventories),
        });

        if regions_lock.len() > self.cache_size {
            regions_lock.pop_front();
        }

        Ok(inventories)
    }
}

impl From<Position> for Point<i32> {
    fn from(pos: Position) -> Self {
        (pos.x, pos.z).into()
    }
}
impl From<&Position> for Point<i32> {
    fn from(pos: &Position) -> Self {
        (pos.x, pos.z).into()
    }
}

impl qutee::AsPoint<i32> for &FoundInventory<'_> {
    fn as_point(&self) -> Point<i32> {
        Point::from(&self.position)
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
        Position, PotentialStashLocation, PotentialStashLocations, PotentialStashLocationsByGroup,
    };
    use qutee::Point;
    use test_case::test_case;

    #[test_case(Position { x: 0, y: 0, z: 0 } => Point::from((0, 0)) )]
    #[test_case(Position { x: 2, y: 0, z: 4 } => Point::from((2, 4) ))]
    fn position_to_point(position: Position) -> Point<i32> {
        position.into()
    }

    #[test]
    fn test_display_potential_stash_locations() {
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

    mod cache {
        use super::super::RegionInventoryCache;
        use std::path::PathBuf;

        #[test]
        fn find() {
            let _cache = RegionInventoryCache::new(PathBuf::from("").as_path(), 5);
        }
    }
}
