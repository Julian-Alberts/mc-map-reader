use std::collections::HashMap;

use thiserror::Error;

use crate::{
    nbt::Tag,
    nbt_data::{block_entity::*, chunk::MissingData},
};

pub fn load(data: &Tag) -> crate::load::Result<BlockEntity> {
    let nbt_data = data.get_as_map()?;
    let id = nbt_data
        .get("tag")
        .ok_or(BlockEntityBuilderError::UnsetId)
        .map_err(BlockEntityMissingDataError::from)
        .map_err(MissingData::from)?
        .get_as_string()?;
    let mut beb = parse_generic_block_entity(nbt_data)?;
    let ty = match id.as_str() {
        "banners" => BlockEntityType::Banner(parse_banner(nbt_data)?),
        "barrel" => BlockEntityType::Barrel(parse_barrel(nbt_data)?),
        "beacon" => BlockEntityType::Beacon(parse_beacon(nbt_data)?),
        "bed" => BlockEntityType::Bed,
        "beehive" => BlockEntityType::Beehive(parse_beehive(nbt_data)?),
        "bell" => BlockEntityType::Bell,
        _ => BlockEntityType::Other(data.clone())
    };
    beb.with_entity_type(ty);
    let be = beb.try_build().map_err(BlockEntityMissingDataError::from).map_err(MissingData::from)?;
    Ok(be)
}

fn parse_beehive(nbt_data: &HashMap<String, Tag>) -> crate::load::Result<Beehive> {
    let mut beehive_builder = BeehiveBuilder::default();
    for (key, value) in nbt_data {
        match key.as_str() {
            "Bees" => beehive_builder.with_bees(parse_bees(value)?),
            "FlowerPos" => beehive_builder.with_flower_pos(parse_flower_pos(value)?),
            _ => &mut beehive_builder
        };
    }
    let beehive = beehive_builder.try_build().map_err(BlockEntityMissingDataError::from).map_err(MissingData::from)?;
    Ok(beehive)
}

fn parse_bees(nbt_bees: &Tag) -> crate::load::Result<Vec<BeeInHive>> {
    let nbt_bees = nbt_bees.get_as_vec_tag()?;
    let mut bees = Vec::with_capacity(nbt_bees.len());
    for nbt_bee in nbt_bees {
        let nbt_bee = nbt_bee.get_as_map()?;
        let mut bee_builder = BeeInHiveBuilder::default();
        for (key, value) in nbt_bee {
            match key.as_str() {
                "EntityData" => bee_builder.with_entity_data(super::entity::parse_entity_from_tag(value)?),
                "MinOccupationTicks" => bee_builder.with_min_occupation_ticks(*value.get_as_i32()?),
                "TicksInHive" => bee_builder.with_ticks_in_hive(*value.get_as_i32()?),
                _ => &mut bee_builder
            };
        }
        bees.push(bee_builder.try_build().map_err(BlockEntityMissingDataError::from).map_err(MissingData::from)?);
    }
    Ok(bees)
}

fn parse_flower_pos(nbt_flower_pos: &Tag) -> crate::load::Result<FlowerPos> {
    let mut flower_pos_builder = FlowerPosBuilder::default();
    let nbt_flower_pos = nbt_flower_pos.get_as_map()?;
    for (key, value) in nbt_flower_pos {
        match key.as_str() {
            "X" => flower_pos_builder.with_x(*value.get_as_i32()?),
            "Y" => flower_pos_builder.with_y(*value.get_as_i32()?),
            "Z" => flower_pos_builder.with_z(*value.get_as_i32()?),
            _ => &mut flower_pos_builder
        };
    }
    let flower_pos = flower_pos_builder.try_build().map_err(BlockEntityMissingDataError::from).map_err(MissingData::from)?;
    Ok(flower_pos)
}

fn parse_beacon(nbt_data: &HashMap<String, Tag>) -> crate::load::Result<Beacon> {
    let mut bb = BeaconBuilder::default();
    for (key, value) in nbt_data {
        match key.as_str() {
            "CustomName" => bb.with_custom_name(value.get_as_string()?.clone()),
            "Lock" => bb.with_lock(value.get_as_string()?.clone()),
            "Primary" => bb.with_primary(*value.get_as_i32()?),
            "Secondary" => bb.with_secondary(*value.get_as_i32()?),
            _ => &mut bb
        };
    }
    let b = bb.try_build().map_err(BlockEntityMissingDataError::from).map_err(MissingData::from)?;
    Ok(b)
}

fn parse_barrel(nbt_data: &HashMap<String, Tag>) -> crate::load::Result<Barrel> {
    let mut bb = BarrelBuilder::default();
    parse_inventory_block_entity(&mut bb, nbt_data)?;
    let b = bb.try_build().map_err(BlockEntityMissingDataError::from).map_err(MissingData::from)?;
    Ok(b)
}

