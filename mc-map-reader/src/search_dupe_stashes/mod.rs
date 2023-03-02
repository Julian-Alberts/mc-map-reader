mod data;

use std::{collections::HashMap, fs::OpenOptions, path::Path};
use data::*;

use mc_map_reader_lib::{
    nbt_data::{
        block_entity::{BlockEntity, BlockEntityType, InventoryBlock, Item, ShulkerBox},
        chunk::ChunkData,
    },
    LoadMcSave,
};

use crate::{
    quadtree::{Bounds, QuadTree},
    read_file, config::Config,
};

pub fn main(world_dir: &Path, data: crate::arguments::SearchDupeStashes, config: Config) {
    let region_groups = if let Some(area) = data.area {
        mc_map_reader_lib::files::get_region_files_in_area(
            world_dir, area.x1, area.z1, area.x2, area.z2,
        )
    } else {
        mc_map_reader_lib::files::get_region_files(world_dir)
            .expect("Could not read region directory")
    };
    let mut thread_count = region_groups.len() / 24;
    if thread_count < 1 {
        thread_count = 1
    }
    let region_groups = region_groups.chunks(thread_count);
    let inventories = std::thread::scope(|s| {
        let mut thread_handler = Vec::with_capacity(4);
        for regions in region_groups {
            let thread = s.spawn(move || {
                regions.iter().fold(Vec::new(), |inventories, region| {
                    let data = match OpenOptions::new().read(true).open(region) {
                        Ok(file) => read_file(file),
                        Err(e) => panic!("{e}"),
                    };
                    let data = match data {
                        Ok(data) => data,
                        Err(e) => panic!("{e}"),
                    };
                    let region = match mc_map_reader_lib::Loader.load_from_bytes(&data[..]) {
                        Ok(data) => data,
                        Err(e) => panic!("{e}"),
                    };
                    region
                        .chunks()
                        .iter()
                        .filter_map(|c| c.as_ref())
                        .map(search_dupe_stashes_in_chunk)
                        .fold(inventories, |mut invnentories, mut new| {
                            invnentories.append(&mut new);
                            invnentories
                        })
                })
            });
            thread_handler.push(thread)
        }
        thread_handler
            .into_iter()
            .map(|j| j.join().expect("PANIC!!!"))
            .fold(Vec::new(), |mut inv, mut new| {
                inv.append(&mut new);
                inv
            })
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

fn search_dupe_stashes_in_chunk(chunk: &ChunkData) -> Vec<FoundInventory> {
    let Some(block_entities) = chunk.block_entities() else {
        return Vec::default()
    };

    block_entities
        .iter()
        .filter_map(|block_entity| match block_entity.entity_type() {
            BlockEntityType::Barrel(block) => search_inventory_block(block, block_entity),
            BlockEntityType::Chest(block) => search_inventory_block(block, block_entity),
            BlockEntityType::Dispenser(block) => search_inventory_block(block, block_entity),
            BlockEntityType::Dropper(block) => search_inventory_block(block, block_entity),
            BlockEntityType::Hopper(block) => search_inventory_block(block, block_entity),
            BlockEntityType::ShulkerBox(block) => search_inventory_block(block, block_entity),
            BlockEntityType::TrappedChest(block) => search_inventory_block(block, block_entity),
            _ => None,
        })
        .collect::<Vec<_>>()
}

fn search_inventory_block(
    inventory: &dyn InventoryBlock,
    base_entity: &BlockEntity,
) -> Option<FoundInventory> {
    if inventory.loot_table().is_some() || inventory.loot_table_seed().is_some() {
        return None;
    }
    let x = base_entity.x();
    let y = base_entity.y();
    let items = if let Some(items) = inventory.items() {
        items.iter().fold(HashMap::default(), |mut item_map, item| {
            let item = item.item();
            item_map
                .entry(item.id().to_owned())
                .and_modify(|item_entry: &mut FoundItem| item_entry.count += item.count() as i16)
                .or_insert(FoundItem {
                    id: item.id().to_owned(),
                    count: item.count() as i16,
                });
            if item.id().starts_with("minecraft") && item.id().ends_with("shulker_box") {
                search_subinventory(item, &mut item_map)
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

fn search_subinventory(item: &Item, item_map: &mut HashMap<String, FoundItem>) {
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
            let item = item.item();
            item_map
                .entry(item.id().to_owned())
                .and_modify(|item_entry: &mut FoundItem| item_entry.count += item.count() as i16)
                .or_insert(FoundItem {
                    id: item.id().to_owned(),
                    count: item.count() as i16,
                });
        })
    }
}
