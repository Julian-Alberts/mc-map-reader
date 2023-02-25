use std::collections::HashMap;

use thiserror::Error;

use crate::{
    nbt::Tag,
    nbt_data::{block_entity::*, chunk::MissingData},
};

impl TryFrom<Tag> for BlockEntity {
    type Error = crate::nbt::Error;
    fn try_from(data: Tag) -> Result<Self, Self::Error> {
        let mut nbt_data = data.get_as_map()?;
        let Tag::String(id) = nbt_data
        .get("id")
        .ok_or(BlockEntityBuilderError::UnsetId)
        .map_err(BlockEntityMissingDataError::from)
        .map_err(MissingData::from)? else {
            return Err(crate::nbt::Error::InvalidValue.into());
        };
        let id = id.clone();
        let mut beb = BlockEntityBuilder::default();
        add_data_to_builder!(beb, nbt_data => [
            "id": set_id,
            "keepPacked": set_keep_packed,
            "x": set_x,
            "y": set_y,
            "z": set_z
        ]);

        let ty = match id.as_str() {
            "banners" => BlockEntityType::Banner(nbt_data.try_into()?),
            "barrel" => BlockEntityType::Barrel(nbt_data.try_into()?),
            "beacon" => BlockEntityType::Beacon(nbt_data.try_into()?),
            "bed" => BlockEntityType::Bed,
            "beehive" => BlockEntityType::Beehive(nbt_data.try_into()?),
            "bell" => BlockEntityType::Bell,
            "blast_furnace" => BlockEntityType::BlastFurnace(nbt_data.try_into()?),
            _ => BlockEntityType::Other(nbt_data),
        };
        beb.set_entity_type(ty);
        let be = beb
            .try_build()
            .map_err(BlockEntityMissingDataError::from)
            .map_err(MissingData::from)?;
        Ok(be)
    }
}

impl TryFrom<Tag> for Beehive {
    type Error = crate::nbt::Error;
    fn try_from(value: Tag) -> Result<Self, Self::Error> {
        value.get_as_map()?.try_into()
    }
}

impl TryFrom<HashMap<String, Tag>> for Beehive {
    type Error = crate::nbt::Error;
    fn try_from(mut nbt_data: HashMap<String, Tag>) -> Result<Self, Self::Error> {
        let mut beehive_builder = BeehiveBuilder::default();
        add_data_to_builder!(beehive_builder, nbt_data => [
            "Bees": set_bees,
            "FlowerPos": set_flower_pos
        ]);
        let beehive = beehive_builder
            .try_build()
            .map_err(BlockEntityMissingDataError::from)
            .map_err(MissingData::from)?;
        Ok(beehive)
    }
}

impl TryFrom<Tag> for BeeInHive {
    type Error = crate::nbt::Error;
    fn try_from(nbt_bee: Tag) -> Result<Self, Self::Error> {
        let nbt_bee = nbt_bee.get_as_map()?;
        let mut bee_builder = BeeInHiveBuilder::default();
        for (key, value) in nbt_bee {
            match key.as_str() {
                "EntityData" => bee_builder.with_entity_data(value.try_into()?),
                "MinOccupationTicks" => bee_builder.with_min_occupation_ticks(value.get_as_i32()?),
                "TicksInHive" => bee_builder.with_ticks_in_hive(value.get_as_i32()?),
                _ => &mut bee_builder,
            };
        }
        bee_builder
            .try_build()
            .map_err(BlockEntityMissingDataError::from)
            .map_err(MissingData::from)
            .map_err(crate::nbt::Error::from)
    }
}

impl TryFrom<Tag> for FlowerPos {
    type Error = crate::nbt::Error;
    fn try_from(nbt_flower_pos: Tag) -> Result<Self, Self::Error> {
        let mut flower_pos_builder = FlowerPosBuilder::default();
        let mut nbt_flower_pos = nbt_flower_pos.get_as_map()?;
        add_data_to_builder!(flower_pos_builder, nbt_flower_pos => [
            "X": set_x,
            "Y": set_y,
            "Z": set_z
        ]);
        let flower_pos = flower_pos_builder
            .try_build()
            .map_err(BlockEntityMissingDataError::from)
            .map_err(MissingData::from)?;
        Ok(flower_pos)
    }
}

impl TryFrom<Tag> for Beacon {
    type Error = crate::nbt::Error;
    fn try_from(value: Tag) -> Result<Self, Self::Error> {
        value.get_as_map()?.try_into()
    }
}

impl TryFrom<HashMap<String, Tag>> for Beacon {
    type Error = crate::nbt::Error;
    fn try_from(mut nbt_data: HashMap<String, Tag>) -> Result<Self, Self::Error> {
        let mut bb = BeaconBuilder::default();
        add_data_to_builder!(bb, nbt_data => [
            "CustomName": set_custom_name,
            "Lock": set_lock,
            "Primary": set_primary,
            "Secondary": set_secondary
        ]);
        let b = bb
            .try_build()
            .map_err(BlockEntityMissingDataError::from)
            .map_err(MissingData::from)?;
        Ok(b)
    }
}

