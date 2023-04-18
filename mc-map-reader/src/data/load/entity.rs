use std::collections::HashMap;

use crate::{
    data::entity::*,
    data::{load::item::ItemError, FieldError},
    nbt::*,
};

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
    builder.set_entity(
        nbt_data
            .try_into()
            .map_err(|e| FieldError::new("<internal> entity", e))?,
    );
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

#[cfg(test)]
pub mod tests {
    use super::{
        macro_tests::{Entity_test_data_provider, Entity_test_result},
        *,
    };
    use test_case::test_case;

    #[test_case(vec![
        ("UUID", Tag::IntArray(Array::from(vec![1, 2, 3, 4])))
    ] => Ok(Leash::Entity(Array::from(vec![1,2,3,4]))); "Success UUID")]
    #[test_case(vec![
        ("X", Tag::Int(1)),
        ("Y", Tag::Int(2)),
        ("Z", Tag::Int(3))
    ] => Ok(Leash::Position { x: 1, y: 2, z: 3 }); "Success Position")]
    #[test_case(vec![
        ("Y", Tag::Int(2)),
        ("Z", Tag::Int(3))
    ] => Err(LeashError::Nbt(crate::nbt::Error::InvalidValue)); "Error Position X")]
    #[test_case(vec![
        ("X", Tag::Int(1)),
        ("Z", Tag::Int(3))
    ] => Err(LeashError::Nbt(crate::nbt::Error::InvalidValue)); "Error Position Y")]
    #[test_case(vec![
        ("X", Tag::Int(1)),
        ("Y", Tag::Int(2)),
    ] => Err(LeashError::Nbt(crate::nbt::Error::InvalidValue)); "Error Position Z")]
    fn test_parse_leash(nbt_data: Vec<(&str, Tag)>) -> Result<Leash, LeashError> {
        let data: HashMap<_, _> = nbt_data
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect();
        data.try_into()
    }

    #[test_case(None, None => Ok(mob_test_result()); "Success")]
    #[test_case(
        Some("Fire"), Some(Tag::Double(42.)) =>
        Err(MobError::EntityField(
            FieldError::new("<internal> entity", EntityError::NbtField(
                FieldError::new("Fire", crate::nbt::Error::InvalidValue)
            ))
        )); "Entity Error"
    )]
    fn test_parse_mob(remove: Option<&str>, new_value: Option<Tag>) -> Result<Mob, MobError> {
        let mut builder = MobBuilder::default();
        parse_mob(&mut builder, data_parse_mob(remove, new_value))?;
        Ok(builder.try_build()?)
    }

    fn data_parse_mob(key: Option<&str>, new_value: Option<Tag>) -> HashMap<String, Tag> {
        let mut data = mob_test_data_provider();
        match (key, new_value) {
            (Some(key), Some(value)) => {
                data.insert(key.to_string(), value);
            }
            (Some(key), None) => {
                data.remove(key);
            }
            _ => {}
        }
        data
    }

    pub fn mob_test_data_provider() -> HashMap<String, Tag> {
        let mut map = HashMap::from_iter(
            [
                ("AbsorptionAmount", Tag::Float(42.)),
                ("ActiveEffects", List::from(vec![]).into()),
                ("ArmorDropChances", List::from(vec![]).into()),
                ("ArmorItems", List::from(vec![]).into()),
                ("Attributes", List::from(vec![]).into()),
                ("Brain", HashMap::new().into()),
                ("CanPickUpLoot", Tag::Byte(0)),
                ("DeathLootTable", Tag::String("loot_table".to_string())),
                ("DeathLootTableSeed", Tag::Long(0)),
                ("DeathTime", Tag::Short(0)),
                ("FallFlying", Tag::Byte(0)),
                ("Health", Tag::Float(0.)),
                ("HurtByTimestamp", Tag::Int(0)),
                ("HurtTime", Tag::Short(0)),
                ("HandDropChances", List::from(vec![]).into()),
                ("HandItems", List::from(vec![]).into()),
                (
                    "Leash",
                    Tag::Compound(HashMap::from_iter([(
                        "UUID".to_string(),
                        Tag::IntArray(Array::from(vec![1, 2, 3, 4])),
                    )])),
                ),
                ("LeftHanded", Tag::Byte(0)),
                ("NoAI", Tag::Byte(0)),
                ("PersistenceRequired", Tag::Byte(0)),
                ("SleepingX", Tag::Int(0)),
                ("SleepingY", Tag::Int(0)),
                ("SleepingZ", Tag::Int(0)),
                ("Team", Tag::String(String::new())),
                ("Air", Tag::Short(1)),
                ("CustomName", Tag::String(String::from("name"))),
                ("CustomNameVisible", Tag::Byte(0)),
                ("FallDistance", Tag::Float(0.)),
                ("Fire", Tag::Short(1)),
                ("Glowing", Tag::Byte(0)),
                ("Invulnerable", Tag::Byte(0)),
                ("OnGround", Tag::Byte(0)),
                ("Passengers", List::from(vec![]).into()),
                ("PortalCooldown", Tag::Int(0)),
                (
                    "Pos",
                    List::from(vec![Tag::Float(0.), Tag::Float(0.), Tag::Float(0.)]).into(),
                ),
                ("Tags", HashMap::new().into()),
                ("TicksFrozen", Tag::Int(0)),
                (
                    "Rotation",
                    List::from(vec![Tag::Float(0.), Tag::Float(0.)]).into(),
                ),
                ("UUID", Tag::IntArray(Array::from(vec![1, 2, 3, 4]))),
                ("id", Tag::String(String::from("id"))),
                (
                    "Motion",
                    List::from(vec![Tag::Double(0.), Tag::Double(0.), Tag::Double(0.)]).into(),
                ),
            ]
            .map(|(k, v)| (k.to_string(), v)),
        );
        map.extend(Entity_test_data_provider().into_iter());
        map
    }

    pub fn mob_test_result() -> Mob {
        Mob {
            absorption_amount: Some(42.),
            active_effects: Some(List::from(vec![])),
            armor_drop_chances: Some(List::from(vec![])),
            armor_items: Some(List::from(vec![])),
            attributes: Some(List::from(vec![])),
            brain: Some(HashMap::new()),
            can_pick_up_loot: Some(false),
            death_loot_table: Some("loot_table".to_string()),
            death_loot_table_seed: Some(0),
            death_time: Some(0),
            fall_flying: Some(false),
            health: Some(0.),
            hurt_by_timestamp: Some(0),
            entity: Entity_test_result(),
            hand_drop_chances: Some(List::from(vec![])),
            hand_items: Some(List::from(vec![])),
            hurt_time: Some(0),
            leash: Some(Leash::Entity(Array::from(vec![1, 2, 3, 4]))),
            left_handed: Some(false),
            no_ai: Some(false),
            persistence_required: Some(false),
            sleeping_x: Some(0),
            sleeping_y: Some(0),
            sleeping_z: Some(0),
            team: Some(String::new()),
        }
    }
}
