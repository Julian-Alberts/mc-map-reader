pub mod args;
pub mod config;
mod data;
mod detection_method;

use data::*;
use qutee::{Boundary, ConstCap};
#[cfg(feature = "parallel")]
use rayon::prelude::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::io::Write;
use std::{collections::HashMap, fs::OpenOptions, path::Path};

use mc_map_reader::{
    data::{
        block_entity::{BlockEntity, BlockEntityType, InventoryBlock, ShulkerBox},
        chunk::ChunkData,
        item::Item,
    },
    RegionLoadError,
};

use crate::search_dupe_stashes::detection_method::DetectionMethod;
use crate::{config::Config, read_file};

use self::config::SearchDupeStashesConfig;

type QuadTree<'a> = qutee::QuadTree<i32, &'a FoundInventory<'a>, ConstCap<32>>;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    RegionLoadError(#[from] RegionLoadError),
}

pub fn main(
    world_dir: &Path,
    data: args::SearchDupeStashes,
    config: Config,
    writer: &mut dyn Write,
) {
    let detection_method = Box::new(detection_method::Absolute::new(
        &config.search_dupe_stashes.groups,
    ));
    let region_groups = if let Some(area) = data.area {
        mc_map_reader::files::get_region_files_in_area(
            world_dir, None, area.x1, area.z1, area.x2, area.z2,
        )
    } else {
        mc_map_reader::files::get_region_files(world_dir, None)
            .expect("Could not read region directory")
    };
    log::debug!(
        "Found {} region files {region_groups:#?}",
        region_groups.len()
    );
    let config = &config.search_dupe_stashes;
    #[cfg(feature = "parallel")]
    let region_groups = region_groups.into_par_iter();
    #[cfg(not(feature = "parallel"))]
    let region_groups = region_groups.into_iter();
    let (inventories, errors) = region_groups
        .map(|region| search_inventories_in_region(&region, config))
        .collect::<Vec<Result<_, Error>>>()
        .into_iter()
        .fold(
            (Vec::default(), Vec::default()),
            |(mut inv, mut err), new| {
                match new {
                    Ok(mut i) => inv.append(&mut i),
                    Err(e) => err.push(e),
                }
                (inv, err)
            },
        );

    for e in errors {
        log::error!("Error while reading region file {}", e);
    }
    if inventories.is_empty() {
        log::info!("No inventories found");
        return;
    }
    log::info!("Found {} inventories", inventories.len());

    let (x1, z1, x2, z2) = inventories
        .iter()
        .fold((i32::MAX, i32::MAX, i32::MIN, i32::MIN), find_corners);
    log::debug!("Bounds: ({x1}, {z1}) - ({x2}, {z2})");
    assert!(x1 <= x2 && z1 <= z2, "{x1} <= {x2} && {z1} <= {z2}");
    let bounds = Boundary::between_points((x1, z1), (x2, z2));
    let mut inventory_tree: QuadTree = QuadTree::new_with_const_cap(bounds);

    inventories
        .iter()
        .for_each(|inv| inventory_tree.insert_unchecked(inv));

    #[cfg(feature = "parallel")]
    let inventory_iter = inventories.par_iter();
    #[cfg(not(feature = "parallel"))]
    let inventory_iter = config.groups.iter();
    let potential_stash_locations = inventory_iter
        .map(|inventory| {
            collect_items_in_area(
                data.radius as i32,
                inventory,
                &inventory_tree,
                detection_method.as_ref(),
            )
        })
        .filter(|(_, i)| !i.is_empty());

    potential_stash_locations
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|(Position { x, y, z }, sl)| {
            sl.iter().for_each(|(item, count)| {
                writer
                    .write_all(format!("{x},{y},{z},{item},{count}").as_bytes())
                    .expect("Error writing message");
            })
        });
}

