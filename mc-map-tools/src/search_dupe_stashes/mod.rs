pub mod args;
pub mod config;
mod data;

use data::*;
use rayon::prelude::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::{collections::HashMap, fs::OpenOptions, path::Path, sync::Mutex};

use mc_map_reader::{
    nbt_data::{
        block_entity::{BlockEntity, BlockEntityType, InventoryBlock, Item, ShulkerBox},
        chunk::ChunkData,
    },
    LoadMcSave,
};

use crate::{
    config::Config,
    quadtree::{Bounds, QuadTree},
    read_file,
};

use self::config::SearchDupeStashesConfig;

pub fn main(world_dir: &Path, data: args::SearchDupeStashes, config: Config) {
    let region_groups = if let Some(area) = data.area {
        mc_map_reader::files::get_region_files_in_area(
            world_dir, None, area.x1, area.z1, area.x2, area.z2,
        )
    } else {
        mc_map_reader::files::get_region_files(world_dir, None)
            .expect("Could not read region directory")
    };
    let config = config.search_dupe_stashes;
    let inventories = region_groups
        .into_par_iter()
        .map(|region| OpenOptions::new().read(true).open(region).unwrap())
        .map(read_file)
        .map(Result::unwrap)
        .map(|data| mc_map_reader::Loader.load_from_bytes(&data[..]).unwrap())
        .map(|region| {
            region
                .chunks()
                .iter()
                .filter_map(|c| search_inventories_in_chunk(c, &config))
                .fold(Vec::default(), |mut invnentories, mut new| {
                    invnentories.append(&mut new);
                    invnentories
                })
        })
        .collect::<Vec<_>>()
        .into_iter()
        .fold(Vec::default(), |mut all, mut new| {
            all.append(&mut new);
            all
        });

    dbg!(&inventories);
    if inventories.is_empty() {
        return;
    }

    let (x1, z1, x2, z2) = inventories.iter().fold(
        (i32::MAX, i32::MAX, i32::MIN, i32::MIN),
        |(mut x1, mut z1, mut x2, mut z2), inv| {
            x1 = x1.min(inv.x);
            z1 = z1.min(inv.z);
            x2 = x2.max(inv.x);
            z2 = z2.max(inv.z);
            (x1, z1, x2, z2)
        },
    );
    assert!(x1 <= x2 && z1 <= z2, "{x1} <= {x2} && {z1} <= {z2}");
    let x_direction = x2 - x1;
    let z_direction = z2 - z1;
    assert!(x_direction >= 0 && z_direction >= 0);
    let bounds = Bounds {
        height: z_direction as f32,
        width: x_direction as f32,
        x: x1 as f32,
        y: z1 as f32,
    };
    let mut inventory_trees = HashMap::new();
    for group in &config.groups {
        inventory_trees.insert(&group.name, Mutex::new(QuadTree::new(bounds.clone())));
    }

    inventories.par_iter().for_each(|inv| {
        inv.items.iter().for_each(|(group_key, item)| {
            let tree = inventory_trees.get(group_key).unwrap();
            let mut tree = tree.lock().unwrap();
            tree.insert(item);
        });
    });
    let inventory_trees = inventory_trees
        .into_iter()
        .map(|(k, v)| (k, v.into_inner().unwrap()))
        .collect::<HashMap<_, _>>();
    let item_stashes = inventory_trees
        .into_par_iter()
        .map(|(group_key, items)| {
            let group = config.groups.iter().find(|g| &g.name == group_key).unwrap();
            let threshold = group.threshold;
            let counts: Vec<_> = items
                .iter()
                .map(|item| {
                    let pos = item.position;
                    let radius = data.radius;
                    let count = count_items_in_area(radius, pos.x, pos.z, &items);
                    PotentialStashLocation {
                        position: pos,
                        count,
                    }
                })
                .filter(|location| location.count >= threshold)
                .collect();
            if counts.is_empty() {
                return None;
            }
            Some(PotentialStashLocationsByGroup {
                group_key,
                locations: counts,
            })
        })
        .filter_map(|x| x)
        .collect::<Vec<_>>();
    let item_stashes = PotentialStashLocations(item_stashes);
    println!("{item_stashes}")
}

