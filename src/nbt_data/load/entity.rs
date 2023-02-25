use thiserror::Error;

use crate::nbt_data::{entity::*, chunk::MissingData};

impl TryFrom<crate::nbt::Tag> for Entity {
    type Error = crate::nbt::Error;
    fn try_from(nbt_entity: crate::nbt::Tag) -> Result<Self, Self::Error> {
        parse_entity_from_tag(nbt_entity)
    }
}

fn parse_entity_from_tag(nbt_entity: crate::nbt::Tag) -> Result<Entity,crate::nbt::Error> {
    let mut nbt_entity = nbt_entity.get_as_map()?;
    let mut entity_builder = EntityBuilder::default();
    
    add_data_to_builder!(entity_builder, nbt_entity => [
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
        "UUID": set_uuid
    ]);
    let entity = entity_builder.try_build().map_err(EntityMissingDataError::from).map_err(MissingData::from)?;
    Ok(entity)
}

#[derive(Debug, Error)]
pub enum EntityMissingDataError {
    #[error(transparent)]
    BlockEntity(#[from] EntityBuilderError),
}