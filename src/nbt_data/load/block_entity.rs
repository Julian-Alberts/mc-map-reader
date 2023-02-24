use thiserror::Error;

use crate::{
    nbt::Tag,
    nbt_data::{block_entity::*, chunk::MissingData},
};

pub fn load(data: &Tag) -> crate::load::Result<BlockEntity> {
    let nbt_data = data.get_as_map()?;
    let id = nbt_data
        .get("tag")
        .ok_or(BlockEntityBuilderError::UnsetId)
        .map_err(BlockEntityMissingDataError::from)
        .map_err(MissingData::from)?;
}

#[derive(Debug, Error)]
pub enum BlockEntityMissingDataError {
    #[error(transparent)]
    BlockEntityBuilderError(#[from] BlockEntityBuilderError),
}
