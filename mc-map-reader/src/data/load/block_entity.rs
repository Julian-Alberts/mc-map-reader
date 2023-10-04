use std::collections::HashMap;

use crate::{
    data::{block_entity::*, load::entity::EntityError, FieldError},
    nbt::Tag,
};

use crate::data::load::item::*;

mod_try_from_tag!(
    BlockEntity: parse_block_entity ? [
        Banner,
        Barrel,
        Beacon,
        Beehive,
        BlastFurnace,
        BrewingStand,
        Campfire,
        ChiseledBookshelf,
        Chest,
        Comparator,
        CommandBlock,
        Conduit,
        Dispenser,
        Dropper,
        EnchantingTable,
        EndGateway,
        Furnace,
        Hopper,
        Jigsaw,
        Jukebox,
        Lectern,
        MobSpawner,
        Piston,
        ShulkerBox,
        Sign,
        Skull,
        Smoker,
        SoulCampfire,
        StructureBlock,
        TrappedChest,
    ],
    Beehive: [
        "Bees" => set_bees test(List::from(vec![BeeInHive_test_data_provider().into()]) => bees = Some(List::from(vec![BeeInHive_test_result()]))),
        "FlowerPos" => set_flower_pos test(FlowerPos_test_data_provider() => flower_pos = Some(FlowerPos_test_result())),
    ] ? [
        BeeInHive,
        FlowerPos,
    ],
    BeeInHive: [
        "EntityData" => set_entity_data test(crate::data::load::entity::macro_tests::Entity_test_data_provider() => entity_data = crate::data::load::entity::macro_tests::Entity_test_result(); BeeInHiveBuilderError::UnsetEntityData),
        "MinOccupationTicks" => set_min_occupation_ticks test(1i32 => min_occupation_ticks = 1; BeeInHiveBuilderError::UnsetMinOccupationTicks),
        "TicksInHive" => set_ticks_in_hive test(1i32 => ticks_in_hive = 1; BeeInHiveBuilderError::UnsetTicksInHive),
    ] ? [
        Entity,
    ],
    FlowerPos: [
        "X" => set_x test(1i32 => x = 1; FlowerPosBuilderError::UnsetX),
        "Y" => set_y test(1i32 => y = 1; FlowerPosBuilderError::UnsetY),
        "Z" => set_z test(1i32 => z = 1; FlowerPosBuilderError::UnsetZ),
    ],
    Beacon: [
        "CustomName" => set_custom_name test("CustomName".to_string() => custom_name = Some("CustomName".to_string())),
        "Lock" => set_lock test("lock".to_string() => lock = Some("lock".to_string())),
        "Primary" => set_primary test(1i32 => primary = 1; BeaconBuilderError::UnsetPrimary),
        "Secondary" => set_secondary test(1i32 => secondary = 1; BeaconBuilderError::UnsetSecondary),
        "Levels" => set_levels test(1i32 => levels = 1; BeaconBuilderError::UnsetLevels),
    ],
    Barrel: parse_inventory_block_entity ? [ ItemWithSlot, ],
    Banner: [
        "CustomName" => set_custom_name test("CustomName".to_string() => custom_name = Some("CustomName".to_string())),
        "Patterns" => set_patterns test(List::from(vec![BannerPattern_test_data_provider().into()]) => patterns = Some(List::from(vec![BannerPattern_test_result()]))),
    ] ? [
        BannerPattern,
    ],
    BannerPattern: [
        "Color" => set_color test(1i32 => color = 1; BannerPatternBuilderError::UnsetColor),
        "Pattern" => set_pattern test("Pattern".to_string() => pattern = "Pattern".to_string(); BannerPatternBuilderError::UnsetPattern),
    ],
    BlastFurnace: parse_cooking_block_entity ? [ ItemWithSlot, ],
    BrewingStand: [
        "BrewTime" => set_brew_time test(1i16 => brew_time = 1; BrewingStandBuilderError::UnsetBrewTime),
        "CustomName" => set_custom_name test("CustomName".to_string() => custom_name = Some("CustomName".to_string())),
        "Fuel" => set_fuel test(1i8 => fuel = 1; BrewingStandBuilderError::UnsetFuel),
        "Items" => set_items test(List::from(vec![crate::data::load::item::tests::item_with_slot_test_data_provider().into()]) => items = Some(List::from(vec![crate::data::load::item::tests::item_with_slot_test_result()]))),
        "Lock" => set_lock test("Lock".to_string() => lock = Some("Lock".to_string())),
    ] ? [ ItemWithSlot, ],
    Campfire: [
        "CookingTimes" => set_cooking_times test(Array::from(vec![1_i32]) => cooking_times = Array::from(vec![1]); CampfireBuilderError::UnsetCookingTimes),
        "CookingTotalTimes" => set_cooking_total_times test(Array::from(vec![1_i32]) => cooking_total_times = Array::from(vec![1_i32]); CampfireBuilderError::UnsetCookingTotalTimes),
        "Items" => set_items test(List::from(vec![crate::data::load::item::tests::item_with_slot_test_data_provider().into()]) => items = Some(List::from(vec![crate::data::load::item::tests::item_with_slot_test_result()]))),
    ] ? [
        ItemWithSlot,
    ],
    ChiseledBookshelf: [
        "Items" => set_items test(List::from(vec![crate::data::load::item::tests::item_with_slot_test_data_provider().into()]) => items = Some(List::from(vec![crate::data::load::item::tests::item_with_slot_test_result()]))),
        "last_interacted_slot" => set_last_interacted_slot test(1i32 => last_interacted_slot = 1; ChiseledBookshelfBuilderError::UnsetLastInteractedSlot),
    ] ? [
        ItemWithSlot,
    ],
    Chest: parse_inventory_block_entity ? [ ItemWithSlot, ],
    Comparator: [
        "OutputSignal" => set_output_signal test(1i32 => output_signal = 1; ComparatorBuilderError::UnsetOutputSignal),
    ],
    CommandBlock: [
        "auto" => set_auto test(1i8 => auto = true; CommandBlockBuilderError::UnsetAuto),
        "Command" => set_command test("Command".to_string() => command = "Command".to_string(); CommandBlockBuilderError::UnsetCommand),
        "conditionMet" => set_condition_met test(1i8 => condition_met = true; CommandBlockBuilderError::UnsetConditionMet),
        "CustomName" => set_custom_name test("CustomName".to_string() => custom_name = Some("CustomName".to_string())),
        "LastExecution" => set_last_execution test(1i64 => last_execution = 1; CommandBlockBuilderError::UnsetLastExecution),
        "LastOutput" => set_last_output test("LastOutput".to_string() => last_output = "LastOutput".to_string(); CommandBlockBuilderError::UnsetLastOutput),
        "powered" => set_powered test(1i8 => powered = true; CommandBlockBuilderError::UnsetPowered),
        "SuccessCount" => set_success_count test(1i32 => success_count = 1; CommandBlockBuilderError::UnsetSuccessCount),
        "UpdateLastExecution" => set_update_last_execution test(1i8 => update_last_execution = true; CommandBlockBuilderError::UnsetUpdateLastExecution),
        "TrackOutput" => set_track_output test(1i8 => track_output = true; CommandBlockBuilderError::UnsetTrackOutput),
    ],
    Conduit: [
        "Target" => set_target test(Array::from(vec![10_i32,32]) => target = Array::from(vec![10_i32,32]); ConduitBuilderError::UnsetTarget),
    ],
    Dispenser: parse_inventory_block_entity ? [ ItemWithSlot, ],
    Dropper: parse_inventory_block_entity ? [ ItemWithSlot, ],
    EnchantingTable: [
        "CustomName" => set_custom_name test("CustomName".to_string() => custom_name = Some("CustomName".to_string())),
    ],
    EndGateway: [
        "Age" => set_age test(1i64 => age = 1; EndGatewayBuilderError::UnsetAge),
        "ExactTeleport" => set_exact_teleport test(1i8 => exact_teleport = true; EndGatewayBuilderError::UnsetExactTeleport),
        "ExitPortal" => set_exit_portal test(ExitPortal_test_data_provider() => exit_portal = ExitPortal_test_result(); EndGatewayBuilderError::UnsetExitPortal),
    ] ? [
        ExitPortal,
    ],
    ExitPortal: [
        "X" => set_x test(1 => x = 1; ExitPortalBuilderError::UnsetX),
        "Y" => set_y test(1 => y = 1; ExitPortalBuilderError::UnsetY),
        "Z" => set_z test(1 => z = 1; ExitPortalBuilderError::UnsetZ),
    ],
    Furnace: parse_cooking_block_entity ? [ ItemWithSlot, ],
    Hopper: parse_hopper ? [ ItemWithSlot, ],
    Jigsaw: [
        "final_state" => set_final_state test("final_state".to_string() => final_state = "final_state".to_string(); JigsawBuilderError::UnsetFinalState),
        "joint" => set_joint test("joint".to_string() => joint = "joint".to_string(); JigsawBuilderError::UnsetJoint),
        "name" => set_name test("name".to_string() => name = "name".to_string(); JigsawBuilderError::UnsetName),
        "pool" => set_pool test("pool".to_string() => pool = "pool".to_string(); JigsawBuilderError::UnsetPool),
        "target" => set_target test("target".to_string() => target = "target".to_string(); JigsawBuilderError::UnsetTarget),
    ],
    Jukebox: [
        "IsPlaying" => set_is_playing test(1i8 => is_playing = true; JukeboxBuilderError::UnsetIsPlaying),
        "RecordItem" => set_record_item test(crate::data::load::item::macro_tests::Item_test_data_provider() => record_item = crate::data::load::item::macro_tests::Item_test_result(); JukeboxBuilderError::UnsetRecordItem),
        "RecordStartTick" => set_record_start_tick test(1i64 => record_start_tick = 1; JukeboxBuilderError::UnsetRecordStartTick),
        "TickCount" => set_tick_count test(1i64 => tick_count = 1; JukeboxBuilderError::UnsetTickCount),
    ] ? [
        Item,
    ],
    Lectern: [
        "Book" => set_book test(crate::data::load::item::macro_tests::Item_test_data_provider() => book = Some(crate::data::load::item::macro_tests::Item_test_result())),
        "Page" => set_page test(1i32 => page = Some(1)),
    ] ? [
        Item,
    ],
    Spawner: [
        "Delay" => set_delay test(1i16 => delay = 1; SpawnerBuilderError::UnsetDelay),
        "MaxNearbyEntities" => set_max_nearby_entities test(1i16 => max_nearby_entities = 1; SpawnerBuilderError::UnsetMaxNearbyEntities),
        "MaxSpawnDelay" => set_max_spawn_delay test(1i16 => max_spawn_delay = 1; SpawnerBuilderError::UnsetMaxSpawnDelay),
        "MinSpawnDelay" => set_min_spawn_delay test(1i16 => min_spawn_delay = 1; SpawnerBuilderError::UnsetMinSpawnDelay),
        "RequiredPlayerRange" => set_required_player_range test(1i16 => required_player_range = 1; SpawnerBuilderError::UnsetRequiredPlayerRange),
        "SpawnCount" => set_spawn_count test(1i16 => spawn_count = 1; SpawnerBuilderError::UnsetSpawnCount),
        "SpawnData" => set_spawn_data test(HashMap::new() => spawn_data = HashMap::new(); SpawnerBuilderError::UnsetSpawnData),
        "SpawnPotentials" => set_spawn_potentials test(List::from(vec![PotentialSpawn_test_data_provider().into()]) => spawn_potentials = Some(List::from(vec![PotentialSpawn_test_result()]))),
        "SpawnRange" => set_spawn_range test(1i16 => spawn_range = 1; SpawnerBuilderError::UnsetSpawnRange),
    ] ? [
        PotentialSpawn,
    ],
    PotentialSpawn: [
        "weight" => set_weight test(1i32 => weight = 1; PotentialSpawnBuilderError::UnsetWeight),
        "data" => set_data test(HashMap::new() => data = HashMap::new(); PotentialSpawnBuilderError::UnsetData),
    ],
    Piston: [
        "blockState" => set_block_state test(PistonBlockState_test_data_provider() => block_state = PistonBlockState_test_result(); PistonBuilderError::UnsetBlockState),
        "extending" => set_extending test(1i8 => extending = true; PistonBuilderError::UnsetExtending),
        "facing" => set_facing test(2i32 => facing = 2; PistonBuilderError::UnsetFacing),
        "progress" => set_progress test(1f32 => progress = 1f32; PistonBuilderError::UnsetProgress),
        "source" => set_source test(1i8 => source = true; PistonBuilderError::UnsetSource),
    ] ? [
        PistonBlockState,
    ],
    PistonBlockState: [
        "Name" => set_name test("name".to_string() => name = "name".to_string(); PistonBlockStateBuilderError::UnsetName),
        "Properties" => set_properties test(HashMap::new() => properties = HashMap::new(); PistonBlockStateBuilderError::UnsetProperties),
    ],
    ShulkerBox: parse_inventory_block_entity ? [ ItemWithSlot, ],
    Sign: [
        "GlowingText" => set_glowing_text test(1i8 => glowing_text = true; SignBuilderError::UnsetGlowingText),
        "Color" => set_color test("color".to_string() => color = "color".to_string(); SignBuilderError::UnsetColor),
        "Text1" => set_text1 test("text1".to_string() => text1 = "text1".to_string(); SignBuilderError::UnsetText1),
        "Text2" => set_text2 test("text2".to_string() => text2 = "text2".to_string(); SignBuilderError::UnsetText2),
        "Text3" => set_text3 test("text3".to_string() => text3 = "text3".to_string(); SignBuilderError::UnsetText3),
        "Text4" => set_text4 test("text4".to_string() => text4 = "text4".to_string(); SignBuilderError::UnsetText4),
    ],
    Skull: [
        "note_block_sound" => set_note_block_sound test("sound".to_string() => note_block_sound = Some("sound".to_string())),
        "ExtraType" => set_extra_type test("extra_type".to_string() => extra_type = Some("extra_type".to_string())),
        "SkullOwner" => set_skull_owner test(SkullOwner_test_data_provider() => skull_owner = Some(SkullOwner_test_result())),
    ] ? [
        SkullOwner,
    ],
    SkullOwner: [
        "Id" => set_id test(Array::<i32>::from(vec![1,2,3,4]) => id = Array::from(vec![1,2,3,4]); SkullOwnerBuilderError::UnsetId),
        "Name" => set_name test("name".to_string() => name = Some("name".to_string())),
        "Properties" => set_properties test(List::from(vec![SkullOwnerProperties_test_data_provider().into()]) => properties = Some(List::from(vec![SkullOwnerProperties_test_result()]))),
    ] ? [
        SkullOwnerProperties,
    ],
    SkullOwnerProperties: [
        "textures" => set_textures test(List::from(vec![SkullOwnerTextures_test_data_provider().into()]) => textures = Some(List::from(vec![SkullOwnerTextures_test_result()]))),
    ] ? [
        SkullOwnerTextures,
    ],
    SkullOwnerTextures: [
        "Value" => set_value test("value".to_string() => value = "value".to_string(); SkullOwnerTexturesBuilderError::UnsetValue),
        "Signature" => set_signature test("signature".to_string() => signature = Some("signature".to_string())),
    ],
    Smoker: parse_cooking_block_entity ? [ ItemWithSlot, ],
    SoulCampfire: [
        "CookingTimes" => set_cooking_times test(Array::from(vec![1i32, 2i32]) => cooking_times = Array::from(vec![1i32, 2i32]); SoulCampfireBuilderError::UnsetCookingTimes),
        "CookingTotalTimes" => set_cooking_total_times test(Array::from(vec![1i32, 2i32]) => cooking_total_times = Array::from(vec![1i32, 2i32]); SoulCampfireBuilderError::UnsetCookingTotalTimes),
        "Items" => set_items test(List::from(
            vec![crate::data::load::item::tests::item_with_slot_test_data_provider().into()]
        ) => items = Some(List::from(vec![
            crate::data::load::item::tests::item_with_slot_test_result()
        ]))),
    ] ? [
        ItemWithSlot,
    ],
    StructureBlock: [
        "author" => set_author test("author".to_string() => author = "author".to_string(); StructureBlockBuilderError::UnsetAuthor),
        "ignoreEntities" => set_ignore_entities test(1i8 => ignore_entities = true; StructureBlockBuilderError::UnsetIgnoreEntities),
        "integrity" => set_integrity test(1f32 => integrity = 1.; StructureBlockBuilderError::UnsetIntegrity),
        "metadata" => set_metadata test("metadata".to_string() => metadata = "metadata".to_string(); StructureBlockBuilderError::UnsetMetadata),
        "mirror" => set_mirror test("test".to_string() => mirror = "test".to_string(); StructureBlockBuilderError::UnsetMirror),
        "mode" => set_mode test("mode".to_string() => mode = "mode".to_string(); StructureBlockBuilderError::UnsetMode),
        "name" => set_name test("name".to_string() => name = "name".to_string(); StructureBlockBuilderError::UnsetName),
        "posX" => set_pos_x test(1i32 => pos_x = 1; StructureBlockBuilderError::UnsetPosX),
        "posY" => set_pos_y test(1i32 => pos_y = 1; StructureBlockBuilderError::UnsetPosY),
        "posZ" => set_pos_z test(1i32 => pos_z = 1; StructureBlockBuilderError::UnsetPosZ),
        "powered" => set_powered test(1i8 => powered = true; StructureBlockBuilderError::UnsetPowered),
        "rotation" => set_rotation test("south".to_string() => rotation = "south".to_string(); StructureBlockBuilderError::UnsetRotation),
        "seed" => set_seed test(1i64 => seed = 1; StructureBlockBuilderError::UnsetSeed),
        "showboundingbox" => set_show_bounding_box test(1i8 => show_bounding_box = true; StructureBlockBuilderError::UnsetShowBoundingBox),
        "sizeX" => set_size_x test(1i32 => size_x = 1; StructureBlockBuilderError::UnsetSizeX),
        "sizeY" => set_size_y test(1i32 => size_y = 1; StructureBlockBuilderError::UnsetSizeY),
        "sizeZ" => set_size_z test(1i32 => size_z = 1; StructureBlockBuilderError::UnsetSizeZ),
    ],
    TrappedChest: parse_inventory_block_entity ? [ ItemWithSlot, ],
    MobSpawner: parse_mob_spawner ? [ Spawner, ],
);

