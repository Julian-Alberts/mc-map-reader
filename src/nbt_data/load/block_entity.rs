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
            "brewing_stand" => BlockEntityType::BrewingStand(nbt_data.try_into()?),
            "campfire" => BlockEntityType::Campfire(nbt_data.try_into()?),
            "chiseled_bookshelf" => BlockEntityType::ChiseledBookshelf(nbt_data.try_into()?),
            "chest" => BlockEntityType::Chest(nbt_data.try_into()?),
            "comparator" => BlockEntityType::Comparator(nbt_data.try_into()?),
            "command_block" => BlockEntityType::CommandBlock(nbt_data.try_into()?),
            "conduit" => BlockEntityType::Conduit(nbt_data.try_into()?),
            "daylight_detector" => BlockEntityType::DaylightDetector,
            "dispenser" => BlockEntityType::Dispenser(nbt_data.try_into()?),
            "dropper" => BlockEntityType::Dropper(nbt_data.try_into()?),
            "enchanting_table" => BlockEntityType::EnchantingTable(nbt_data.try_into()?),
            "ender_chest" => BlockEntityType::EnderChest,
            "end_gateway" => BlockEntityType::EndGateway(nbt_data.try_into()?),
            "end_portal" => BlockEntityType::EndPortal,
            "furnace" => BlockEntityType::Furnace(nbt_data.try_into()?),
            "hopper" => BlockEntityType::Hopper(nbt_data.try_into()?),
            "jigsaw" => BlockEntityType::Jigsaw(nbt_data.try_into()?),
            "jukebox" => BlockEntityType::Jukebox(nbt_data.try_into()?),
            "lectern" => BlockEntityType::Lectern(nbt_data.try_into()?),
            "mob_spawner" => BlockEntityType::MobSpawner(nbt_data.try_into()?),	
            "piston" => BlockEntityType::Piston(nbt_data.try_into()?),
            "shulker_box" => BlockEntityType::ShulkerBox(nbt_data.try_into()?),
            "sign" => BlockEntityType::Sign(nbt_data.try_into()?),
            "skull" => BlockEntityType::Skull(nbt_data.try_into()?),
            "smoker" => BlockEntityType::Smoker(nbt_data.try_into()?),
            "soul_campfire" => BlockEntityType::SoulCampfire(nbt_data.try_into()?),
            "structure_block" => BlockEntityType::StructureBlock(nbt_data.try_into()?),
            "trapped_chest" => BlockEntityType::TrappedChest(nbt_data.try_into()?),
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
    fn try_from(nbt_data: HashMap<String, Tag>) -> Result<Self, Self::Error> {
        let mut bb = BlastFurnaceBuilder::default();
        parse_cooking_block_entity(&mut bb, nbt_data)?;
        let b = bb
            .try_build()
            .map_err(BlockEntityMissingDataError::from)
            .map_err(MissingData::from)?;
        Ok(b)
    }
}

impl TryFrom<HashMap<String, Tag>> for BrewingStand {
    type Error = crate::nbt::Error;
    fn try_from(mut nbt_data: HashMap<String, Tag>) -> Result<Self, Self::Error> {
        let mut brewing_stand_builder = BrewingStandBuilder::default();
        add_data_to_builder!(brewing_stand_builder, nbt_data => [
            "BrewTime": set_brew_time,
            "CustomName": set_custom_name,
            "Fuel": set_fuel,
            "Items": set_items,
            "Lock": set_lock
        ]);
        let b = brewing_stand_builder
            .try_build()
            .map_err(BlockEntityMissingDataError::from)
            .map_err(MissingData::from)?;
        Ok(b)
    }
}

impl TryFrom<HashMap<String, Tag>> for Campfire {
    type Error = crate::nbt::Error;
    fn try_from(mut nbt_data: HashMap<String, Tag>) -> Result<Self, Self::Error> {
        let mut campfireBuilder = CampfireBuilder::default();
        add_data_to_builder!(campfireBuilder, nbt_data => [
            "CookingTimes": set_cooking_times,
            "CookingTotalTimes": set_cooking_total_times,
            "Items": set_items
        ]);
        let c = campfireBuilder
            .try_build()
            .map_err(BlockEntityMissingDataError::from)
            .map_err(MissingData::from)?;
        Ok(c)
    }
}

