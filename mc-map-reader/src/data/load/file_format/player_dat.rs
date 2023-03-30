use std::collections::HashMap;

use crate::{
    data::{
        file_format::player_dat::*,
        load::{
            entity::{EntityError, MobError},
            item::{ItemError, ItemWithSlotError},
        },
        FieldError,
    },
    nbt::Tag,
};

mod_try_from_tag!(
    Player: parse_player ? [
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
    ],
EnteredNetherPosition: [
    "x" => set_x test(1f64 => x = 1.; EnteredNetherPositionBuilderError::UnsetX),
    "y" => set_y test(1f64 => y = 1.; EnteredNetherPositionBuilderError::UnsetY),
    "z" => set_z test(1f64 => z = 1.; EnteredNetherPositionBuilderError::UnsetZ),
],
LastDeathLocation: [
    "pos" => set_pos test(crate::nbt::Array::<i32>::from(vec![]) => pos = crate::nbt::Array::from(vec![]); LastDeathLocationBuilderError::UnsetPos),
    "dimension" => set_dimension test("dim".to_string() => dimension = "dim".to_string(); LastDeathLocationBuilderError::UnsetDimension),
],
RecipeBook: [
    "recipes" => set_recipes test(crate::nbt::List::from(vec![]) => recipes = crate::nbt::List::from(vec![]); RecipeBookBuilderError::UnsetRecipes),
    "toBeDisplayed" => set_to_be_displayed test(crate::nbt::List::from(vec![]) => to_be_displayed = crate::nbt::List::from(vec![]); RecipeBookBuilderError::UnsetToBeDisplayed),
    "isFilteringCraftable" => set_is_filtering_craftable test(1i8 => is_filtering_craftable = true; RecipeBookBuilderError::UnsetIsFilteringCraftable),
    "isGuiOpen" => set_is_gui_open test(1i8 => is_gui_open = true; RecipeBookBuilderError::UnsetIsGuiOpen),
    "isFurnaceFilteringCraftable" => set_is_furnace_filtering_craftable test(1i8 => is_furnace_filtering_craftable = true; RecipeBookBuilderError::UnsetIsFurnaceFilteringCraftable),
    "isFurnaceGuiOpen" => set_is_furnace_gui_open test(1i8 => is_furnace_gui_open = true; RecipeBookBuilderError::UnsetIsFurnaceGuiOpen),
    "isBlastingFurnaceFilteringCraftable" => set_is_blasting_furnace_filtering_craftable test(1i8 => is_blasting_furnace_filtering_craftable = true; RecipeBookBuilderError::UnsetIsBlastingFurnaceFilteringCraftable),
    "isBlastingFurnaceGuiOpen" => set_is_blasting_furnace_gui_open test(1i8 => is_blasting_furnace_gui_open = true; RecipeBookBuilderError::UnsetIsBlastingFurnaceGuiOpen),
    "isSmokerFilteringCraftable" => set_is_smoker_filtering_craftable test(1i8 => is_smoker_filtering_craftable = true; RecipeBookBuilderError::UnsetIsSmokerFilteringCraftable),
    "isSmokerGuiOpen" => set_is_smoker_gui_open test(1i8 => is_smoker_gui_open = true; RecipeBookBuilderError::UnsetIsSmokerGuiOpen),
],
RootVehicle: [
    "Entity" => set_entity test(HashMap::new() => entity = crate::data::entity::EntityBuilder::default().build(); RootVehicleBuilderError::UnsetEntity),
    "Attach" => set_attach test(crate::nbt::Array::<i32>::from(vec![]) => attach = crate::nbt::Array::from(vec![]); RootVehicleBuilderError::UnsetAttach),
] ? [
    Entity,
],
WardenSpawnTracker: [
    "cooldown_ticks" => set_cooldown_ticks test(1i32 => cooldown_ticks = 1; WardenSpawnTrackerBuilderError::UnsetCooldownTicks),
    "ticks_since_last_warning" => set_ticks_since_last_warning test(1i32 => ticks_since_last_warning = 1; WardenSpawnTrackerBuilderError::UnsetTicksSinceLastWarning),
    "warning_level" => set_warning_level test(1i32 => warning_level = 1; WardenSpawnTrackerBuilderError::UnsetWarningLevel),
],
PlayerAbilities: [
    "flying" => set_flying test(1i8 => flying = true; PlayerAbilitiesBuilderError::UnsetFlying),
    "flySpeed" => set_fly_speed test(1f32 => fly_speed = 1.; PlayerAbilitiesBuilderError::UnsetFlySpeed),
    "instabuild" => set_insta_build test(1i8 => insta_build = true; PlayerAbilitiesBuilderError::UnsetInstaBuild),
    "invulnerable" => set_invulnerable test(1i8 => invulnerable = true; PlayerAbilitiesBuilderError::UnsetInvulnerable),
    "mayBuild" => set_may_build test(1i8 => may_build = true; PlayerAbilitiesBuilderError::UnsetMayBuild),
    "mayfly" => set_may_fly test(1i8 => may_fly = true; PlayerAbilitiesBuilderError::UnsetMayFly),
    "walkSpeed" => set_walk_speed test(1f32 => walk_speed = 1.; PlayerAbilitiesBuilderError::UnsetWalkSpeed),
],);

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
    builder.set_mob(
        nbt_data
            .try_into()
            .map_err(|e| FieldError::new("<internal> mob", e))?,
    );
    Ok(())
}