fn parse_block_entity(
    builder: &mut BlockEntityBuilder,
    mut nbt_data: HashMap<String, Tag>,
) -> Result<(), BlockEntityError> {
    let Tag::String(id) = nbt_data.get("id").ok_or(BlockEntityBuilderError::UnsetId)? else {
        return Err(FieldError::new("id", crate::nbt::Error::InvalidValue).into());
    };
    let id = id.clone();
    add_data_to_builder!(builder, nbt_data => [
        "id": set_id,
        "keepPacked": set_keep_packed,
        "x": set_x,
        "y": set_y,
        "z": set_z,
    ]);

    const ENTITY_TYPE_KEY: &str = "<internal> entity_type";
    let ty = match id.as_str() {
        "minecraft:banners" => nbt_data
            .try_into()
            .map(BlockEntityType::Banner)
            .map_err(|e| FieldError::new(ENTITY_TYPE_KEY, e))?,
        "minecraft:barrel" => nbt_data
            .try_into()
            .map(BlockEntityType::Barrel)
            .map_err(|e| FieldError::new(ENTITY_TYPE_KEY, e))?,
        "minecraft:beacon" => nbt_data
            .try_into()
            .map(BlockEntityType::Beacon)
            .map_err(|e| FieldError::new(ENTITY_TYPE_KEY, e))?,
        "minecraft:bed" => BlockEntityType::Bed,
        "minecraft:beehive" => nbt_data
            .try_into()
            .map(BlockEntityType::Beehive)
            .map_err(|e| FieldError::new(ENTITY_TYPE_KEY, e))?,
        "minecraft:bell" => BlockEntityType::Bell,
        "minecraft:blast_furnace" => nbt_data
            .try_into()
            .map(BlockEntityType::BlastFurnace)
            .map_err(|e| FieldError::new(ENTITY_TYPE_KEY, e))?,
        "minecraft:brewing_stand" => nbt_data
            .try_into()
            .map(BlockEntityType::BrewingStand)
            .map_err(|e| FieldError::new(ENTITY_TYPE_KEY, e))?,
        "minecraft:campfire" => nbt_data
            .try_into()
            .map(BlockEntityType::Campfire)
            .map_err(|e| FieldError::new(ENTITY_TYPE_KEY, e))?,
        "minecraft:chiseled_bookshelf" => nbt_data
            .try_into()
            .map(BlockEntityType::ChiseledBookshelf)
            .map_err(|e| FieldError::new(ENTITY_TYPE_KEY, e))?,
        "minecraft:chest" => nbt_data
            .try_into()
            .map(BlockEntityType::Chest)
            .map_err(|e| FieldError::new(ENTITY_TYPE_KEY, e))?,
        "minecraft:comparator" => nbt_data
            .try_into()
            .map(BlockEntityType::Comparator)
            .map_err(|e| FieldError::new(ENTITY_TYPE_KEY, e))?,
        "minecraft:command_block" => nbt_data
            .try_into()
            .map(BlockEntityType::CommandBlock)
            .map_err(|e| FieldError::new(ENTITY_TYPE_KEY, e))?,
        "minecraft:conduit" => nbt_data
            .try_into()
            .map(BlockEntityType::Conduit)
            .map_err(|e| FieldError::new(ENTITY_TYPE_KEY, e))?,
        "minecraft:daylight_detector" => BlockEntityType::DaylightDetector,
        "minecraft:dispenser" => nbt_data
            .try_into()
            .map(BlockEntityType::Dispenser)
            .map_err(|e| FieldError::new(ENTITY_TYPE_KEY, e))?,
        "minecraft:dropper" => nbt_data
            .try_into()
            .map(BlockEntityType::Dropper)
            .map_err(|e| FieldError::new(ENTITY_TYPE_KEY, e))?,
        "minecraft:enchanting_table" => nbt_data
            .try_into()
            .map(BlockEntityType::EnchantingTable)
            .map_err(|e| FieldError::new(ENTITY_TYPE_KEY, e))?,
        "minecraft:ender_chest" => BlockEntityType::EnderChest,
        "minecraft:end_gateway" => nbt_data
            .try_into()
            .map(BlockEntityType::EndGateway)
            .map_err(|e| FieldError::new(ENTITY_TYPE_KEY, e))?,
        "minecraft:end_portal" => BlockEntityType::EndPortal,
        "minecraft:furnace" => nbt_data
            .try_into()
            .map(BlockEntityType::Furnace)
            .map_err(|e| FieldError::new(ENTITY_TYPE_KEY, e))?,
        "minecraft:hopper" => nbt_data
            .try_into()
            .map(BlockEntityType::Hopper)
            .map_err(|e| FieldError::new(ENTITY_TYPE_KEY, e))?,
        "minecraft:jigsaw" => nbt_data
            .try_into()
            .map(BlockEntityType::Jigsaw)
            .map_err(|e| FieldError::new(ENTITY_TYPE_KEY, e))?,
        "minecraft:jukebox" => nbt_data
            .try_into()
            .map(BlockEntityType::Jukebox)
            .map_err(|e| FieldError::new(ENTITY_TYPE_KEY, e))?,
        "minecraft:lectern" => nbt_data
            .try_into()
            .map(BlockEntityType::Lectern)
            .map_err(|e| FieldError::new(ENTITY_TYPE_KEY, e))?,
        "minecraft:mob_spawner" => nbt_data
            .try_into()
            .map(BlockEntityType::MobSpawner)
            .map_err(|e| FieldError::new(ENTITY_TYPE_KEY, e))?,
        "minecraft:piston" => nbt_data
            .try_into()
            .map(BlockEntityType::Piston)
            .map_err(|e| FieldError::new(ENTITY_TYPE_KEY, e))?,
        "minecraft:shulker_box" => nbt_data
            .try_into()
            .map(BlockEntityType::ShulkerBox)
            .map_err(|e| FieldError::new(ENTITY_TYPE_KEY, e))?,
        "minecraft:sign" => nbt_data
            .try_into()
            .map(BlockEntityType::Sign)
            .map_err(|e| FieldError::new(ENTITY_TYPE_KEY, e))?,
        "minecraft:skull" => nbt_data
            .try_into()
            .map(BlockEntityType::Skull)
            .map_err(|e| FieldError::new(ENTITY_TYPE_KEY, e))?,
        "minecraft:smoker" => nbt_data
            .try_into()
            .map(BlockEntityType::Smoker)
            .map_err(|e| FieldError::new(ENTITY_TYPE_KEY, e))?,
        "minecraft:soul_campfire" => nbt_data
            .try_into()
            .map(BlockEntityType::SoulCampfire)
            .map_err(|e| FieldError::new(ENTITY_TYPE_KEY, e))?,
        "minecraft:structure_block" => nbt_data
            .try_into()
            .map(BlockEntityType::StructureBlock)
            .map_err(|e| FieldError::new(ENTITY_TYPE_KEY, e))?,
        "minecraft:trapped_chest" => nbt_data
            .try_into()
            .map(BlockEntityType::TrappedChest)
            .map_err(|e| FieldError::new(ENTITY_TYPE_KEY, e))?,
        _ => BlockEntityType::Other(nbt_data),
    };
    builder.set_entity_type(ty);
    Ok(())
}

