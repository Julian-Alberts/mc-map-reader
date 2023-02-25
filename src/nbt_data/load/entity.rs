use thiserror::Error;

use crate::nbt_data::entity::*;

pub(crate) fn parse_entity_from_tag(value: crate::nbt::Tag) -> crate::load::Result<Entity> {
    todo!()
}

#[derive(Debug, Error)]
pub enum EntityMissingDataError {
    #[error(transparent)]
    BlockEntity(#[from] EntityBuilderError),
}