use std::collections::HashMap;

use crate::{
    data::{
        file_format::player_dat::*,
        load::{
            block_entity::{ItemError, ItemWithSlotError},
            entity::{EntityError, MobError},
        },
    },
    nbt::Tag,
};

try_from_tag![
    Player => parse_player ? [
        Mob,
        PlayerAbilities,
        ItemWithSlot,
        Item,
        EnteredNetherPosition,
        LastDeathLocation,
        RecipeBook,
        RootVehicle,
        Entity,
        WardenSpawnTracker,
    ]
];

fn parse_player(
    builder: &mut PlayerBuilder,
    mut nbt_data: HashMap<String, Tag>,
) -> Result<(), PlayerError> {
    add_data_to_builder!(builder, nbt_data => [
        "abilities": set_abilities,
        "DataVersion": set_data_version,
        "Dimension": set_dimension,
        "EnderItems": set_ender_items,
        "enteredNetherPosition": set_entered_nether_position,
        "foodExhaustionLevel": set_food_exhaustion_level,
        "foodLevel": set_food_level,
        "foodSaturationLevel": set_food_saturation_level,
        "foodTickTimer": set_food_tick_timer,
        "Inventory": set_inventory,
        "LastDeathLocation": set_last_death_location,
        "playerGameType": set_player_game_type,
        "previousPlayerGameType": set_previous_player_game_type,
        "recipeBook": set_recipe_book,
        "RootVehicle": set_root_vehicle,
        "Score": set_score,
        "seenCredits": set_seen_credits,
        "SelectedItem": set_selected_item,
        "SelectedItemSlot": set_selected_item_slot,
        "ShoulderEntityLeft": set_shoulder_entity_left,
        "ShoulderEntityRight": set_shoulder_entity_right,
        "SleepTimer": set_sleep_timer,
        "SpawnDimension": set_spawn_dimension,
        "SpawnForced": set_spawn_forced,
        "SpawnX": set_spawn_x,
        "SpawnY": set_spawn_y,
        "SpawnZ": set_spawn_z,
        "warden_spawn_tracker": set_warden_spawn_tracker,
        "XpLevel": set_xp_level,
        "XpP": set_xp_p,
        "XpSeed": set_xp_seed,
        "XpTotal": set_xp_total,
    ]);
    builder.set_mob(nbt_data.try_into().map_err(|e| PlayerError::Mob(e))?);
    Ok(())
}
try_from_tag!(EnteredNetherPosition => [
    "x": set_x,
    "y": set_y,
    "z": set_z,
]);
try_from_tag!(LastDeathLocation => [
    "pos": set_pos,
    "dimension": set_dimension,
]);
try_from_tag!(RecipeBook => [
    "recipes": set_recipes,
    "toBeDisplayed": set_to_be_displayed,
    "isFilteringCraftable": set_is_filtering_craftable,
    "isGuiOpen": set_is_gui_open,
    "isFurnaceFilteringCraftable": set_is_furnace_filtering_craftable,
    "isFurnaceGuiOpen": set_is_furnace_gui_open,
    "isBlastingFurnaceFilteringCraftable": set_is_blasting_furnace_filtering_craftable,
    "isBlastingFurnaceGuiOpen": set_is_blasting_furnace_gui_open,
    "isSmokerFilteringCraftable": set_is_smoker_filtering_craftable,
    "isSmokerGuiOpen": set_is_smoker_gui_open,
]);
try_from_tag!(RootVehicle => [
    "Entity": set_entity,
    "Attach": set_attach,
] ? [
    Entity,
]);
try_from_tag!(WardenSpawnTracker => [
    "cooldown_ticks": set_cooldown_ticks,
    "ticks_since_last_warning": set_ticks_since_last_warning,
    "warning_level": set_warning_level,
]);
try_from_tag!(PlayerAbilities => [
    "flying": set_flying,
    "flySpeed": set_fly_speed,
    "instabuild": set_insta_build,
    "invulnerable": set_invulnerable,
    "mayBuild": set_may_build,
    "mayfly": set_may_fly,
    "walkSpeed": set_walk_speed,
]);