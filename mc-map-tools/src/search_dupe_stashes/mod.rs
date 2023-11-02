pub mod args;
pub mod config;
mod data;
mod detection_method;

use async_std::fs::OpenOptions;
use data::*;
use futures::AsyncWriteExt;
use qutee::{Boundary, ConstCap};
use std::hash::{Hash, Hasher};
use std::io::Write;
use std::sync::Arc;
use std::{collections::HashMap, path::Path};

use mc_map_reader::{
    data::{
        block_entity::{BlockEntity, BlockEntityType, InventoryBlock, ShulkerBox},
        chunk::ChunkData,
        item::Item,
    },
    RegionLoadError,
};

use crate::file::region_inventories::Inventory;
use crate::file::FileItemWrite;
use crate::search_dupe_stashes::detection_method::DetectionMethod;
use crate::tmp_dir::TmpDir;
use crate::{config::Config, read_file};

use self::config::SearchDupeStashesConfig;

const BLOCKS_IN_CHUNK: i32 = 16;
const CHUNKS_IN_REGION_FILE: i32 = 32;
type QuadTree<'a> = qutee::QuadTree<i32, &'a Inventory, ConstCap<32>>;

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
        mc_map_reader::files::get_regions(world_dir, None).expect("Could not read region directory")
    };
    log::debug!(
        "Found {} region files {region_files:#?}",
        region_files.len()
    );
    let config = &config.search_dupe_stashes;

    let temp_dir = TmpDir::new().expect("Error creating tmp dir");
    let inventories_dir = temp_dir.as_ref().join("inventories");

    if let Err(e) = async_std::fs::create_dir(&inventories_dir).await {
        log::error!("Error creating tmp directory: {e}");
        return;
    }
    let inventories_dir = inventories_dir.as_path();
    let regions_future = region_files.into_iter().map(|region| async move {
        let inventories = search_inventories_in_region(region.as_path(), config).await;
        let inventories = match inventories {
            Ok(inventories) => inventories,
            Err(err) => {
                log::error!("{err}");
                return Err(err);
            }
        };
        save_region_inventories(inventories_dir, region.x(), region.z(), inventories).await?;
        Ok((region.x(), region.z()))
    });
    let results = futures::future::join_all(regions_future).await;

    let regions = results.into_iter().filter_map(|e| match e {
        Ok((x, z)) => Some((x, z)),
        Err(e) => {
            log::error!("Error while reading region file {}", e);
            None
        }
    });

    let group_hash_lookup_table = HashMap::from_iter(config.groups.keys().map(|key| {
        let mut hasher = std::collections::hash_map::DefaultHasher::default();
        key.hash(&mut hasher);
        (hasher.finish(), key.as_str())
    }));
    let region_cache = RegionInventoryCache::new(inventories_dir, 128);
    let detection_method_ref = detection_method.as_ref();
    let group_hash_lookup_table_ref = &group_hash_lookup_table;
    let region_cache_ref = &region_cache;
    let potential_stash_locations = regions.map(|(x, z)| async move {
        let top = z - 1;
        let bottom = z + 1;
        let left = x - 1;
        let right = x + 1;
        let regions =
            (left..right).flat_map(|x| (top..bottom).map(move |z| region_cache_ref.get(x, z)));
        let regions = futures::future::join_all(regions).await;

        let Some(Ok(center_region)) = regions.get(4) else {
            return Vec::default();
        };
        let center_region = Arc::clone(center_region);

        let top_left_coords = min_corner_block_in_chunk(left, top);
        let bottom_right_coords = max_corner_block_in_chunk(right, bottom);
        let mut tree = QuadTree::new_with_const_cap(Boundary::between_points(
            top_left_coords,
            bottom_right_coords,
        ));
        regions
            .iter()
            .filter_map(|region| match region {
                Ok(region) => Some(region.inventories.iter()),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => None,
                Err(e) => {
                    log::error!("Error reading region inventory file {e}");
                    None
                }
            })
            .flatten()
            .for_each(|inventory| {
                tree.insert_at((inventory.x, inventory.z), inventory)
                    .expect("Inventory is outside of quad tree");
            });
        center_region
            .inventories
            .iter()
            .map(move |inventory| {
                collect_items_in_area(
                    data.radius as i32,
                    inventory,
                    &tree,
                    detection_method_ref,
                    group_hash_lookup_table_ref,
                )
            })
            .collect::<Vec<_>>()
    });

    let potential_stash_locations = futures::future::join_all(potential_stash_locations).await;

    potential_stash_locations
        .into_iter()
        .filter(|location| location.is_empty())
        .flatten()
        .for_each(|(Position { x, y, z }, sl)| {
            sl.iter().for_each(|(item, count)| {
                writer
                    .write_all(format!("{x},{y},{z},{item},{count}").as_bytes())
                    .expect("Error writing message");
            })
        });

    if let Err(err) = async_std::fs::remove_dir_all(temp_dir.as_ref()).await {
        log::error!(
            "Could not remove temporary directory \"{}\": {err}",
            temp_dir.as_ref().display()
        );
    }
}

