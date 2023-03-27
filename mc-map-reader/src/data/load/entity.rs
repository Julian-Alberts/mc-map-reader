use std::collections::HashMap;

use crate::{data::entity::*, data::{load::item::ItemError, FieldError}, nbt::*};

mod_try_from_tag!({
Entity: [
    "Air" => set_air test(1_i16 => air = Some(1)),
    "CustomName" => set_custom_name test("test_name".to_string() => custom_name = Some("test_name".to_string())),
    "CustomNameVisible" => set_custom_name_visible test(1_i8 => custom_name_visible = Some(true)),
    "FallDistance" => set_fall_distance test(2_f32 => fall_distance = Some(2.)),
    "Fire" => set_fire test(3i16 => fire = 3),
    "Glowing" => set_glowing test(1i8 => glowing = true),
    "HasVisualFire" => set_has_visual_fire test(1i8 => has_visual_fire = true),
    "id" => set_id test("test_id".to_string() => id = Some("test_id".to_string())),
    "Invulnerable" => set_invulnerable test(1i8 => invulnerable = true),
    "Motion" => set_motion test(List::<Tag>::from(vec![1_f64.into(),2f64.into(),3f64.into()]) => motion = Some(List::from_iter([1.,2.,3.]))),
    "NoGravity" => set_no_gravity test(1i8 => no_gravity = true),
    "OnGround" => set_on_ground test(0i8 => on_ground = false),
    "Passengers" => set_passengers test(List::from_iter([]) => passengers = Some(List::from_iter([]))),
    "PortalCooldown" => set_portal_colldown test(4i32 => portal_colldown = 4),
    "Pos" => set_pos test(List::from_iter([]) => pos = Some(List::from_iter([]))),
    "Rotation" => set_rotation test(List::from_iter([]) => rotation = Some(List::from_iter([]))),
    "Silent" => set_silent test(1i8 => silent = true),
    "Tags" => set_tags test(HashMap::new() => tags = Some(HashMap::new())),
    "TicksFrozen" => set_ticks_frozen test(5i32 => ticks_frozen = Some(5)),
    "UUID" => set_uuid test(Array::<i32>::from(vec![]) => uuid = Some(Array::from_iter([]))),
] ? [ 
    Entity, 
],
Mob: parse_mob ? [
    Entity,
    ActiveEffect,
    Item,
    Leash,
],
ActiveEffect: [
    "Ambient" => set_ambient test(1i8 => ambient = true),
    "Amplifier" => set_amplifier test(1i8 => amplifier = 1),
    "Duration" => set_duration test(1i32 => duration = 1),
    "Id" => set_id test(1i32 => id = 1),
    "ShowIcon" => set_show_icon test(1i8 => show_icon = true),
    "ShowParticles" => set_show_particles test(1i8 => show_particles = true),
],
});
try_from_tag!(enum Leash => parse_leash);
fn parse_mob(builder: &mut MobBuilder, mut nbt_data: HashMap<String, Tag>) -> Result<(), MobError> {
    add_data_to_builder!(builder, nbt_data => [
        "AbsorptionAmount": set_absorption_amount,
        "ActiveEffects": set_active_effects,
        "ArmorDropChances": set_armor_drop_chances,
        "ArmorItems": set_armor_items,
        "Attributes": set_attributes,
        "Brain": set_brain,
        "CanPickUpLoot": set_can_pick_up_loot,
        "DeathLootTable": set_death_loot_table,
        "DeathLootTableSeed": set_death_loot_table_seed,
        "DeathTime": set_death_time,
        "FallFlying": set_fall_flying,
        "Health": set_health,
        "HurtByTimestamp": set_hurt_by_timestamp,
        "HurtTime": set_hurt_time,
        "HandDropChances": set_hand_drop_chances,
        "HandItems": set_hand_items,
        "Leash": set_leash,
        "LeftHanded": set_left_handed,
        "NoAI": set_no_ai,
        "PersistenceRequired": set_persistence_required,
        "SleepingX": set_sleeping_x,
        "SleepingY": set_sleeping_y,
        "SleepingZ": set_sleeping_z,
        "Team": set_team,
    ]);
    builder.set_entity(nbt_data.try_into().map_err(|e| FieldError::new("<internal> entity", e))?);
    Ok(())
}
fn parse_leash(mut nbt_data: HashMap<String, Tag>) -> Result<Leash, LeashError> {
    if let Some(Tag::IntArray(uuid)) = nbt_data.remove("UUID") {
        return Ok(Leash::Entity(uuid));
    }
    if let (Some(Tag::Int(x)), Some(Tag::Int(y)), Some(Tag::Int(z))) = (
        nbt_data.remove("X"),
        nbt_data.remove("Y"),
        nbt_data.remove("Z"),
    ) {
        return Ok(Leash::Position { x, y, z });
    }
    Err(crate::nbt::Error::InvalidValue.into())
}
