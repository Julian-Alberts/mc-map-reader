pub mod args;
pub mod config;
mod data;

use data::*;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::{collections::HashMap, fs::OpenOptions, path::Path};

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
    let config = config.search_pube_stashes.unwrap_or_default();
    let inventories = region_groups
        .into_par_iter()
        .map(|region| OpenOptions::new().read(true).open(region).unwrap())
        .map(read_file)
        .map(Result::unwrap)
        .map(|data| {
            mc_map_reader::Loader
                .load_from_bytes(&data[..])
                .unwrap()
        })
        .map(|region| {
            region
                .chunks()
                .iter()
                .map(|c| search_dupe_stashes_in_chunk(c, &config))
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
    let inventories = inventories
        .iter()
        .fold(QuadTree::new(bounds), |mut tree, inv| {
            tree.insert(inv);
            tree
        });
    println!("{inventories:#?}")
}

fn search_dupe_stashes_in_chunk<'a, 'b>(
    chunk: &ChunkData,
    config: &'b SearchDupeStashesConfig,
) -> Vec<FoundInventory<'a>> 
    where 'b: 'a
{
    let Some(block_entities) = chunk.block_entities() else {
        return Vec::default()
    };

    block_entities
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
        .collect::<Vec<_>>()
}

fn search_inventory_block<'a, 'b>(
    inventory: &dyn InventoryBlock,
    base_entity: &BlockEntity,
    config: &'b SearchDupeStashesConfig,
) -> Option<FoundInventory<'a>> 
    where 'b: 'a
{
    if inventory.loot_table().is_some() || inventory.loot_table_seed().is_some() {
        return None;
    }
    let x = base_entity.x();
    let y = base_entity.y();
    let items = if let Some(items) = inventory.items() {
        items.iter().fold(HashMap::default(), |mut item_map, item| {
            add_item_to_map(item, &mut item_map, config);
            if item_is_shulker_box(item.item().id()) {
                search_subinventory(item.item(), &mut item_map, config)
            }
            item_map
        })
    } else {
        HashMap::default()
    };
    Some(FoundInventory {
        inventory_type: base_entity.id().clone(),
        items,
        x,
        z: y,
    })
}

#[inline]
fn item_is_shulker_box(id: &str) -> bool {
    id.starts_with("minecraft:") && id.ends_with("shulker_box")
}

fn search_subinventory<'a, 'b>(item: &Item, item_map: &mut HashMap<String, FoundItem<'a>>, config: &'b SearchDupeStashesConfig) 
    where 'b: 'a
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
        items.iter().for_each(|item| {
            add_item_to_map(item, item_map, config)
        })
    }
}

fn add_item_to_map<'a, 'b>(item: &mc_map_reader::nbt_data::block_entity::ItemWithSlot, item_map: &mut HashMap<String, FoundItem<'a>>, config: &'b SearchDupeStashesConfig) 
    where 'b: 'a
{
    let item = item.item();
    let Some((group_key, _)) = config.groups.iter().find(|(_, item_config)| item_config.items.iter().any(|i| i.matches(item))) else {
        return
    };
    item_map
        .entry(group_key.clone())
        .and_modify(|item_entry: &mut FoundItem| item_entry.count += item.count() as i16)
        .or_insert(FoundItem {
            group_key,
            count: item.count() as i16,
        });
}

