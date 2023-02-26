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

macro_rules! try_from_tag {
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

try_from_tag!(Beehive, BeehiveBuilder => [
    "Bees": set_bees,
    "FlowerPos": set_flower_pos
]);
try_from_tag!(BeeInHive, BeeInHiveBuilder => [
    "EntityData": set_entity_data,
    "MinOccupationTicks": set_min_occupation_ticks,
    "TicksInHive": set_ticks_in_hive
]);
try_from_tag!(FlowerPos, FlowerPosBuilder => [
    "X": set_x,
    "Y": set_y,
    "Z": set_z
]);
try_from_tag!(Beacon, BeaconBuilder => [
    "CustomName": set_custom_name,
    "Lock": set_lock,
    "Primary": set_primary,
    "Secondary": set_secondary
]);
try_from_tag!(Barrel, BarrelBuilder => parse_inventory_block_entity);
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
try_from_tag!(Item, ItemBuilder => [
    "Count": set_count,
    "id": set_id,
    "tag": set_tag
]);
try_from_tag!(Banner, BannerBuilder => [
    "CustomName": set_custom_name,
    "Patterns": set_patterns
]);
try_from_tag!(BannerPattern, BannerPatternBuilder => [
    "Color": set_color,
    "Pattern": set_pattern
]);
try_from_tag!(BlastFurnace, BlastFurnaceBuilder => parse_cooking_block_entity);
try_from_tag!(BrewingStand, BrewingStandBuilder => [
    "BrewTime": set_brew_time,
    "CustomName": set_custom_name,
    "Fuel": set_fuel,
    "Items": set_items,
    "Lock": set_lock
]);
try_from_tag!(Campfire, CampfireBuilder => [
    "CookingTimes": set_cooking_times,
    "CookingTotalTimes": set_cooking_total_times,
    "Items": set_items
]);
try_from_tag!(ChiseledBookshelf, ChiseledBookshelfBuilder => [
    "Items": set_items,
    "last_interacted_slot": set_last_interacted_slot
]);
try_from_tag!(Chest, ChestBuilder => parse_inventory_block_entity);
try_from_tag!(Comparator, ComparatorBuilder => [
    "OutputSignal": set_output_signal
]);
try_from_tag!(CommandBlock, CommandBlockBuilder => [
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
try_from_tag!(Conduit, ConduitBuilder => [
    "Target": set_target
]);
try_from_tag!(Dispenser, DispenserBuilder => parse_inventory_block_entity);
try_from_tag!(Dropper, DropperBuilder => parse_inventory_block_entity);
try_from_tag!(EnchantingTable, EnchantingTableBuilder => [
    "CustomName": set_custom_name
]);
try_from_tag!(EndGateway, EndGatewayBuilder => [
    "Age": set_age,
    "ExactTeleport": set_exact_teleport,
    "ExitPortal": set_exit_portal
]);
try_from_tag!(ExitPortal, ExitPortalBuilder => [
    "X": set_x,
    "Y": set_y,
    "Z": set_z
]);
try_from_tag!(Furnace, FurnaceBuilder => parse_cooking_block_entity);
try_from_tag!(Hopper, HopperBuilder => parse_inventory_block_entity);
try_from_tag!(Jigsaw, JigsawBuilder => [
    "final_state": set_final_state,
    "joint": set_joint,
    "name": set_name,
    "pool": set_pool,
    "target": set_target
]);
try_from_tag!(Jukebox, JukeboxBuilder => [
    "IsPlaying": set_is_playing,
    "RecordItem": set_record_item,
    "RecordStartTick": set_record_start_tick,
    "TickCount": set_tick_count
]);
try_from_tag!(Lectern, LecternBuilder => [
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
try_from_tag!(Spawner, SpawnerBuilder => [
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
try_from_tag!(PotentialSpawn, PotentialSpawnBuilder => [
    "weight": set_weight,
    "data": set_data
]);
try_from_tag!(Piston, PistonBuilder => [
    "blockState": set_block_state,
    "extending": set_extending,
    "facing": set_facing,
    "progress": set_progress,
    "source": set_source
]);
try_from_tag!(ShulkerBox, ShulkerBoxBuilder => parse_inventory_block_entity);
try_from_tag!(Sign, SignBuilder => [
    "GlowingText": set_glowing_text,
    "Color": set_color,
    "Text1": set_text1,
    "Text2": set_text2,
    "Text3": set_text3,
    "Text4": set_text4
]);
try_from_tag!(Skull, SkullBuilder => [
    "note_block_sound": set_note_block_sound,
    "ExtraType": set_extra_type,
    "SkullOwner": set_skull_owner
]);
try_from_tag!(SkullOwner, SkullOwnerBuilder => [
    "Id": set_id,
    "Name": set_name,
    "Properties": set_properties
]);
try_from_tag!(SkullOwnerProperties, SkullOwnerPropertiesBuilder => [
    "textures": set_textures
]);
try_from_tag!(SkullOwnerTextures, SkullOwnerTexturesBuilder => [
    "Value": set_value,
    "Signature": set_signature
]);
try_from_tag!(Smoker, SmokerBuilder => parse_cooking_block_entity);
try_from_tag!(SoulCampfire, SoulCampfireBuilder => [
    "CookingTimes": set_cooking_times,
    "CookingTotalTimes": set_cooking_total_times,
    "Items": set_items
]);
try_from_tag!(StructureBlock, StructureBlockBuilder => [
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
try_from_tag!(TrappedChest, TrappedChestBuilder => parse_inventory_block_entity);

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