fn parse_mob_spawner(
    builder: &mut MobSpawnerBuilder,
    nbt_data: HashMap<String, Tag>,
) -> Result<(), MobSpawnerError> {
    builder.set_spawner(
        nbt_data
            .try_into()
            .map_err(|e| FieldError::new("<internal> spawner", e))?,
    );
    Ok(())
}

fn parse_cooking_block_entity<B>(
    builder: &mut B,
    mut nbt_data: HashMap<String, Tag>,
) -> Result<(), B::CookingBlockError>
where
    B: CookingBlockEntityBuilder,
{
    add_data_to_builder!(builder, nbt_data => [
        "BurnTime": set_burn_time,
        "CookTime": set_cook_time,
        "CookTimeTotal": set_cook_time_total,
        "CustomName": set_custom_name,
        "Items": set_items,
        "Lock": set_lock,
    ]);
    if let Some(value) = nbt_data.remove("RecipesUsed") {
        let r = value
            .get_as_map()
            .map_err(|e| FieldError::new("recipes_used", e))?
            .into_iter()
            .map(|(k, v)| v.try_into().map(|v| (k, v)))
            .collect::<Result<HashMap<String, i32>, _>>()
            .map_err(|e| FieldError::new("recipes_used", e))?;
        builder.set_recipes_used(r)
    }
    Ok(())
}