impl TryFrom<HashMap<String, Tag>> for ChiseledBookshelf {
    type Error = crate::nbt::Error;
    fn try_from(mut nbt_data: HashMap<String, Tag>) -> Result<Self, Self::Error> {
        let mut chiseled_bookshelf_builder = ChiseledBookshelfBuilder::default();
        add_data_to_builder!(chiseled_bookshelf_builder, nbt_data => [
            "Items": set_items,
            "last_interacted_slot": set_last_interacted_slot
        ]);
        let c = chiseled_bookshelf_builder
            .try_build()
            .map_err(BlockEntityMissingDataError::from)
            .map_err(MissingData::from)?;
        Ok(c)
    }
}

impl TryFrom<HashMap<String, Tag>> for Chest {
    type Error = crate::nbt::Error;
    fn try_from(nbt_data: HashMap<String, Tag>) -> Result<Self, Self::Error> {
        let mut chest_builder = ChestBuilder::default();
        parse_inventory_block_entity(&mut chest_builder, nbt_data)?;
        let c = chest_builder
            .try_build()
            .map_err(BlockEntityMissingDataError::from)
            .map_err(MissingData::from)?;
        Ok(c)
    }
}

impl TryFrom<HashMap<String, Tag>> for Comparator {
    type Error = crate::nbt::Error;
    fn try_from(mut nbt_data: HashMap<String, Tag>) -> Result<Self, Self::Error> {
        let mut comparator_builder = ComparatorBuilder::default();
        add_data_to_builder!(comparator_builder, nbt_data => [
            "OutputSignal": set_output_signal
        ]);
        let c = comparator_builder
            .try_build()
            .map_err(BlockEntityMissingDataError::from)
            .map_err(MissingData::from)?;
        Ok(c)
    }
}

macro_rules! try_from_hash_map_for_block_entity {
    ($name:ident, $builder:ident => [$(
        $key:literal: $setter:ident
    ),*]) => {
        impl TryFrom<Tag> for $name {
            type Error = crate::nbt::Error;
            fn try_from(nbt_data: Tag) -> Result<Self, Self::Error> {
                let nbt_data = nbt_data.get_as_map()?;
                Self::try_from(nbt_data)
            }
        }
        impl TryFrom<HashMap<String, Tag>> for $name {
            type Error = crate::nbt::Error;
            fn try_from(mut nbt_data: HashMap<String, Tag>) -> Result<Self, Self::Error> {
                let mut builder = $builder::default();
                add_data_to_builder!(builder, nbt_data => [
                    $(
                        $key: $setter
                    ),*
                ]);
                let b = builder
                    .try_build()
                    .map_err(BlockEntityMissingDataError::from)
                    .map_err(MissingData::from)?;
                Ok(b)
            }
        }
    };
    ($name:ident, $builder:ident => $fn:ident) => {
        impl TryFrom<HashMap<String, Tag>> for $name {
            type Error = crate::nbt::Error;
            fn try_from(nbt_data: HashMap<String, Tag>) -> Result<Self, Self::Error> {
                let mut builder = $builder::default();
                $fn(&mut builder, nbt_data)?;
                let b = builder
                    .try_build()
                    .map_err(BlockEntityMissingDataError::from)
                    .map_err(MissingData::from)?;
                Ok(b)
            }
        }
    };
}