impl TryFrom<Tag> for Barrel {
    type Error = crate::nbt::Error;
    fn try_from(value: Tag) -> Result<Self, Self::Error> {
        value.get_as_map()?.try_into()
    }
}

impl TryFrom<HashMap<String, Tag>> for Barrel {
    type Error = crate::nbt::Error;
    fn try_from(nbt_data: HashMap<String, Tag>) -> Result<Self, Self::Error> {
        let mut bb = BarrelBuilder::default();
        parse_inventory_block_entity(&mut bb, nbt_data)?;
        let b = bb
            .try_build()
            .map_err(BlockEntityMissingDataError::from)
            .map_err(MissingData::from)?;
        Ok(b)
    }
}

fn parse_inventory_block_entity(
    builder: &mut impl InventoryBlockEntityBuilder,
    mut nbt_data: HashMap<String, Tag>,
) -> Result<(), crate::nbt::Error> {
    add_data_to_builder!(builder, nbt_data => [
        "CustomName": set_custom_name,
        "Items": set_items,
        "Lock": set_lock,
        "LootTable": set_loot_table,
        "LootTableSeed": set_loot_table_seed
    ]);
    Ok(())
}

impl TryFrom<Tag> for ItemWithSlot {
    type Error = crate::nbt::Error;
    fn try_from(item: Tag) -> Result<Self, Self::Error> {
        let mut item = item.get_as_map()?;
        let mut iwsb = ItemWithSlotBuilder::default();

        add_data_to_builder!(iwsb, item => [
            "Slot": set_slot
        ]);
        iwsb.set_item(item.try_into()?);
        iwsb.try_build()
            .map_err(BlockEntityMissingDataError::from)
            .map_err(MissingData::from)
            .map_err(crate::nbt::Error::from)
    }
}

impl TryFrom<Tag> for Item {
    type Error = crate::nbt::Error;
    fn try_from(item: Tag) -> Result<Self, Self::Error> {
        item.get_as_map()?.try_into()
    }
}

impl TryFrom<HashMap<String, Tag>> for Item {
    type Error = crate::nbt::Error;
    fn try_from(mut item: HashMap<String, Tag>) -> Result<Self, Self::Error> {
        let mut ib = ItemBuilder::default();
        add_data_to_builder!(ib, item => [
            "Count": set_count,
            "id": set_id,
            "tag": set_tag
        ]);
        ib.try_build()
            .map_err(BlockEntityMissingDataError::from)
            .map_err(MissingData::from)
            .map_err(crate::nbt::Error::from)
    }
}

impl TryFrom<Tag> for Banner {
    type Error = crate::nbt::Error;
    fn try_from(banner: Tag) -> Result<Self, Self::Error> {
        banner.get_as_map()?.try_into()
    }
}

impl TryFrom<HashMap<String, Tag>> for Banner {
    type Error = crate::nbt::Error;
    fn try_from(mut banner: HashMap<String, Tag>) -> Result<Self, Self::Error> {
        let mut bb = BannerBuilder::default();
        add_data_to_builder!(bb, banner => [
            "CustomName": set_custom_name,
            "Patterns": set_patterns
        ]);
        let banner = bb
            .try_build()
            .map_err(BlockEntityMissingDataError::from)
            .map_err(MissingData::from)?;
        Ok(banner)
    }
}

impl TryFrom<Tag> for BannerPattern {
    type Error = crate::nbt::Error;
    fn try_from(nbt_pattern: Tag) -> Result<Self, Self::Error> {
        let mut nbt_pattern = nbt_pattern.get_as_map()?;
        let mut pb = BannerPatternBuilder::default();
        add_data_to_builder!(pb, nbt_pattern => [
            "Color": set_color,
            "Pattern": set_pattern
        ]);
        pb.try_build()
            .map_err(BlockEntityMissingDataError::from)
            .map_err(MissingData::from)
            .map_err(crate::nbt::Error::from)
    }
}

impl TryFrom<HashMap<String, Tag>> for BlastFurnace {
    type Error = crate::nbt::Error;
    fn try_from(mut nbt_data: HashMap<String, Tag>) -> Result<Self, Self::Error> {
        let mut bb = BlastFurnaceBuilder::default();
        parse_cooking_block_entity(&mut bb, nbt_data)?;
        let b = bb
            .try_build()
            .map_err(BlockEntityMissingDataError::from)
            .map_err(MissingData::from)?;
        Ok(b)
    }
}

fn parse_cooking_block_entity(
    builder: &mut impl CookingBlockEntityBuilder,
    mut nbt_data: HashMap<String, Tag>,
) -> Result<(), crate::nbt::Error> {
    add_data_to_builder!(builder, nbt_data => [
        "BurnTime": set_burn_time,
        "CookTime": set_cook_time,
        "CookTimeTotal": set_cook_time_total,
        "CustomName": set_custom_name,
        "Items": set_items,
        "Lock": set_lock,
        "RecipesUsed": set_recipes_used
    ]);
    Ok(())
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
    #[error(transparent)]
    BlastFurnace(#[from] BlastFurnaceBuilderError)
}
