use std::collections::HashMap;

use thiserror::Error;

use crate::{
    nbt::Tag,
    nbt_data::{block_entity::*, chunk::MissingData},
};

impl TryFrom<Tag> for BlockEntity {
    type Error = crate::nbt::Error;
    fn try_from(data: Tag) -> Result<Self, Self::Error> {
    let nbt_data = data.get_as_map()?;
    let Tag::String(id) = nbt_data
        .get("tag")
        .ok_or(BlockEntityBuilderError::UnsetId)
        .map_err(BlockEntityMissingDataError::from)
        .map_err(MissingData::from)? else {
            return Err(crate::nbt::Error::InvalidValue.into());
        };
    let id = id.clone();
    
    let (mut beb, nbt_data) = parse_generic_block_entity(nbt_data)?;
    let ty = match id.as_str() {
        "banners" => BlockEntityType::Banner(parse_banner(nbt_data)?),
        "barrel" => BlockEntityType::Barrel(parse_barrel(nbt_data)?),
        "beacon" => BlockEntityType::Beacon(parse_beacon(nbt_data)?),
        "bed" => BlockEntityType::Bed,
        "beehive" => BlockEntityType::Beehive(parse_beehive(nbt_data)?),
        "bell" => BlockEntityType::Bell,
        _ => BlockEntityType::Other(nbt_data)
    };
    beb.set_entity_type(ty);
    let be = beb.try_build().map_err(BlockEntityMissingDataError::from).map_err(MissingData::from)?;
    Ok(be)
}
}

fn parse_beehive(nbt_data: HashMap<String, Tag>) -> Result<Beehive, crate::nbt::Error> {
    let mut beehive_builder = BeehiveBuilder::default();
    for (key, value) in nbt_data {
        match key.as_str() {
            "Bees" => beehive_builder.set_bees(parse_bees(value)?),
            "FlowerPos" => beehive_builder.set_flower_pos(parse_flower_pos(value)?),
            _ => {}
        }
    }
    let beehive = beehive_builder.try_build().map_err(BlockEntityMissingDataError::from).map_err(MissingData::from)?;
    Ok(beehive)
}

fn parse_bees(nbt_bees: Tag) -> Result<Vec<BeeInHive>, crate::nbt::Error> {
    let nbt_bees = nbt_bees.get_as_list()?;
    nbt_bees.into_iter().map(|nbt_bee| {
        let nbt_bee = nbt_bee.get_as_map()?;
        let mut bee_builder = BeeInHiveBuilder::default();
        for (key, value) in nbt_bee {
            match key.as_str() {
                "EntityData" => bee_builder.with_entity_data(value.try_into()?),
                "MinOccupationTicks" => bee_builder.with_min_occupation_ticks(value.get_as_i32()?),
                "TicksInHive" => bee_builder.with_ticks_in_hive(value.get_as_i32()?),
                _ => &mut bee_builder
            };
        }
        bee_builder.try_build().map_err(BlockEntityMissingDataError::from).map_err(MissingData::from).map_err(crate::nbt::Error::from)
    }).collect::<Result<Vec<_>,_>>()
}

fn parse_flower_pos(nbt_flower_pos: Tag) -> Result<FlowerPos, crate::nbt::Error> {
    let mut flower_pos_builder = FlowerPosBuilder::default();
    let nbt_flower_pos = nbt_flower_pos.get_as_map()?;
    for (key, value) in nbt_flower_pos {
        match key.as_str() {
            "X" => flower_pos_builder.with_x(value.get_as_i32()?),
            "Y" => flower_pos_builder.with_y(value.get_as_i32()?),
            "Z" => flower_pos_builder.with_z(value.get_as_i32()?),
            _ => &mut flower_pos_builder
        };
    }
    let flower_pos = flower_pos_builder.try_build().map_err(BlockEntityMissingDataError::from).map_err(MissingData::from)?;
    Ok(flower_pos)
}

fn parse_beacon(nbt_data: HashMap<String, Tag>) -> Result<Beacon, crate::nbt::Error> {
    let mut bb = BeaconBuilder::default();
    for (key, value) in nbt_data {
        match key.as_str() {
            "CustomName" => bb.with_custom_name(value.get_as_string()?),
            "Lock" => bb.with_lock(value.get_as_string()?),
            "Primary" => bb.with_primary(value.get_as_i32()?),
            "Secondary" => bb.with_secondary(value.get_as_i32()?),
            _ => &mut bb
        };
    }
    let b = bb.try_build().map_err(BlockEntityMissingDataError::from).map_err(MissingData::from)?;
    Ok(b)
}

