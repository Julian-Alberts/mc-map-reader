use thiserror::Error;

use crate::{
    compression::{self, decompress},
    data::chunk::*,
    data::file_format::anvil::ChunkInfo,
};

#[cfg(feature = "block_entity")]
use crate::data::load::block_entity::BlockEntityError;

/// 1KiB
const KIB: u32 = 1024;
/// The alignment of chunks in the region file.
pub const CHUNK_ALIGNMENT: u32 = KIB * 4;

#[derive(Debug, Error)]
pub enum LoadChunkDataError {
    #[error(transparent)]
    ChunkData(#[from] ChunkDataError),
    #[error(transparent)]
    Compression(compression::Error),
}

pub fn load_chunk(raw: &[u8], chunk_info: &ChunkInfo) -> Result<ChunkData, LoadChunkDataError> {
    let offset = ((chunk_info.offset - 2) * CHUNK_ALIGNMENT) as usize;
    let chunk_data = &raw[offset..];
    let chunk_len = u32::from_be_bytes(chunk_data[..4].try_into().expect("Length does not match"));
    let compression = chunk_data[4].into();
    let data = &chunk_data[5..chunk_len as usize];

    let data = decompress(data, &compression).map_err(|e| LoadChunkDataError::Compression(e))?;
    let tag = crate::nbt::parse(data.as_slice()).map_err(|e| ChunkDataError::Nbt(e))?;
    let chunk_data = tag.try_into()?;
    Ok(chunk_data)
}

try_from_tag!(ChunkData => [
    "DataVersion": set_data_version,
    "xPos": set_x_pos,
    "yPos": set_y_pos,
    "zPos": set_z_pos,
    "Status": set_status,
    "LastUpdate": set_last_update,
    "sections": set_sections feature = "chunk_section",
    "block_entities": set_block_entities feature = "block_entity",
] ? [
    ChunkStatus,
    if feature = "chunk_section" Section,
    if feature = "block_entity" BlockEntity,
]);

#[cfg(feature = "chunk_section")]
try_from_tag!(Section => [
    "Y": set_y,
    "block_states": set_block_states,
    "biomes": set_biomes,
] ? [
    BlockStates,
    Biomes,
]);

try_from_tag!(Biomes => [
    "palette": set_palette,
    "data": set_data,
]);

#[cfg(feature = "chunk_section")]
try_from_tag!(BlockStates => [
    "palette": set_palette,
    "data": set_data,
] ? [
    BlockState,
]);

#[cfg(feature = "chunk_section")]
try_from_tag!(BlockState => [
    "Name": set_name,
    "Properties": set_properties,
]);
try_from_tag!(error ChunkStatus => []);

impl TryFrom<crate::nbt::Tag> for ChunkStatus {
    type Error = ChunkStatusError;

    fn try_from(value: crate::nbt::Tag) -> Result<Self, Self::Error> {
        let status = match value
            .get_as_string()
            .or(Err(crate::nbt::Error::InvalidValue))?
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
            _ => return Err(crate::nbt::Error::InvalidValue.into()),
        };
        Ok(status)
    }
}