fn parse_inventory_block_entity(builder: &mut impl InventoryBlockEntityBuilder, nbt_data: &HashMap<String, Tag>) -> crate::load::Result<()> {
    for (key, value) in nbt_data {
        match key.as_str() {
            "CustomName" => builder.set_custom_name(value.get_as_string()?.clone()),
            "Items" => builder.set_items(parse_items_with_slot(value)?),
            "Lock" => builder.set_lock(value.get_as_string()?.clone()),
            "LootTalbe" => builder.set_loot_table(value.get_as_string()?.clone()),
            "LootTableSeed" => builder.set_loot_table_seed(value.get_as_i64()?.clone()),
            _ => {}
        };
    }
    Ok(())
}

fn parse_items_with_slot(nbt_items: &Tag) -> crate::load::Result<Vec<ItemWithSlot>> {
    let nbt_items = nbt_items.get_as_vec_tag()?;
    let mut items = Vec::with_capacity(nbt_items.len());
    for item in nbt_items {
        let item = item.get_as_map()?;
        let mut iwsb = ItemWithSlotBuilder::default();
        let mut ib = ItemBuilder::default();
        for (key, value) in item {
            match key.as_str() {
                "Count" => {ib.with_count(*value.get_as_i8()?);},
                "Slot" => {iwsb.with_slot(*value.get_as_i8()?);},
                "id" => {ib.with_id(value.get_as_string()?.clone());},
                "tag" => {ib.with_tag(value.get_as_map()?.clone());},
                _ => {}
            }
        }
        let item = ib.try_build().map_err(BlockEntityMissingDataError::from).map_err(MissingData::from)?;
        iwsb.with_item(item);
        let item_with_slot = iwsb.try_build().map_err(BlockEntityMissingDataError::from).map_err(MissingData::from)?;
        items.push(item_with_slot)
    }
    Ok(items)
}

fn parse_banner(data: &HashMap<String, Tag>) -> crate::load::Result<Banner> {
    let mut bb = BannerBuilder::default();
    for (key, value) in data {
        match key.as_str() {
            "CustomName" => bb.with_custom_name(value.get_as_string()?.clone()),
            "Patterns" => bb.with_patterns(parse_banner_patterns(value)?),
            _ => &mut bb
        };
    }
    let banner = bb.try_build().map_err(BlockEntityMissingDataError::from).map_err(MissingData::from)?;
    Ok(banner)
}

fn parse_banner_patterns(value: &Tag) -> crate::load::Result<Vec<BannerPattern>> {
    let nbt_patterns = value.get_as_vec_tag()?;
    let mut patterns = Vec::with_capacity(nbt_patterns.len());
    for nbt_pattern in nbt_patterns {
        let nbt_pattern = nbt_pattern.get_as_map()?;
        let mut pb = BannerPatternBuilder::default();
        for (key, value) in nbt_pattern {
            match key.as_str() {
                "Color" => pb.with_color(*value.get_as_i32()?),
                "Pattern" => pb.with_pattern(value.get_as_string()?.clone()),
                _ => &mut pb
            };
        }
        patterns.push(pb.try_build().map_err(BlockEntityMissingDataError::from).map_err(MissingData::from)?)
    }
    Ok(patterns)
}

fn parse_generic_block_entity(nbt_data: &HashMap<String, Tag>) -> crate::load::Result<BlockEntityBuilder> {
    let mut beb = BlockEntityBuilder::default();
    for (key, value) in nbt_data {
        match key.as_str() {
            "id" => beb.with_id(value.get_as_string()?.clone()),
            "keepPacked" => beb.with_keep_packed(*value.get_as_i8()? == 1),
            "x" => beb.with_x(*value.get_as_i32()?),
            "y" => beb.with_y(*value.get_as_i32()?),
            "z" => beb.with_z(*value.get_as_i32()?),
            _ => &mut beb
        };
    }
    Ok(beb)
}

#[derive(Debug, Error)]
pub enum BlockEntityMissingDataError {
    #[error(transparent)]
    BlockEntity(#[from] BlockEntityBuilderError),
    #[error(transparent)]
    Banner(#[from] BannerBuilderError),
    #[error(transparent)]
    BannerPattern(#[from] BannerPatternBuilderError),
    #[error(transparent)]
    Barrel(#[from] BarrelBuilderError),
    #[error(transparent)]
    Item(#[from] ItemBuilderError),
    #[error(transparent)]
    ItemWithSlot(#[from] ItemWithSlotBuilderError),
    #[error(transparent)]
    Beacon(#[from] BeaconBuilderError),
    #[error(transparent)]
    Beehive(#[from] BeehiveBuilderError),
    #[error(transparent)]
    FlowerPos(#[from] FlowerPosBuilderError),
    #[error(transparent)]
    BeeInHive(#[from] BeeInHiveBuilderError),
}