fn parse_inventory_block_entity<B>(
    builder: &mut B,
    mut nbt_data: HashMap<String, Tag>,
) -> Result<(), B::InventoryBlockError>
where
    B: InventoryBlockEntityBuilder,
{
    add_data_to_builder!(builder, nbt_data => [
        "CustomName": set_custom_name,
        "Items": set_items,
        "Lock": set_lock,
        "LootTable": set_loot_table,
        "LootTableSeed": set_loot_table_seed,
    ]);
    Ok(())
}

fn parse_hopper(
    builder: &mut HopperBuilder,
    mut nbt_data: HashMap<String, Tag>,
) -> Result<(), HopperError> {
    add_data_to_builder!(builder, nbt_data => [
        "TransferCooldown": set_transfer_cooldown,
    ]);
    parse_inventory_block_entity(builder, nbt_data)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::macro_tests::*;
    use super::{
        parse_block_entity, parse_inventory_block_entity, BlastFurnaceBuilder, BlockEntityError,
    };
    use crate::{
        data::{block_entity::*, load::block_entity::parse_hopper},
        nbt::*,
    };
    use std::collections::HashMap;
    use test_case::test_case;

    #[test_case(
        crate::test_util::merge(
            crate::test_util::with(block_entity_test_data_provider(), "id", "minecraft:banners".to_string().into()),
            Banner_test_data_provider()
        ) => Ok(()); "minecraft:banners"
    )]
    #[test_case(
        crate::test_util::merge(
            crate::test_util::with(block_entity_test_data_provider(), "id", "minecraft:barrel".to_string().into()),
            inventory_block_test_data_provider()
        ) => Ok(()); "minecraft:barrel"
    )]
    #[test_case(
        crate::test_util::merge(
            crate::test_util::with(block_entity_test_data_provider(), "id", "minecraft:beacon".to_string().into()),
            Beacon_test_data_provider()
        ) => Ok(()); "minecraft:beacon"
    )]
    #[test_case(
        crate::test_util::merge(
            crate::test_util::with(block_entity_test_data_provider(), "id", "minecraft:beehive".to_string().into()),
            Beehive_test_data_provider()
        ) => Ok(()); "minecraft:beehive"
    )]
    #[test_case(
        crate::test_util::merge(
            crate::test_util::with(block_entity_test_data_provider(), "id", "minecraft:blast_furnace".to_string().into()),
            cooking_block_test_data_provider()
        ) => Ok(()); "minecraft:blast_furnace"
    )]
    #[test_case(
        crate::test_util::merge(
            crate::test_util::with(block_entity_test_data_provider(), "id", "minecraft:brewing_stand".to_string().into()),
            BrewingStand_test_data_provider()
        ) => Ok(()); "minecraft:brewing_stand"
    )]
    #[test_case(
        crate::test_util::merge(
            crate::test_util::with(block_entity_test_data_provider(), "id", "minecraft:campfire".to_string().into()),
            Campfire_test_data_provider()
        ) => Ok(()); "minecraft:campfire"
    )]
    #[test_case(
        crate::test_util::merge(
            crate::test_util::with(block_entity_test_data_provider(), "id", "minecraft:chiseled_bookshelf".to_string().into()),
            ChiseledBookshelf_test_data_provider()
        ) => Ok(()); "minecraft:chiseled_bookshelf"
    )]
    #[test_case(
        crate::test_util::merge(
            crate::test_util::with(block_entity_test_data_provider(), "id", "minecraft:chest".to_string().into()),
            inventory_block_test_data_provider()
        ) => Ok(()); "minecraft:chest"
    )]
    #[test_case(
        crate::test_util::merge(
            crate::test_util::with(block_entity_test_data_provider(), "id", "minecraft:comparator".to_string().into()),
            Comparator_test_data_provider()
        ) => Ok(()); "minecraft:comparator"
    )]
    #[test_case(
        crate::test_util::merge(
            crate::test_util::with(block_entity_test_data_provider(), "id", "minecraft:command_block".to_string().into()),
            CommandBlock_test_data_provider()
        ) => Ok(()); "minecraft:command_block"
    )]
    #[test_case(
        crate::test_util::merge(
            crate::test_util::with(block_entity_test_data_provider(), "id", "minecraft:conduit".to_string().into()),
            Conduit_test_data_provider()
        ) => Ok(()); "minecraft:conduit"
    )]
    #[test_case(
        crate::test_util::merge(
            crate::test_util::with(block_entity_test_data_provider(), "id", "minecraft:dispenser".to_string().into()),
            inventory_block_test_data_provider()
        ) => Ok(()); "minecraft:dispenser"
    )]
    #[test_case(
        crate::test_util::merge(
            crate::test_util::with(block_entity_test_data_provider(), "id", "minecraft:dropper".to_string().into()),
            inventory_block_test_data_provider()
        ) => Ok(()); "minecraft:dropper"
    )]
    #[test_case(
        crate::test_util::merge(
            crate::test_util::with(block_entity_test_data_provider(), "id", "minecraft:enchanting_table".to_string().into()),
            EnchantingTable_test_data_provider()
        ) => Ok(()); "minecraft:enchanting_table"
    )]
    #[test_case(
        crate::test_util::merge(
            crate::test_util::with(block_entity_test_data_provider(), "id", "minecraft:end_gateway".to_string().into()),
            EndGateway_test_data_provider()
        ) => Ok(()); "minecraft:end_gateway"
    )]
    #[test_case(
        crate::test_util::merge(
            crate::test_util::with(block_entity_test_data_provider(), "id", "minecraft:furnace".to_string().into()),
            cooking_block_test_data_provider()
        ) => Ok(()); "minecraft:furnace"
    )]
    #[test_case(
        crate::test_util::merge(
            crate::test_util::with(block_entity_test_data_provider(), "id", "minecraft:hopper".to_string().into()),
            hopper_test_data_provider()
        ) => Ok(()); "minecraft:hopper"
    )]
    #[test_case(
        crate::test_util::merge(
            crate::test_util::with(block_entity_test_data_provider(), "id", "minecraft:jigsaw".to_string().into()),
            Jigsaw_test_data_provider()
        ) => Ok(()); "minecraft:jigsaw"
    )]
    #[test_case(
        crate::test_util::merge(
            crate::test_util::with(block_entity_test_data_provider(), "id", "minecraft:jukebox".to_string().into()),
            Jukebox_test_data_provider()
        ) => Ok(()); "minecraft:jukebox"
    )]
    #[test_case(
        crate::test_util::merge(
            crate::test_util::with(block_entity_test_data_provider(), "id", "minecraft:lectern".to_string().into()),
            Lectern_test_data_provider()
        ) => Ok(()); "minecraft:lectern"
    )]
    #[test_case(
        crate::test_util::merge(
            crate::test_util::with(block_entity_test_data_provider(), "id", "minecraft:mob_spawner".to_string().into()),
            Spawner_test_data_provider()
        ) => Ok(()); "minecraft:mob_spawner"
    )]
    #[test_case(
        crate::test_util::merge(
            crate::test_util::with(block_entity_test_data_provider(), "id", "minecraft:piston".to_string().into()),
            Piston_test_data_provider()
        ) => Ok(()); "minecraft:piston"
    )]
    #[test_case(
        crate::test_util::merge(
            crate::test_util::with(block_entity_test_data_provider(), "id", "minecraft:shulker_box".to_string().into()),
            inventory_block_test_data_provider()
        ) => Ok(()); "minecraft:shulker_box"
    )]
    #[test_case(
        crate::test_util::merge(
            crate::test_util::with(block_entity_test_data_provider(), "id", "minecraft:sign".to_string().into()),
            Sign_test_data_provider()
        ) => Ok(()); "minecraft:sign"
    )]
    #[test_case(
        crate::test_util::merge(
            crate::test_util::with(block_entity_test_data_provider(), "id", "minecraft:skull".to_string().into()),
            Skull_test_data_provider()
        ) => Ok(()); "minecraft:skull"
    )]
    #[test_case(
        crate::test_util::merge(
            crate::test_util::with(block_entity_test_data_provider(), "id", "minecraft:smoker".to_string().into()),
            cooking_block_test_data_provider()
        ) => Ok(()); "minecraft:smoker"
    )]
    #[test_case(
        crate::test_util::merge(
            crate::test_util::with(block_entity_test_data_provider(), "id", "minecraft:soul_campfire".to_string().into()),
            SoulCampfire_test_data_provider()
        ) => Ok(()); "minecraft:soul_campfire"
    )]
    #[test_case(
        crate::test_util::merge(
            crate::test_util::with(block_entity_test_data_provider(), "id", "minecraft:structure_block".to_string().into()),
            StructureBlock_test_data_provider()
        ) => Ok(()); "minecraft:structure_block"
    )]
    #[test_case(
        crate::test_util::merge(
            crate::test_util::with(block_entity_test_data_provider(), "id", "minecraft:trapped_chest".to_string().into()),
            inventory_block_test_data_provider()
        ) => Ok(()); "minecraft:trapped_chest"
    )]
    #[test_case(
        crate::test_util::merge(
            crate::test_util::with(block_entity_test_data_provider(), "id", "unknonwn".to_string().into()),
            inventory_block_test_data_provider()
        ) => Ok(()); "unknonwn id"
    )]
    #[test_case(crate::test_util::without(&block_entity_test_data_provider, "id") => Err(BlockEntityBuilderError::UnsetId.into()); "unset id")]
    #[test_case(crate::test_util::with(block_entity_test_data_provider(), "id", 1i8.into()) => Err(crate::data::FieldError::new("id", crate::nbt::Error::InvalidValue).into()); "invalid id")]
    fn test_parse_entity_builder(map: HashMap<String, Tag>) -> Result<(), BlockEntityError> {
        let mut builder = BlockEntityBuilder::default();
        parse_block_entity(&mut builder, map)?;
        builder.try_build()?;
        Ok(())
    }

    fn block_entity_test_data_provider() -> HashMap<String, Tag> {
        HashMap::from_iter([
            ("x".to_string(), Tag::Int(123)),
            ("y".to_string(), Tag::Int(123)),
            ("z".to_string(), Tag::Int(123)),
            ("id".to_string(), Tag::String("test".to_string())),
        ])
    }

    fn cooking_block_test_data_provider() -> HashMap<String, Tag> {
        HashMap::from_iter([
            ("BurnTime".to_string(), Tag::Short(10)),
            ("CookTime".to_string(), Tag::Short(10)),
            ("CookTimeTotal".to_string(), Tag::Short(10)),
            ("CustomName".to_string(), Tag::String("test".to_string())),
            ("Items".to_string(), Tag::List(List::from_iter([]))),
            ("Lock".to_string(), Tag::String("test".to_string())),
            ("RecipesUsed".to_string(), Tag::Compound(HashMap::new())),
        ])
    }

    fn inventory_block_test_data_provider() -> HashMap<String, Tag> {
        HashMap::from_iter([
            ("CustomName".to_string(), Tag::String("test".to_string())),
            ("Items".to_string(), Tag::List(List::from_iter([]))),
            ("Lock".to_string(), Tag::String("test".to_string())),
            ("LootTable".to_string(), Tag::String("test".to_string())),
            ("LootTableSeed".to_string(), Tag::Long(123)),
        ])
    }

    fn hopper_test_data_provider() -> HashMap<String, Tag> {
        let mut map = HashMap::from_iter([("TransferCooldown".to_string(), Tag::Int(123))]);
        map.extend(inventory_block_test_data_provider());
        map
    }

    #[test_case(BarrelBuilder::default() => Ok(Barrel {
        custom_name: Some("test".to_string()),
        items: Some(List::from(vec![])),
        lock: Some("test".to_string()),
        loot_table: Some("test".to_string()),
        loot_table_seed: Some(123)
    }); "Barrel")]
    #[test_case(ChestBuilder::default() => Ok(Chest {
        custom_name: Some("test".to_string()),
        items: Some(List::from(vec![])),
        lock: Some("test".to_string()),
        loot_table: Some("test".to_string()),
        loot_table_seed: Some(123)
    }); "Chest")]
    #[test_case(DispenserBuilder::default() => Ok(Dispenser {
        custom_name: Some("test".to_string()),
        items: Some(List::from(vec![])),
        lock: Some("test".to_string()),
        loot_table: Some("test".to_string()),
        loot_table_seed: Some(123)
    }); "Dispenser")]
    #[test_case(DropperBuilder::default() => Ok(Dropper {
        custom_name: Some("test".to_string()),
        items: Some(List::from(vec![])),
        lock: Some("test".to_string()),
        loot_table: Some("test".to_string()),
        loot_table_seed: Some(123)
    }); "Dropper")]
    #[test_case(ShulkerBoxBuilder::default() => Ok(ShulkerBox {
        custom_name: Some("test".to_string()),
        items: Some(List::from(vec![])),
        lock: Some("test".to_string()),
        loot_table: Some("test".to_string()),
        loot_table_seed: Some(123)
    }); "ShulkerBox")]
    #[test_case(TrappedChestBuilder::default() => Ok(TrappedChest {
        custom_name: Some("test".to_string()),
        items: Some(List::from(vec![])),
        lock: Some("test".to_string()),
        loot_table: Some("test".to_string()),
        loot_table_seed: Some(123)
    }); "TrappedChest")]
    fn test_parse_inventory_block_entity<B>(
        mut builder: B,
    ) -> Result<B::Target, B::InventoryBlockError>
    where
        B: InventoryBlockEntityBuilder,
    {
        let nbt_data = inventory_block_test_data_provider();
        parse_inventory_block_entity(&mut builder, nbt_data)?;
        builder.try_build()
    }

    #[test]
    fn test_parse_hopper() {
        let expected = Ok(Hopper {
            custom_name: Some("test".to_string()),
            items: Some(List::from(vec![])),
            lock: Some("test".to_string()),
            transfer_cooldown: 123,
            loot_table: Some("test".to_string()),
            loot_table_seed: Some(123),
        });
        let nbt_data = hopper_test_data_provider();
        let mut builder = HopperBuilder::default();
        parse_hopper(&mut builder, nbt_data).unwrap();
        assert_eq!(builder.try_build(), expected);
    }

    #[test_case(HashMap::from_iter([
        ("BurnTime".to_string(), Tag::Short(10)),
        ("CookTime".to_string(), Tag::Short(10)),
        ("CookTimeTotal".to_string(), Tag::Short(10)),
        ("CustomName".to_string(), Tag::String("test".to_string())),
        ("Items".to_string(), Tag::List(List::from_iter([]))),
        ("Lock".to_string(), Tag::String("test".to_string())),
        ("RecipesUsed".to_string(), Tag::Compound(HashMap::new()))
    ]), BlastFurnaceBuilder::default() => Ok(BlastFurnace {
        burn_time: 10,
        cook_time: 10,
        cook_time_total: 10,
        custom_name: Some("test".to_string()),
        items: Some(List::from_iter([])),
        lock: Some("test".to_string()),
        recipes_used: HashMap::new(),
    }); "BlastFurnace")]
    #[test_case(HashMap::from_iter([
        ("BurnTime".to_string(), Tag::Short(10)),
        ("CookTime".to_string(), Tag::Short(10)),
        ("CookTimeTotal".to_string(), Tag::Short(10)),
        ("CustomName".to_string(), Tag::String("test".to_string())),
        ("Items".to_string(), Tag::List(List::from_iter([]))),
        ("Lock".to_string(), Tag::String("test".to_string())),
        ("RecipesUsed".to_string(), Tag::Compound(HashMap::new()))
    ]), FurnaceBuilder::default() => Ok(Furnace {
        burn_time: 10,
        cook_time: 10,
        cook_time_total: 10,
        custom_name: Some("test".to_string()),
        items: Some(List::from_iter([])),
        lock: Some("test".to_string()),
        recipes_used: HashMap::new(),
    }); "Furnace")]
    #[test_case(HashMap::from_iter([
        ("BurnTime".to_string(), Tag::Short(10)),
        ("CookTime".to_string(), Tag::Short(10)),
        ("CookTimeTotal".to_string(), Tag::Short(10)),
        ("CustomName".to_string(), Tag::String("test".to_string())),
        ("Items".to_string(), Tag::List(List::from_iter([]))),
        ("Lock".to_string(), Tag::String("test".to_string())),
        ("RecipesUsed".to_string(), Tag::Compound(HashMap::new()))
    ]), SmokerBuilder::default() => Ok(Smoker {
        burn_time: 10,
        cook_time: 10,
        cook_time_total: 10,
        custom_name: Some("test".to_string()),
        items: Some(List::from_iter([])),
        lock: Some("test".to_string()),
        recipes_used: HashMap::new(),
    }); "Smoker")]
    fn test_parse_cooking_block<B>(
        nbt: HashMap<String, Tag>,
        mut builder: B,
    ) -> Result<B::Target, B::CookingBlockError>
    where
        B: CookingBlockEntityBuilder,
    {
        super::parse_cooking_block_entity(&mut builder, nbt)?;
        builder.try_build()
    }
}
