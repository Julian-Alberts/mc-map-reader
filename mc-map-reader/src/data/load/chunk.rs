use thiserror::Error;

use crate::{
    compression::{self, decompress},
    data::chunk::*,
    data::file_format::anvil::ChunkInfo,
    data::load::block_entity::BlockEntityError,
};

/// 1KiB
const KIB: u32 = 1024;
pub const CHUNK_ALLIGNMENT: u32 = KIB * 4;

#[derive(Debug, Error)]
pub enum LoadChunkDataError {
    #[error(transparent)]
    ChunkData(#[from] ChunkDataError),
    #[error(transparent)]
    Compression(compression::Error),
}

pub fn load_chunk(raw: &[u8], chunk_info: &ChunkInfo) -> Result<ChunkData, LoadChunkDataError> {
    let offset = ((chunk_info.offset - 2) * CHUNK_ALLIGNMENT) as usize;
    let chunk_data = &raw[offset..];
    let chunk_len = u32::from_be_bytes(chunk_data[..4].try_into().expect("Length does not match"));
    let compression = chunk_data[4].into();
    let data = &chunk_data[5..chunk_len as usize];

    let data = decompress(data, &compression).map_err(|e| LoadChunkDataError::Compression(e))?;
    let tag = crate::nbt::parse(data.as_slice()).map_err(|e| ChunkDataError::Nbt(e))?;
    let chunk_data = tag.try_into()?;
    Ok(chunk_data)
}

try_from_tag!(ChunkData, ChunkDataBuilder => [
    "DataVersion": set_data_version,
    "xPos": set_x_pos,
    "yPos": set_y_pos,
    "zPos": set_z_pos,
    "Status" as ChunkStatus: set_status,
    "LastUpdate": set_last_update,
    "sections" as Section: set_sections feature = "chunk_section",
    "block_entities" as BlockEntity: set_block_entities feature = "block_entity",
]);

#[cfg(feature = "chunk_section")]
try_from_tag!(Section, SectionBuilder => [
    "Y": set_y,
    "block_states" as BlockStates: set_block_states,
    "biomes" as Biomes: set_biomes,
]);

try_from_tag!(Biomes, BiomesBuilder => [
    "palette": set_palette,
    "data": set_data,
]);

#[cfg(feature = "chunk_section")]
try_from_tag!(BlockStates, BlockStatesBuilder => [
    "palette" as BlockState: set_palette,
    "data": set_data,
]);

#[cfg(feature = "chunk_section")]
try_from_tag!(BlockState, BlockStateBuilder => [
    "Name": set_name,
    "Properties": set_properties,
]);