fn collect_items_in_area<'a>(
    radius: i32,
    inventory: &FoundInventory,
    inventory_tree: &'a QuadTree,
    detection_method: &dyn DetectionMethod,
) -> (Position, HashMap<&'a str, usize>) {
    let boundary = Boundary::new((inventory.x - radius, inventory.z - radius), radius, radius);
    let mut items_in_area_by_group =
        inventory_tree
            .query(boundary)
            .fold(HashMap::new(), |mut items_in_area, inv| {
                inv.items.iter().for_each(|(_key, item)| {
                    items_in_area
                        .entry(item.group_key)
                        .and_modify(|count| *count += item.count)
                        .or_insert(item.count);
                });
                items_in_area
            });
    items_in_area_by_group.retain(|group, count| detection_method.exceeds_max(group, *count));
    (
        Position {
            x: inventory.x,
            y: inventory.y,
            z: inventory.z,
        },
        items_in_area_by_group,
    )
}

fn search_inventories_in_region<'a>(
    region: &Path,
    config: &'a SearchDupeStashesConfig,
) -> Result<Vec<FoundInventory<'a>>, Error> {
    let region = OpenOptions::new().read(true).open(region)?;
    let region = read_file(region)?;
    let region = mc_map_reader::load_region(region.as_slice(), None)?;
    let inv = region
        .chunks
        .iter()
        .filter_map(|c| search_inventories_in_chunk(c, config))
        .fold(Vec::default(), |mut invnentories, mut new| {
            invnentories.append(&mut new);
            invnentories
        });
    Ok(inv)
}

fn search_inventories_in_chunk<'a, 'b>(
    chunk: &ChunkData,
    config: &'b SearchDupeStashesConfig,
) -> Option<Vec<FoundInventory<'a>>>
where
    'b: 'a,
{
    let Some(block_entities) = &chunk.block_entities else {
        return None;
    };
    let res = block_entities
        .iter()
        .filter_map(|block_entity| {
            let inventory: &dyn InventoryBlock = match &block_entity.entity_type {
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
    log::debug!("Found {} inventories in chunk", res.len());
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
    let x = base_entity.x;
    let z = base_entity.z;
    let y = base_entity.y;
    let items = if let Some(items) = inventory.items() {
        items.iter().fold(HashMap::default(), |mut item_map, item| {
            add_item_to_map(item, &mut item_map, config, x, y, z);
            if item_is_shulker_box(&item.item.id) {
                search_subinventory(&item.item, &mut item_map, config, x, y, z)
            }
            item_map
        })
    } else {
        return None;
    };
    log::debug!(
        "Found inventory at ({x}, {y}, {z}) with {items_len} items",
        items_len = items.len()
    );
    Some(FoundInventory {
        inventory_type: base_entity.id.clone(),
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
    let Some(tag) = &item.tag else {
        return;
    };
    let Some(block_entity_tag) = tag.get("BlockEntityTag").cloned() else {
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
    item: &mc_map_reader::data::item::ItemWithSlot,
    item_map: &mut HashMap<&'a String, FoundItem<'a>>,
    config: &'b SearchDupeStashesConfig,
    x: i32,
    y: i32,
    z: i32,
) where
    'b: 'a,
{
    let item = &item.item;
    config
        .groups
        .iter()
        .filter(|(_, group)| group.matches(item))
        .for_each(|(group_name, group)| {
            let mul = group
                .items
                .iter()
                .find(|i| i.matches(item))
                .map(|i| i.multiplier)
                .unwrap_or(1);
            item_map
                .entry(group_name)
                .and_modify(|item_entry: &mut FoundItem| {
                    item_entry.count += item.count as usize * mul;
                })
                .or_insert_with(|| FoundItem {
                    group_key: group_name,
                    position: Position { x, y, z },
                    count: item.count as usize * mul,
                });
        });
}

fn find_corners(
    (mut x1, mut z1, mut x2, mut z2): (i32, i32, i32, i32),
    inv: &FoundInventory,
) -> (i32, i32, i32, i32) {
    x1 = x1.min(inv.x);
    z1 = z1.min(inv.z);
    x2 = x2.max(inv.x);
    z2 = z2.max(inv.z);
    (x1, z1, x2, z2)
}
