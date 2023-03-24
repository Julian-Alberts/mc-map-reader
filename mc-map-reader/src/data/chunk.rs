use std::collections::HashMap;

use jbe::Builder;

use crate::nbt::{Array, List};

#[cfg(feature = "block_entity")]
use super::block_entity::BlockEntity;
pub use super::load::chunk::*;

#[derive(jbe::Builder, Debug)]
pub struct ChunkData {
    pub data_version: i32,
    pub x_pos: i32,
    pub y_pos: i32,
    pub z_pos: i32,
    pub status: ChunkStatus,
    pub last_update: i64,
    #[cfg(feature = "chunk_section")]
    pub sections: List<Section>,
    pub block_entities: Option<List<BlockEntity>>, /*#[get = "pub"]
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

#[cfg(feature = "chunk_section")]
#[derive(Debug, Builder)]
pub struct Section {
    pub y: i8,
    pub block_states: BlockStates,
    pub biomes: Biomes,
    pub block_light: Option<Array<i8>>,
    pub sky_light: Option<Array<i8>>,
}

#[cfg(feature = "chunk_section")]
#[derive(Debug, Builder)]
pub struct BlockStates {
    pub palette: List<BlockState>,
    pub data: Option<Array<i64>>,
}

#[derive(Debug, Builder)]
pub struct Biomes {
    pub palette: List<String>,
    pub data: Option<Array<i64>>,
}

#[cfg(feature = "chunk_section")]
#[derive(Debug, Builder, Clone)]
pub struct BlockState {
    pub name: String,
    pub properties: Option<HashMap<String, crate::nbt::Tag>>,
}