fn min_corner_block_in_chunk(region_x: i32, region_z: i32) -> (i32, i32) {
    let chunk_x = region_x << 5;
    let chunk_z = region_z << 5;
    let block_x = chunk_x * BLOCKS_IN_CHUNK;
    let block_z = chunk_z * BLOCKS_IN_CHUNK;
    (block_x, block_z)
}

fn max_corner_block_in_chunk(region_x: i32, region_z: i32) -> (i32, i32) {
    let (min_block_x, min_block_z) = min_corner_block_in_chunk(region_x, region_z);
    (
        min_block_x + CHUNKS_IN_REGION_FILE * BLOCKS_IN_CHUNK,
        min_block_z + CHUNKS_IN_REGION_FILE * BLOCKS_IN_CHUNK,
    )
}

fn collect_items_in_area(
    radius: i32,
    inventory: &Inventory,
    inventory_tree: &QuadTree,
    detection_method: &dyn DetectionMethod,
    group_hash_lookup_table: &HashMap<u64, &str>,
) -> (Position, HashMap<u64, u64>) {
    let boundary = Boundary::new((inventory.x - radius, inventory.z - radius), radius, radius);
    let mut items_in_area_by_group =
        inventory_tree
            .query(boundary)
            .fold(HashMap::new(), |mut items_in_area, inv| {
                inv.items.iter().for_each(|item| {
                    items_in_area
                        .entry(item.group_id)
                        .and_modify(|count| *count += item.count)
                        .or_insert(item.count);
                });
                items_in_area
            });
    items_in_area_by_group.retain(|group, count| {
        detection_method.exceeds_max(
            group_hash_lookup_table
                .get(group)
                .expect("Tried to access unknown group"),
            *count as usize,
        )
    });
    (
        Position {
            x: inventory.x,
            y: inventory.y,
            z: inventory.z,
        },
        items_in_area_by_group,
    )
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
    'chunk: 'inventory,
{
    let Some(block_entities) = chunk.block_entities else {
        return None;
    };
    let inventories = block_entities.into_iter().filter_map(|block_entity| {
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

async fn save_region_inventories<'a>(
    dir: &Path,
    x: i32,
    z: i32,
    inventories: impl Iterator<Item = FoundInventory<'a>>,
) -> std::io::Result<()> {
    use crate::file::region_inventories::{Item, RegionInventories};

    fn into_inv_file_item(key: &str, item: FoundItem) -> Item {
        let mut hasher = std::collections::hash_map::DefaultHasher::default();
        key.hash(&mut hasher);
        let group_id = hasher.finish();
        Item {
            group_id,
            count: item.count as u64,
        }
    }

    let path = dir.join(format!("region_{x}_{z}.mtri"));
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(path)
        .await?;
    let inventories = RegionInventories {
        inventories: inventories
            .map(|inv| Inventory {
                x: inv.position.x,
                y: inv.position.y,
                z: inv.position.z,
                items: inv
                    .items
                    .into_iter()
                    .map(|(key, item)| into_inv_file_item(key, item))
                    .collect(),
            })
            .collect(),
    };
    let mut buf = Vec::new();
    inventories.write(&mut buf).await?;
    file.write_all(&buf).await?;
    Ok(())
}
