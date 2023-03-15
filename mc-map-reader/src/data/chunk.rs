use std::collections::HashMap;

use getset::{CopyGetters, Getters};
use jbe::Builder;

use crate::nbt::{Array, List, Tag};

#[cfg(feature = "block_entity")]
use super::block_entity::BlockEntity;

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
    #[cfg(feature = "chunk_section")]
    #[get = "pub"]
    sections: List<Section>,
    #[cfg(feature = "block_entity")]
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

#[cfg(feature = "block_entity")]
impl ChunkData {
    pub fn block_entities(&self) -> Option<&List<BlockEntity>> {
        self.block_entities.as_ref()
    }
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
#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct Section {
    #[get_copy = "pub"]
    y: i8,
    #[get = "pub"]
    block_states: BlockStates,
    #[get = "pub"]
    biomes: Biomes,
    block_light: Option<Array<i8>>,
    sky_light: Option<Array<i8>>,
}

#[cfg(feature = "chunk_section")]
impl Section {
    pub fn block_light(&self) -> Option<&Array<i8>> {
        self.block_light.as_ref()
    }

    pub fn sky_light(&self) -> Option<&Array<i8>> {
        self.sky_light.as_ref()
    }
}

#[cfg(feature = "chunk_section")]
#[derive(Debug, Builder, Getters)]
pub struct BlockStates {
    #[get = "pub"]
    palette: List<BlockState>,
    data: Option<Array<i64>>,
}

#[cfg(feature = "chunk_section")]
impl BlockStates {
    pub fn data(&self) -> Option<&Array<i64>> {
        self.data.as_ref()
    }
}

#[derive(Debug, Builder, Getters)]
pub struct Biomes {
    #[get = "pub"]
    palette: List<String>,
    data: Option<Array<i64>>,
}

impl Biomes {
    pub fn data(&self) -> Option<&Array<i64>> {
        self.data.as_ref()
    }
}

#[cfg(feature = "chunk_section")]
#[derive(Debug, Builder, Getters, Clone)]
pub struct BlockState {
    #[get = "pub"]
    name: String,
    properties: Option<HashMap<String, crate::nbt::Tag>>,
}

#[cfg(feature = "chunk_section")]
impl BlockState {
    pub fn properties(&self) -> Option<&HashMap<String, Tag>> {
        self.properties.as_ref()
    }
}