fn parse_barrel(nbt_data: HashMap<String, Tag>) -> Result<Barrel, crate::nbt::Error> {
    let mut bb = BarrelBuilder::default();
    parse_inventory_block_entity(&mut bb, nbt_data)?;
    let b = bb.try_build().map_err(BlockEntityMissingDataError::from).map_err(MissingData::from)?;
    Ok(b)
}

fn parse_inventory_block_entity(builder: &mut impl InventoryBlockEntityBuilder, nbt_data: HashMap<String, Tag>) -> Result<(), crate::nbt::Error> {
    for (key, value) in nbt_data {
        match key.as_str() {
            "CustomName" => builder.set_custom_name(value.get_as_string()?),
            "Items" => builder.set_items(parse_items_with_slot(value)?),
            "Lock" => builder.set_lock(value.get_as_string()?),
            "LootTalbe" => builder.set_loot_table(value.get_as_string()?),
            "LootTableSeed" => builder.set_loot_table_seed(value.get_as_i64()?),
            _ => {}
        };
    }
    Ok(())
}

fn parse_items_with_slot(nbt_items: Tag) -> Result<Vec<ItemWithSlot>, crate::nbt::Error> {
    let nbt_items = nbt_items.get_as_list()?;
    nbt_items.into_iter().map(|item| {
        let item = item.get_as_map()?;
        let mut iwsb = ItemWithSlotBuilder::default();
        let mut ib = ItemBuilder::default();
        for (key, value) in item {
            match key.as_str() {
                "Count" => ib.set_count(value.get_as_i8()?),
                "Slot" => iwsb.set_slot(value.get_as_i8()?),
                "id" => ib.set_id(value.get_as_string()?),
                "tag" => ib.set_tag(value.get_as_map()?),
                _ => {}
            }
        }
        let item = ib.try_build().map_err(BlockEntityMissingDataError::from).map_err(MissingData::from)?;
        iwsb.set_item(item);
        iwsb.try_build().map_err(BlockEntityMissingDataError::from).map_err(MissingData::from).map_err(crate::nbt::Error::from)
    }).collect()
}

fn parse_banner(data: HashMap<String, Tag>) -> Result<Banner, crate::nbt::Error> {
    let mut bb = BannerBuilder::default();
    for (key, value) in data {
        match key.as_str() {
            "CustomName" => bb.with_custom_name(value.get_as_string()?),
            "Patterns" => bb.with_patterns(parse_banner_patterns(value)?),
            _ => &mut bb
        };
    }
    let banner = bb.try_build().map_err(BlockEntityMissingDataError::from).map_err(MissingData::from)?;
    Ok(banner)
}

fn parse_banner_patterns(value: Tag) -> Result<Vec<BannerPattern>, crate::nbt::Error> {
    let nbt_patterns = value.get_as_list()?;
    nbt_patterns.into_iter().map(|nbt_pattern| {
        let nbt_pattern = nbt_pattern.get_as_map()?;
        let mut pb = BannerPatternBuilder::default();
        for (key, value) in nbt_pattern {
            match key.as_str() {
                "Color" => pb.with_color(value.get_as_i32()?),
                "Pattern" => pb.with_pattern(value.get_as_string()?),
                _ => &mut pb
            };
        }
        pb.try_build().map_err(BlockEntityMissingDataError::from).map_err(MissingData::from).map_err(crate::nbt::Error::from)
    }).collect()
}

fn parse_generic_block_entity(nbt_data: HashMap<String, Tag>) -> Result<(BlockEntityBuilder, HashMap<String, Tag>), crate::nbt::Error> {
    let mut beb = BlockEntityBuilder::default();
    let mut remaining_nbt_data = HashMap::new();
    for (key, value) in nbt_data {
        match key.as_str() {
            "id" => beb.with_id(value.get_as_string()?),
            "keepPacked" => beb.with_keep_packed(value.get_as_i8()? == 1),
            "x" => beb.with_x(value.get_as_i32()?),
            "y" => beb.with_y(value.get_as_i32()?),
            "z" => beb.with_z(value.get_as_i32()?),
            _ => {
                remaining_nbt_data.insert(key, value);
                &mut beb
            }
        };
    }
    Ok((beb, remaining_nbt_data))
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