#[cfg(test)]
pub mod tests {
    use std::collections::HashMap;
    use super::macro_tests::*;

    use crate::{nbt::{Tag, List}, data::{load::{entity::{macro_tests::{Entity_test_data_provider, Entity_test_result}, tests::{mob_test_data_provider, mob_test_result}}, item::macro_tests::{Item_test_data_provider, Item_test_result}}, file_format::player_dat::Player}};

    #[test]
    fn test_parse_player() {
        assert_eq!(
            Player::try_from(player_test_data_provider()),
            Ok(player_test_result())
        )
    }

    pub fn player_test_data_provider() -> HashMap<String, Tag> {
        let mut map: HashMap<String, Tag> = [
            ("abilities", PlayerAbilities_test_data_provider().into()),
            ("DataVersion", Tag::Int(1)),
            ("Dimension", Tag::String("minecraft:overworld".to_string())),
            ("EnderItems", Tag::List(List::from(vec![]))),
            ("enteredNetherPosition", EnteredNetherPosition_test_data_provider().into()),
            ("foodExhaustionLevel", Tag::Float(2.)),
            ("foodLevel", Tag::Int(4)),
            ("foodSaturationLevel", Tag::Float(4.)),
            ("foodTickTimer", Tag::Int(5)),
            ("Inventory", Tag::List(List::from(vec![]))),
            ("LastDeathLocation", LastDeathLocation_test_data_provider().into()),
            ("playerGameType", Tag::Int(34)),
            ("previousPlayerGameType", Tag::Int(54)),
            ("recipeBook", RecipeBook_test_data_provider().into()),
            ("RootVehicle", RootVehicle_test_data_provider().into()),
            ("Score", Tag::Int(354)),
            ("seenCredits", Tag::Byte(1)),
            ("SelectedItem", Item_test_data_provider().into()),
            ("SelectedItemSlot", Tag::Int(54)),
            ("ShoulderEntityLeft", Entity_test_data_provider().into()),
            ("ShoulderEntityRight", Entity_test_data_provider().into()),
            ("SleepTimer", Tag::Int(12)),
            ("SpawnDimension", Tag::String("minecraft:overworld".to_string())),
            ("SpawnForced", Tag::Byte(1)),
            ("SpawnX", Tag::Int(12)),
            ("SpawnY", Tag::Int(75)),
            ("SpawnZ", Tag::Int(13)),
            ("warden_spawn_tracker", WardenSpawnTracker_test_data_provider().into()),
            ("XpLevel", Tag::Int(123)),
            ("XpP", Tag::Float(45.)),
            ("XpSeed", Tag::Int(96)),
            ("XpTotal", Tag::Int(12)),
        ].map(|(k, v)| (k.to_string(), v)).into();
        map.extend(mob_test_data_provider().into_iter());
        map
    }

    pub fn player_test_result() -> Player {
        Player {
            abilities: PlayerAbilities_test_result(),
            data_version: 1,
            dimension: "minecraft:overworld".to_string(),
            ender_items: vec![].into(),
            entered_nether_position: Some(EnteredNetherPosition_test_result()),
            food_exhaustion_level: 2.,
            food_level: 4,
            food_saturation_level: 4.,
            food_tick_timer: 5,
            inventory: vec![].into(),
            last_death_location: Some(LastDeathLocation_test_result()),
            player_game_type: 34,
            previous_player_game_type: 54,
            recipe_book: RecipeBook_test_result(),
            root_vehicle: Some(RootVehicle_test_result()),
            score: 354,
            seen_credits: true,
            selected_item: Some(Item_test_result()),
            selected_item_slot: 54,
            shoulder_entity_left: Some(Entity_test_result()),
            shoulder_entity_right: Some(Entity_test_result()),
            sleep_timer: 12,
            spawn_dimension: "minecraft:overworld".to_string(),
            spawn_forced: true,
            spawn_x: 12,
            spawn_y: 75,
            spawn_z: 13,
            warden_spawn_tracker: Some(WardenSpawnTracker_test_result()),
            xp_level: 123,
            xp_p: 45.,
            xp_seed: 96,
            xp_total: 12,
            mob: mob_test_result()
        }
    }

}
