use std::collections::HashMap;

use crate::{
    data::{block_entity::*, load::entity::EntityError, FieldError},
    nbt::Tag,
};

use crate::data::load::item::*;

mod_try_from_tag!(BlockEntity: parse_block_entity ? [
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
);

try_from_tag!(
    Beehive => [
        "Bees": set_bees,
        "FlowerPos": set_flower_pos,
    ] ? [
        BeeInHive,
        FlowerPos,
    ]
);

try_from_tag!(BeeInHive => [
    "EntityData": set_entity_data,
    "MinOccupationTicks": set_min_occupation_ticks,
    "TicksInHive": set_ticks_in_hive,
] ? [
    Entity,
]);
try_from_tag!(FlowerPos => [
    "X": set_x,
    "Y": set_y,
    "Z": set_z,
]);
try_from_tag!(Beacon => [
        "CustomName": set_custom_name,
        "Lock": set_lock,
        "Primary": set_primary,
        "Secondary": set_secondary,
]);
try_from_tag!(Barrel => parse_inventory_block_entity ? [ ItemWithSlot, ]);
try_from_tag!(Banner => [
    "CustomName": set_custom_name,
    "Patterns": set_patterns,
] ? [
    BannerPattern,
]);
try_from_tag!(BannerPattern => [
    "Color": set_color,
    "Pattern": set_pattern,
]);
try_from_tag!(BlastFurnace => parse_cooking_block_entity ? [ ItemWithSlot, ]);
try_from_tag!(BrewingStand => [
    "BrewTime": set_brew_time,
    "CustomName": set_custom_name,
    "Fuel": set_fuel,
    "Items": set_items,
    "Lock": set_lock,
] ? [ ItemWithSlot, ]);
try_from_tag!(Campfire => [
    "CookingTimes": set_cooking_times,
    "CookingTotalTimes": set_cooking_total_times,
    "Items": set_items,
] ? [
    ItemWithSlot,
]);
try_from_tag!(ChiseledBookshelf => [
    "Items": set_items,
    "last_interacted_slot": set_last_interacted_slot,
] ? [
    ItemWithSlot,
]);
try_from_tag!(Chest => parse_inventory_block_entity ? [ ItemWithSlot, ]);
try_from_tag!(Comparator => [
    "OutputSignal": set_output_signal,
]);
try_from_tag!(CommandBlock => [
    "auto": set_auto,
    "Command": set_command,
    "conditionMet": set_condition_met,
    "CustomName": set_custom_name,
    "LastExecution": set_last_execution,
    "LastOutput": set_last_output,
    "powered": set_powered,
    "SuccessCount": set_success_count,
    "UpdateLastExecution": set_update_last_execution,
]);
try_from_tag!(Conduit => [
    "Target": set_target,
]);
try_from_tag!(Dispenser => parse_inventory_block_entity ? [ ItemWithSlot, ]);
try_from_tag!(Dropper => parse_inventory_block_entity ? [ ItemWithSlot, ]);
try_from_tag!(EnchantingTable => [
    "CustomName": set_custom_name,
]);
try_from_tag!(EndGateway => [
    "Age": set_age,
    "ExactTeleport": set_exact_teleport,
    "ExitPortal": set_exit_portal,
] ? [
    ExitPortal,
]);
try_from_tag!(ExitPortal => [
    "X": set_x,
    "Y": set_y,
    "Z": set_z,
]);
try_from_tag!(Furnace => parse_cooking_block_entity ? [ ItemWithSlot, ]);
try_from_tag!(Hopper => parse_inventory_block_entity ? [ ItemWithSlot, ]);
try_from_tag!(Jigsaw => [
    "final_state": set_final_state,
    "joint": set_joint,
    "name": set_name,
    "pool": set_pool,
    "target": set_target,
]);
try_from_tag!(Jukebox => [
    "IsPlaying": set_is_playing,
    "RecordItem": set_record_item,
    "RecordStartTick": set_record_start_tick,
    "TickCount": set_tick_count,
] ? [
    Item,
]);
try_from_tag!(Lectern => [
    "Book": set_book,
    "Page": set_page,
] ? [
    Item,
]);
try_from_tag!(Spawner => [
    "Delay": set_delay,
    "MaxNearbyEntities": set_max_nearby_entities,
    "MaxSpawnDelay": set_max_spawn_delay,
    "MinSpawnDelay": set_min_spawn_delay,
    "RequiredPlayerRange": set_required_player_range,
    "SpawnCount": set_spawn_count,
    "SpawnData": set_spawn_data,
    "SpawnPotentials": set_spawn_potentials,
    "SpawnRange": set_spawn_range,
] ? [
    PotentialSpawn,
]);
try_from_tag!(PotentialSpawn => [
    "weight": set_weight,
    "data": set_data,
]);
try_from_tag!(Piston => [
    "blockState": set_block_state,
    "extending": set_extending,
    "facing": set_facing,
    "progress": set_progress,
    "source": set_source,
] ? [
    PistonBlockState,
]);
try_from_tag!(PistonBlockState => [
    "Name": set_name,
    "Properties": set_properties,
]);
try_from_tag!(ShulkerBox => parse_inventory_block_entity ? [ ItemWithSlot, ]);
try_from_tag!(Sign => [
    "GlowingText": set_glowing_text,
    "Color": set_color,
    "Text1": set_text1,
    "Text2": set_text2,
    "Text3": set_text3,
    "Text4": set_text4,
]);
try_from_tag!(Skull => [
    "note_block_sound": set_note_block_sound,
    "ExtraType": set_extra_type,
    "SkullOwner": set_skull_owner,
] ? [
    SkullOwner,
]);
try_from_tag!(SkullOwner => [
    "Id": set_id,
    "Name": set_name,
    "Properties": set_properties,
] ? [
    SkullOwnerProperties,
]);
try_from_tag!(SkullOwnerProperties => [
    "textures": set_textures,
] ? [
    SkullOwnerTextures,
]);
try_from_tag!(SkullOwnerTextures => [
    "Value": set_value,
    "Signature": set_signature,
]);
try_from_tag!(Smoker => parse_cooking_block_entity ? [ ItemWithSlot, ]);
try_from_tag!(SoulCampfire => [
    "CookingTimes": set_cooking_times,
    "CookingTotalTimes": set_cooking_total_times,
    "Items": set_items,
] ? [
    ItemWithSlot,
]);
try_from_tag!(StructureBlock => [
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
    "sizeZ": set_size_z,
]);
try_from_tag!(TrappedChest => parse_inventory_block_entity ? [ ItemWithSlot, ]);
try_from_tag!(MobSpawner => parse_mob_spawner ? [ Spawner, ]);

fn parse_block_entity(
    builder: &mut BlockEntityBuilder,
    mut nbt_data: HashMap<String, Tag>,
) -> Result<(), BlockEntityError> {
    let Tag::String(id) = nbt_data
        .get("id")
        .ok_or(BlockEntityBuilderError::UnsetId)? else {
            return Err(crate::nbt::Error::InvalidValue.into());
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

fn parse_inventory_block_entity<E>(
    builder: &mut impl InventoryBlockEntityBuilder,
    mut nbt_data: HashMap<String, Tag>,
) -> Result<(), E>
where
    E: From<FieldError<crate::nbt::Error>> + From<FieldError<ItemWithSlotError>>,
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

#[cfg(test)]
mod tests {
    use super::BlastFurnaceBuilder;
    use crate::{data::block_entity::*, nbt::*};
    use std::collections::HashMap;
    use test_case::test_case;

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
