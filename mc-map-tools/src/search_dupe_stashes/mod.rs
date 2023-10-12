pub mod args;
pub mod config;
mod data;
mod detection_method;

use data::*;
use qutee::{Boundary, ConstCap};
#[cfg(feature = "parallel")]
use rayon::prelude::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::io::Write;
use std::{collections::HashMap, path::Path};
use async_std::fs::OpenOptions;
use futures::AsyncWriteExt;

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
use crate::file::FileItemWrite;
use crate::file::region_inventories::RegionInventories;

use self::config::SearchDupeStashesConfig;

type QuadTree<'a> = qutee::QuadTree<i32, &'a FoundInventory<'a>, ConstCap<32>>;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    RegionLoadError(#[from] RegionLoadError),
}

pub async fn main(
    world_dir: &Path,
    data: args::SearchDupeStashes,
    config: Config,
    writer: &mut dyn Write,
) {
    let detection_method = Box::new(detection_method::Absolute::new(
        &config.search_dupe_stashes.groups,
    ));
    let region_files = if let Some(area) = data.area {
        mc_map_reader::files::get_regions_in_area(
            world_dir, None, area.x1, area.z1, area.x2, area.z2,
        )
    } else {
        mc_map_reader::files::get_regions(world_dir, None)
            .expect("Could not read region directory")
    };
    log::debug!(
        "Found {} region files {region_files:#?}",
        region_files.len()
    );
    let config = &config.search_dupe_stashes;

    let mut inventories_dir = std::env::temp_dir();
    inventories_dir.push(format!("mc-map-tools-{}", std::process::id()));
    inventories_dir.push("inventories");

    if let Err(e) = async_std::fs::create_dir(&inventories_dir).await {
        log::error!("Error creating tmp directory: {e}");
        return
    }
    let inventories_dir = inventories_dir.as_path();
    let regions_future = region_files
        .into_iter()
        .map(|region| async move {
            let inventories = search_inventories_in_region(region.as_path(), config).await;
            let inventories = match inventories {
                Ok(inventories) => inventories,
                Err(err) => {
                    log::error!("{err}");
                    return Err(err);
                }
            };
            save_region_inventories(&inventories_dir, region.x(), region.z(), inventories).await.map_err(Error::Io)
        });
    let errors = futures::future::join_all(regions_future).await;

    for e in errors {
        if let Err(e) = e {
            log::error!("Error while reading region file {}", e);
        }
    }
/*
    if inventories.is_empty() {
        log::info!("No inventories found");
        return;
    }
    log::info!("Found {} inventories", inventories.len());

    let (x1, z1, x2, z2) = inventories
        .iter()
        .fold((i32::MAX, i32::MAX, i32::MIN, i32::MIN), |v, inv| {
            find_corners(v, &inv.position)
        });
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
        });*/
}

fn collect_items_in_area<'a>(
    radius: i32,
    inventory: &FoundInventory,
    inventory_tree: &'a QuadTree,
    detection_method: &dyn DetectionMethod,
) -> (Position, HashMap<&'a str, usize>) {
    let boundary = Boundary::new(
        (inventory.position.x - radius, inventory.position.z - radius),
        radius,
        radius,
    );
    let mut items_in_area_by_group =
        inventory_tree
            .query(boundary)
            .fold(HashMap::new(), |mut items_in_area, inv| {
                inv.items.iter().for_each(|(key, item)| {
                    items_in_area
                        .entry(*key)
                        .and_modify(|count| *count += item.count)
                        .or_insert(item.count);
                });
                items_in_area
            });
    items_in_area_by_group.retain(|group, count| detection_method.exceeds_max(group, *count));
    (inventory.position.clone(), items_in_area_by_group)
}

async fn search_inventories_in_region<'a>(
    region: &Path,
    config: &'a SearchDupeStashesConfig,
) -> Result<impl Iterator<Item = FoundInventory<'a>>, Error> {
    let region = OpenOptions::new().read(true).open(region).await?;
    let region = read_file(region).await?;
    let region = mc_map_reader::load_region(region.as_slice(), None)?;
    let inv = region
        .chunks
        .into_iter()
        .filter_map(|c| search_inventories_in_chunk(c, config))
        .flatten();
    Ok(inv)
}

fn search_inventories_in_chunk<'inventory, 'config, 'chunk>(
    chunk: ChunkData,
    config: &'config SearchDupeStashesConfig,
) -> Option<impl Iterator<Item = FoundInventory<'inventory>>>
where
    'config: 'inventory,
    'chunk: 'inventory
{
    let Some(block_entities) = chunk.block_entities else {
        return None;
    };
    let inventories = block_entities
        .into_iter()
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
            search_inventory_block(inventory, &block_entity, config)
        });
    Some(inventories)
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
            add_item_to_map(item, &mut item_map, config);
            if item_is_shulker_box(&item.item.id) {
                search_subinventory(&item.item, &mut item_map, config)
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
        position: Position { x, y, z },
    })
}

#[inline]
fn item_is_shulker_box(id: &str) -> bool {
    id.starts_with("minecraft:") && id.ends_with("shulker_box")
}

fn search_subinventory<'a, 'b>(
    item: &Item,
    item_map: &mut HashMap<&'a str, FoundItem>,
    config: &'b SearchDupeStashesConfig,
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
            .for_each(|item| add_item_to_map(item, item_map, config))
    }
}

fn add_item_to_map<'a, 'b>(
    item: &mc_map_reader::data::item::ItemWithSlot,
    item_map: &mut HashMap<&'a str, FoundItem>,
    config: &'b SearchDupeStashesConfig,
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
                    count: item.count as usize * mul,
                });
        });
}

async fn save_region_inventories<'a>(dir: &Path, x: i32, z: i32, inventories: impl Iterator<Item = FoundInventory<'a>>) -> std::io::Result<()> {
    use crate::file::region_inventories::{Item, Inventory, RegionInventories};

    fn into_inv_file_item(key: &str, item: FoundItem) -> Item {
        Item {
            group_id: 0,
            count: item.count as i32,
        }
    }

    let path = dir.join(format!("region_{x}_{z}.mtri"));
    let mut file = OpenOptions::new().create(true).write(true).open(path).await?;
    let inventories = RegionInventories {
        inventories: inventories.map(|inv| {
            Inventory {
                x: inv.position.x,
                y: inv.position.y,
                z: inv.position.z,
                items: inv.items.into_iter().map(|(key, item)| into_inv_file_item(key, item)).collect()
            }
        }).collect(),
    };
    let mut buf = Vec::new();
    inventories.write(&mut buf).await;
    file.write_all(&buf).await?;
    Ok(())
}

fn find_corners(
    (mut x1, mut z1, mut x2, mut z2): (i32, i32, i32, i32),
    inv: &Position,
) -> (i32, i32, i32, i32) {
    x1 = x1.min(inv.x);
    z1 = z1.min(inv.z);
    x2 = x2.max(inv.x);
    z2 = z2.max(inv.z);
    (x1, z1, x2, z2)
}
