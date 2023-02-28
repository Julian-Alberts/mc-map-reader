use std::{collections::HashMap, fs::OpenOptions, path::Path};

use mc_map_reader_lib::{
    nbt_data::{
        block_entity::{self, BlockEntity, BlockEntityType, InventoryBlock},
        chunk::ChunkData,
    },
    LoadMcSave,
};

use crate::{
    quadtree::{Bounded, Bounds, QuadTree},
    read_file,
};

pub fn search_dupe_stashes(world_dir: &Path, data: crate::arguments::SearchDupeStashes) {
    let region_groups = if let Some(area) = data.area {
        mc_map_reader_lib::files::get_region_files_in_area(
            world_dir, area.x1, area.z1, area.x2, area.z2,
        )
    } else {
        mc_map_reader_lib::files::get_region_files(world_dir)
            .expect("Could not read region directory")
    };
    let region_groups = region_groups.chunks(region_groups.len() / 24);
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
    assert!(x1 <= x2 && z1 <= z2);
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
                .and_modify(|count| *count += item.count() as i16)
                .or_insert(item.count() as i16);
            item_map
        })
    } else {
        HashMap::default()
    };

    Some(FoundInventory {
        inventory_type: base_entity.id().clone(),
        item_counts: items,
        x,
        z: y,
    })
}

#[derive(Debug)]
pub struct FoundInventory {
    inventory_type: String,
    x: i32,
    z: i32,
    item_counts: HashMap<String, i16>,
}

impl Bounded for FoundInventory {
    fn bounds(&self) -> Bounds {
        Bounds {
            x: self.x as f32,
            y: self.z as f32,
            width: 1.,
            height: 1.,
        }
    }
}