fn search_inventories_in_chunk<'a, 'b>(
    chunk: &ChunkData,
    config: &'b SearchDupeStashesConfig,
) -> Option<Vec<FoundInventory<'a>>>
where
    'b: 'a,
{
    let Some(block_entities) = chunk.block_entities() else {
        return None
    };
    let res = block_entities
        .iter()
        .filter_map(|block_entity| {
            let inventory: &dyn InventoryBlock = match block_entity.entity_type() {
                BlockEntityType::Barrel(block) => block,
                BlockEntityType::Chest(block) => block,
                BlockEntityType::Dispenser(block) => block,
                BlockEntityType::Dropper(block) => block,
                BlockEntityType::Hopper(block) => block,
                BlockEntityType::ShulkerBox(block) => block,
                BlockEntityType::TrappedChest(block) => block,
                _ => return None,
            };
            search_inventory_block(inventory, block_entity, config)
        })
        .collect::<Vec<_>>();
    if res.is_empty() {
        None
    } else {
        Some(res)
    }
}

fn search_inventory_block<'a, 'b>(
    inventory: &dyn InventoryBlock,
    base_entity: &BlockEntity,
    config: &'b SearchDupeStashesConfig,
) -> Option<FoundInventory<'a>>
where
    'b: 'a,
{
    if inventory.loot_table().is_some() || inventory.loot_table_seed().is_some() {
        return None;
    }
    let x = base_entity.x();
    let z = base_entity.z();
    let y = base_entity.y();
    let items = if let Some(items) = inventory.items() {
        items.iter().fold(HashMap::default(), |mut item_map, item| {
            add_item_to_map(item, &mut item_map, config, x, y, z);
            if item_is_shulker_box(item.item().id()) {
                search_subinventory(item.item(), &mut item_map, config, x, y, z)
            }
            item_map
        })
    } else {
        return None;
    };
    Some(FoundInventory {
        inventory_type: base_entity.id().clone(),
        items,
        x,
        y,
        z,
    })
}

#[inline]
fn item_is_shulker_box(id: &str) -> bool {
    id.starts_with("minecraft:") && id.ends_with("shulker_box")
}

fn search_subinventory<'a, 'b>(
    item: &Item,
    item_map: &mut HashMap<&'a String, FoundItem<'a>>,
    config: &'b SearchDupeStashesConfig,
    x: i32,
    y: i32,
    z: i32,
) where
    'b: 'a,
{
    let Some (tag) = item.tag() else {
        return;
    };
    let Some (block_entity_tag) = tag.get("BlockEntityTag").cloned() else {
        return;
    };
    let Ok(inventory) = ShulkerBox::try_from(block_entity_tag) else {
        return;
    };
    if let Some(items) = inventory.items() {
        items
            .iter()
            .for_each(|item| add_item_to_map(item, item_map, config, x, y, z))
    }
}

fn add_item_to_map<'a, 'b>(
    item: &mc_map_reader::nbt_data::block_entity::ItemWithSlot,
    item_map: &mut HashMap<&'a String, FoundItem<'a>>,
    config: &'b SearchDupeStashesConfig,
    x: i32,
    y: i32,
    z: i32,
) where
    'b: 'a,
{
    let item = item.item();
    config
        .groups
        .iter()
        .filter(|group| group.matches(item))
        .for_each(|group| {
            let mult = group
                .items
                .iter()
                .find(|i| i.matches(item))
                .map(|i| i.multiplier)
                .unwrap_or(1);
            item_map
                .entry(&group.name)
                .and_modify(|item_entry: &mut FoundItem| {
                    item_entry.count += item.count() as usize * mult;
                })
                .or_insert_with(|| FoundItem {
                    group_key: &group.name,
                    position: Position { x, y, z },
                    count: item.count() as usize * mult,
                });
        });
}

fn count_items_in_area(radius: u32, x: i32, z: i32, inventories: &QuadTree<FoundItem>) -> usize {
    let radius_f32 = radius as f32;
    let area = Bounds {
        x: x as f32 - radius_f32,
        y: z as f32 - radius_f32,
        width: (radius * 2) as f32,
        height: (radius * 2) as f32,
    };

    inventories.query(&area).map(|i| i.count).sum()
}
