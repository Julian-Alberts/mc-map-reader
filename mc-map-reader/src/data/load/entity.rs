use std::collections::HashMap;

use thiserror::Error;

use crate::{
    data::{chunk::MissingData, entity::*},
    nbt::Tag,
};

try_from_tag!(
    Entity, EntityBuilder => [
        "Air": set_air,
        "CustomName": set_custom_name,
        "CustomNameVisible": set_custom_name_visible,
        "FallDistance": set_fall_distance,
        "Fire": set_fire,
        "Glowing": set_glowing,
        "HasVisualFire": set_has_visual_fire,
        "id": set_id,
        "Invulnerable": set_invulnerable,
        "Motion": set_motion,
        "NoGravity": set_no_gravity,
        "OnGround": set_on_ground,
        "Passengers": set_passengers,
        "PortalCooldown": set_portal_colldown,
        "Pos": set_pos,
        "Rotation": set_rotation,
        "Silent": set_silent,
        "Tags": set_tags,
        "TicksFrozen": set_ticks_frozen,
        "UUID": set_uuid,
    ]); 
    /*TODO Enable Mob
try_from_tag!(Mob, MobBuilder => parse_mob [ Entity, ]);

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
    builder.set_entity(nbt_data.try_into()?);
    Ok(())
}
*/
