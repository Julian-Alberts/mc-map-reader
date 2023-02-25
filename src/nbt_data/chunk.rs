use std::collections::HashMap;

use getset::{CopyGetters, Getters};
use jbe::Builder;
use thiserror::Error;

use crate::nbt::{Array, List, Tag};

use super::{block_entity::BlockEntity, load::entity::EntityMissingDataError};

#[derive(jbe::Builder, Debug, Getters, CopyGetters)]
pub struct ChunkData {
    #[get_copy = "pub"]
    data_version: i32,
    #[get_copy = "pub"]
    x_pos: i32,
    #[get_copy = "pub"]
    y_pos: i32,
    #[get_copy = "pub"]
    z_pos: i32,
    #[get_copy = "pub"]
    status: ChunkStatus,
    #[get_copy = "pub"]
    last_update: i64,
    #[get = "pub"]
    sections: List<Section>,
    #[get = "pub"]
    block_entities: Option<List<BlockEntity>>, /*#[get = "pub"]
                                               carving_masks: Option<()>,
                                               #[get = "pub"]
                                               height_maps: (),
                                               #[get = "pub"]
                                               lights: Vec<i16>,
                                               #[get = "pub"]
                                               entities: Vec<()>,
                                               #[get = "pub"]
                                               fluid_ticks: Vec<()>,
                                               #[get = "pub"]
                                               block_ticks: Vec<()>,
                                               #[get_copy = "pub"]
                                               inhabited_time: i64,
                                               #[get = "pub"]
                                               post_processing: Vec<()>*/
}

#[derive(Debug, Clone, Copy)]
pub enum ChunkStatus {
    Empty,
    StructureStarts,
    StructureReferences,
    Biomes,
    Noise,
    Surface,
    Carvers,
    LiquidCarvers,
    Features,
    Light,
    Spawn,
    Heightmaps,
    Full,
}

#[derive(Debug, Builder)]
pub struct Section {
    y: i8,
    block_states: BlockStates,
    biomes: Biomes,
    block_light: Option<Array<i8>>,
    sky_light: Option<Array<i8>>,
}

#[derive(Debug, Builder)]
pub struct BlockStates {
    palette: List<BlockState>,
    data: Option<Array<i64>>,
}

#[derive(Debug, Builder)]
pub struct Biomes {
    palette: List<String>,
    data: Option<Array<i64>>,
}

#[derive(Debug, Builder)]
pub struct BlockState {
    name: String,
    properties: Option<HashMap<String, crate::nbt::Tag>>,
}

#[derive(Debug, Error)]
pub enum ChunkStatusError {
    #[error("Unknown status")]
    UnknownStatus,
    #[error("Invalid value")]
    InvalidValue,
}

#[derive(Debug, Error)]
pub enum MissingData {
    #[error(transparent)]
    SectionData(#[from] SectionBuilderError),
    #[error(transparent)]
    BlockStatesData(#[from] BlockStatesBuilderError),
    #[error(transparent)]
    BlockStateData(#[from] BlockStateBuilderError),
    #[error(transparent)]
    ChunkData(#[from] ChunkDataBuilderError),
    #[error(transparent)]
    BiomesData(#[from] BiomesBuilderError),
    #[error(transparent)]
    BlockEntityData(#[from] super::load::block_entity::BlockEntityMissingDataError),
    #[error(transparent)]
    EntityData(#[from] EntityMissingDataError),
}

impl TryFrom<Tag> for ChunkStatus {
    type Error = ChunkStatusError;

    fn try_from(value: Tag) -> Result<Self, Self::Error> {
        let status = match value
            .get_as_string()
            .or(Err(ChunkStatusError::InvalidValue))?
            .as_str()
        {
            "empty" => Self::Empty,
            "structure_starts" => Self::StructureStarts,
            "structure_references" => Self::StructureReferences,
            "biomes" => Self::Biomes,
            "noise" => Self::Noise,
            "surface" => Self::Surface,
            "carvers" => Self::Carvers,
            "liquid_carvers" => Self::LiquidCarvers,
            "features" => Self::Features,
            "light" => Self::Light,
            "spawn" => Self::Spawn,
            "heightmaps" => Self::Heightmaps,
            "full" => Self::Full,
            _ => return Err(ChunkStatusError::UnknownStatus),
        };
        Ok(status)
    }
}