try_from_hash_map_for_block_entity!(CommandBlock, CommandBlockBuilder => [
    "auto": set_auto,
    "Command": set_command,
    "conditionMet": set_condition_met,
    "CustomName": set_custom_name,
    "LastExecution": set_last_execution,
    "LastOutput": set_last_output,
    "powered": set_powered,
    "SuccessCount": set_success_count,
    "UpdateLastExecution": set_update_last_execution
]);
try_from_hash_map_for_block_entity!(Conduit, ConduitBuilder => [
    "Target": set_target
]);
try_from_hash_map_for_block_entity!(Dispenser, DispenserBuilder => parse_inventory_block_entity);
try_from_hash_map_for_block_entity!(Dropper, DropperBuilder => parse_inventory_block_entity);
try_from_hash_map_for_block_entity!(EnchantingTable, EnchantingTableBuilder => [
    "CustomName": set_custom_name
]);
try_from_hash_map_for_block_entity!(EndGateway, EndGatewayBuilder => [
    "Age": set_age,
    "ExactTeleport": set_exact_teleport,
    "ExitPortal": set_exit_portal
]);
try_from_hash_map_for_block_entity!(ExitPortal, ExitPortalBuilder => [
    "X": set_x,
    "Y": set_y,
    "Z": set_z
]);
try_from_hash_map_for_block_entity!(Furnace, FurnaceBuilder => parse_cooking_block_entity);
try_from_hash_map_for_block_entity!(Hopper, HopperBuilder => parse_inventory_block_entity);
try_from_hash_map_for_block_entity!(Jigsaw, JigsawBuilder => [
    "final_state": set_final_state,
    "joint": set_joint,
    "name": set_name,
    "pool": set_pool,
    "target": set_target
]);
try_from_hash_map_for_block_entity!(Jukebox, JukeboxBuilder => [
    "IsPlaying": set_is_playing,
    "RecordItem": set_record_item,
    "RecordStartTick": set_record_start_tick,
    "TickCount": set_tick_count
]);
try_from_hash_map_for_block_entity!(Lectern, LecternBuilder => [
    "Book": set_book,
    "Page": set_page
]);
impl TryFrom<HashMap<String, Tag>> for MobSpawner {
    type Error = crate::nbt::Error;
    fn try_from(nbt_data: HashMap<String, Tag>) -> Result<Self, Self::Error> {
        let mut mob_spawner_builder = MobSpawnerBuilder::default();
        mob_spawner_builder.set_spawner(nbt_data.try_into()?);
        let c = mob_spawner_builder
            .try_build()
            .map_err(BlockEntityMissingDataError::from)
            .map_err(MissingData::from)?;
        Ok(c)
    }
}
try_from_hash_map_for_block_entity!(Spawner, SpawnerBuilder => [
    "Delay": set_delay,
    "MaxNearbyEntities": set_max_nearby_entities,
    "MaxSpawnDelay": set_max_spawn_delay,
    "MinSpawnDelay": set_min_spawn_delay,
    "RequiredPlayerRange": set_required_player_range,
    "SpawnCount": set_spawn_count,
    "SpawnData": set_spawn_data,
    "SpawnPotentials": set_spawn_potentials,
    "SpawnRange": set_spawn_range
]);
try_from_hash_map_for_block_entity!(PotentialSpawn, PotentialSpawnBuilder => [
    "weight": set_weight,
    "data": set_data
]);
try_from_hash_map_for_block_entity!(Piston, PistonBuilder => [
    "blockState": set_block_state,
    "extending": set_extending,
    "facing": set_facing,
    "progress": set_progress,
    "source": set_source
]);
try_from_hash_map_for_block_entity!(ShulkerBox, ShulkerBoxBuilder => parse_inventory_block_entity);
try_from_hash_map_for_block_entity!(Sign, SignBuilder => [
    "GlowingText": set_glowing_text,
    "Color": set_color,
    "Text1": set_text1,
    "Text2": set_text2,
    "Text3": set_text3,
    "Text4": set_text4
]);
try_from_hash_map_for_block_entity!(Skull, SkullBuilder => [
    "note_block_sound": set_note_block_sound,
    "ExtraType": set_extra_type,
    "SkullOwner": set_skull_owner
]);
try_from_hash_map_for_block_entity!(SkullOwner, SkullOwnerBuilder => [
    "Id": set_id,
    "Name": set_name,
    "Properties": set_properties
]);
try_from_hash_map_for_block_entity!(SkullOwnerProperties, SkullOwnerPropertiesBuilder => [
    "textures": set_textures
]);
try_from_hash_map_for_block_entity!(SkullOwnerTextures, SkullOwnerTexturesBuilder => [
    "Value": set_value,
    "Signature": set_signature
]);
try_from_hash_map_for_block_entity!(Smoker, SmokerBuilder => parse_cooking_block_entity);
try_from_hash_map_for_block_entity!(SoulCampfire, SoulCampfireBuilder => [
    "CookingTimes": set_cooking_times,
    "CookingTotalTimes": set_cooking_total_times,
    "Items": set_items
]);
try_from_hash_map_for_block_entity!(StructureBlock, StructureBlockBuilder => [
    "author": set_author,
    "ignoreEntities": set_ignore_entities,
    "integrity": set_integrity,
    "metadata": set_metadata,
    "mirror": set_mirror,
    "mode": set_mode,
    "name": set_name,
    "posX": set_pos_x,
    "posY": set_pos_y,
    "posZ": set_pos_z,
    "powered": set_powered,
    "rotation": set_rotation,
    "seed": set_seed,
    "showboundingbox": set_show_bounding_box,
    "sizeX": set_size_x,
    "sizeY": set_size_y,
    "sizeZ": set_size_z
]);
try_from_hash_map_for_block_entity!(TrappedChest, TrappedChestBuilder => parse_inventory_block_entity);

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
        "Lock": set_lock
    ]);
    if let Some(value) = nbt_data.remove("RecipesUsed") {
        let r = value.get_as_map()?.into_iter().map(|(k,v)| v.try_into().and_then(|v|Ok((k,v)))).collect::<Result<HashMap<String, i32>,_>>()?;
        builder.set_recipes_used(r)
    }
    Ok(())
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
    BlastFurnace(#[from] BlastFurnaceBuilderError),
    #[error(transparent)]
    BrewingStand(#[from] BrewingStandBuilderError),
    #[error(transparent)]
    Campfire(#[from] CampfireBuilderError),
    #[error(transparent)]
    ChiseledBookshelf(#[from] ChiseledBookshelfBuilderError),
    #[error(transparent)]
    Chest(#[from] ChestBuilderError),
    #[error(transparent)]
    Comparator(#[from] ComparatorBuilderError),
    #[error(transparent)]
    CommandBlock(#[from] CommandBlockBuilderError),
    #[error(transparent)]
    Conduit(#[from] ConduitBuilderError),
    #[error(transparent)]
    Dispenser(#[from] DispenserBuilderError),
    #[error(transparent)]
    Dropper(#[from] DropperBuilderError),
    #[error(transparent)]
    EnchantingTable(#[from] EnchantingTableBuilderError),
    #[error(transparent)]
    EndGateway(#[from] EndGatewayBuilderError),
    #[error(transparent)]
    ExitPortal(#[from] ExitPortalBuilderError),
    #[error(transparent)]
    Furnace(#[from] FurnaceBuilderError),
    #[error(transparent)]
    Hopper(#[from] HopperBuilderError),
    #[error(transparent)]
    Jigsaw(#[from] JigsawBuilderError),
    #[error(transparent)]
    Jukebox(#[from] JukeboxBuilderError),
    #[error(transparent)]
    Lectern(#[from] LecternBuilderError),
    #[error(transparent)]
    MobSpawner(#[from] MobSpawnerBuilderError),
    #[error(transparent)]
    Spawner(#[from] SpawnerBuilderError),
    #[error(transparent)]
    PotentialSpawn(#[from] PotentialSpawnBuilderError),
    #[error(transparent)]
    Piston(#[from] PistonBuilderError),
    #[error(transparent)]
    ShulkerBox(#[from] ShulkerBoxBuilderError),
    #[error(transparent)]
    Sign(#[from] SignBuilderError),
    #[error(transparent)]
    Skull(#[from] SkullBuilderError),
    #[error(transparent)]
    SkullOwner(#[from] SkullOwnerBuilderError),
    #[error(transparent)]
    SkullOwnerProperties(#[from] SkullOwnerPropertiesBuilderError),
    #[error(transparent)]
    SkullOwnerTextures(#[from] SkullOwnerTexturesBuilderError),
    #[error(transparent)]
    Smoker(#[from] SmokerBuilderError),
    #[error(transparent)]
    SoulCampfire(#[from] SoulCampfireBuilderError),
    #[error(transparent)]
    StructureBlock(#[from] StructureBlockBuilderError),
    #[error(transparent)]
    TrappedChest(#[from] TrappedChestBuilderError),
}
