use std::collections::HashMap;

use thiserror::Error;

use crate::{
    nbt::Tag,
    data::{block_entity::*, chunk::MissingData},
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
            return Err(crate::nbt::Error::InvalidValue);
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
            "minecraft:banners" => BlockEntityType::Banner(nbt_data.try_into()?),
            "minecraft:barrel" => BlockEntityType::Barrel(nbt_data.try_into()?),
            "minecraft:beacon" => BlockEntityType::Beacon(nbt_data.try_into()?),
            "minecraft:bed" => BlockEntityType::Bed,
            "minecraft:beehive" => BlockEntityType::Beehive(nbt_data.try_into()?),
            "minecraft:bell" => BlockEntityType::Bell,
            "minecraft:blast_furnace" => BlockEntityType::BlastFurnace(nbt_data.try_into()?),
            "minecraft:brewing_stand" => BlockEntityType::BrewingStand(nbt_data.try_into()?),
            "minecraft:campfire" => BlockEntityType::Campfire(nbt_data.try_into()?),
            "minecraft:chiseled_bookshelf" => {
                BlockEntityType::ChiseledBookshelf(nbt_data.try_into()?)
            }
            "minecraft:chest" => BlockEntityType::Chest(nbt_data.try_into()?),
            "minecraft:comparator" => BlockEntityType::Comparator(nbt_data.try_into()?),
            "minecraft:command_block" => BlockEntityType::CommandBlock(nbt_data.try_into()?),
            "minecraft:conduit" => BlockEntityType::Conduit(nbt_data.try_into()?),
            "minecraft:daylight_detector" => BlockEntityType::DaylightDetector,
            "minecraft:dispenser" => BlockEntityType::Dispenser(nbt_data.try_into()?),
            "minecraft:dropper" => BlockEntityType::Dropper(nbt_data.try_into()?),
            "minecraft:enchanting_table" => BlockEntityType::EnchantingTable(nbt_data.try_into()?),
            "minecraft:ender_chest" => BlockEntityType::EnderChest,
            "minecraft:end_gateway" => BlockEntityType::EndGateway(nbt_data.try_into()?),
            "minecraft:end_portal" => BlockEntityType::EndPortal,
            "minecraft:furnace" => BlockEntityType::Furnace(nbt_data.try_into()?),
            "minecraft:hopper" => BlockEntityType::Hopper(nbt_data.try_into()?),
            "minecraft:jigsaw" => BlockEntityType::Jigsaw(nbt_data.try_into()?),
            "minecraft:jukebox" => BlockEntityType::Jukebox(nbt_data.try_into()?),
            "minecraft:lectern" => BlockEntityType::Lectern(nbt_data.try_into()?),
            "minecraft:mob_spawner" => BlockEntityType::MobSpawner(nbt_data.try_into()?),
            "minecraft:piston" => BlockEntityType::Piston(nbt_data.try_into()?),
            "minecraft:shulker_box" => BlockEntityType::ShulkerBox(nbt_data.try_into()?),
            "minecraft:sign" => BlockEntityType::Sign(nbt_data.try_into()?),
            "minecraft:skull" => BlockEntityType::Skull(nbt_data.try_into()?),
            "minecraft:smoker" => BlockEntityType::Smoker(nbt_data.try_into()?),
            "minecraft:soul_campfire" => BlockEntityType::SoulCampfire(nbt_data.try_into()?),
            "minecraft:structure_block" => BlockEntityType::StructureBlock(nbt_data.try_into()?),
            "minecraft:trapped_chest" => BlockEntityType::TrappedChest(nbt_data.try_into()?),
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

try_from_tag_for_module![{Beehive => [
    "Bees": set_bees,
    "FlowerPos": set_flower_pos
]
},{BeeInHive => [
    "EntityData": set_entity_data,
    "MinOccupationTicks": set_min_occupation_ticks,
    "TicksInHive": set_ticks_in_hive
]
},{FlowerPos => [
    "X": set_x,
    "Y": set_y,
    "Z": set_z
]
},{Beacon => [
    "CustomName": set_custom_name,
    "Lock": set_lock,
    "Primary": set_primary,
    "Secondary": set_secondary
]
},{Barrel => parse_inventory_block_entity
},{Item => [
    "Count": set_count,
    "id": set_id,
    "tag": set_tag
]
},{Banner => [
    "CustomName": set_custom_name,
    "Patterns": set_patterns
]
},{BannerPattern => [
    "Color": set_color,
    "Pattern": set_pattern
]
},{BlastFurnace => parse_cooking_block_entity
},{BrewingStand => [
    "BrewTime": set_brew_time,
    "CustomName": set_custom_name,
    "Fuel": set_fuel,
    "Items": set_items,
    "Lock": set_lock
]
},{Campfire => [
    "CookingTimes": set_cooking_times,
    "CookingTotalTimes": set_cooking_total_times,
    "Items": set_items
]
},{ChiseledBookshelf => [
    "Items": set_items,
    "last_interacted_slot": set_last_interacted_slot
]
},{Chest => parse_inventory_block_entity
},{Comparator => [
    "OutputSignal": set_output_signal
]
},{CommandBlock => [
    "auto": set_auto,
    "Command": set_command,
    "conditionMet": set_condition_met,
    "CustomName": set_custom_name,
    "LastExecution": set_last_execution,
    "LastOutput": set_last_output,
    "powered": set_powered,
    "SuccessCount": set_success_count,
    "UpdateLastExecution": set_update_last_execution
]
},{Conduit => [
    "Target": set_target
]
},{Dispenser => parse_inventory_block_entity
},{Dropper => parse_inventory_block_entity
},{EnchantingTable => [
    "CustomName": set_custom_name
]
},{EndGateway => [
    "Age": set_age,
    "ExactTeleport": set_exact_teleport,
    "ExitPortal": set_exit_portal
]
},{ExitPortal => [
    "X": set_x,
    "Y": set_y,
    "Z": set_z
]
},{Furnace => parse_cooking_block_entity
},{Hopper => parse_inventory_block_entity
},{Jigsaw => [
    "final_state": set_final_state,
    "joint": set_joint,
    "name": set_name,
    "pool": set_pool,
    "target": set_target
]
},{Jukebox => [
    "IsPlaying": set_is_playing,
    "RecordItem": set_record_item,
    "RecordStartTick": set_record_start_tick,
    "TickCount": set_tick_count
]
},{Lectern => [
    "Book": set_book,
    "Page": set_page
]
},{Spawner => [
    "Delay": set_delay,
    "MaxNearbyEntities": set_max_nearby_entities,
    "MaxSpawnDelay": set_max_spawn_delay,
    "MinSpawnDelay": set_min_spawn_delay,
    "RequiredPlayerRange": set_required_player_range,
    "SpawnCount": set_spawn_count,
    "SpawnData": set_spawn_data,
    "SpawnPotentials": set_spawn_potentials,
    "SpawnRange": set_spawn_range
]
},{PotentialSpawn => [
    "weight": set_weight,
    "data": set_data
]
},{Piston => [
    "blockState": set_block_state,
    "extending": set_extending,
    "facing": set_facing,
    "progress": set_progress,
    "source": set_source
]
},{ShulkerBox => parse_inventory_block_entity
},{Sign => [
    "GlowingText": set_glowing_text,
    "Color": set_color,
    "Text1": set_text1,
    "Text2": set_text2,
    "Text3": set_text3,
    "Text4": set_text4
]
},{Skull => [
    "note_block_sound": set_note_block_sound,
    "ExtraType": set_extra_type,
    "SkullOwner": set_skull_owner
]
},{SkullOwner => [
    "Id": set_id,
    "Name": set_name,
    "Properties": set_properties
]
},{SkullOwnerProperties => [
    "textures": set_textures
]
},{SkullOwnerTextures => [
    "Value": set_value,
    "Signature": set_signature
]
},{Smoker => parse_cooking_block_entity
},{SoulCampfire => [
    "CookingTimes": set_cooking_times,
    "CookingTotalTimes": set_cooking_total_times,
    "Items": set_items
]
},{StructureBlock => [
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
]
},{TrappedChest => parse_inventory_block_entity}
];

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
        let r = value
            .get_as_map()?
            .into_iter()
            .map(|(k, v)| v.try_into().map(|v| (k, v)))
            .collect::<Result<HashMap<String, i32>, _>>()?;
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

#[derive(Debug, Error, PartialEq, Eq)]
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
