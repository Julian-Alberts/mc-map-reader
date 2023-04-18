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
const CHUNK_ALIGNMENT: u32 = KIB * 4;

/// Errors that can occur when loading chunk data.
#[derive(Debug, Error, PartialEq)]
pub enum LoadChunkDataError {
    /// The chunk data is not valid.
    #[error(transparent)]
    ChunkData(#[from] ChunkDataError),
    /// The chunk data length could not be parsed.
    #[error("Could not parse chunk data length")]
    ChunkDataLengthError,
    /// The chunk data could not be decompressed.
    #[error(transparent)]
    Compression(compression::Error),
}

/// Load chunk data from a region file.
pub fn load_chunk(raw: &[u8], chunk_info: &ChunkInfo) -> Result<ChunkData, LoadChunkDataError> {
    let offset = ((chunk_info.offset - 2) * CHUNK_ALIGNMENT) as usize;
    let chunk_data = &raw[offset..];
    if chunk_data.len() < 6 {
        return Err(LoadChunkDataError::ChunkDataLengthError);
    }
    let chunk_len = u32::from_be_bytes(
        chunk_data[..4]
            .try_into()
            .map_err(|_| LoadChunkDataError::ChunkDataLengthError)?,
    );
    let compression = chunk_data[4].into();

    if chunk_data.len() < chunk_len as usize || chunk_len < 5 {
        return Err(LoadChunkDataError::ChunkDataLengthError);
    }

    let data = &chunk_data[5..chunk_len as usize];

    let data = decompress(data, &compression).map_err(LoadChunkDataError::Compression)?;
    let tag = crate::nbt::parse(data.as_slice()).map_err(ChunkDataError::Nbt)?;
    let chunk_data = tag.try_into()?;
    Ok(chunk_data)
}

mod_try_from_tag!(ChunkData: [
    "DataVersion" => set_data_version test(crate::nbt::Tag::Int(1) => data_version = 1),
    "xPos" => set_x_pos test(crate::nbt::Tag::Int(2) => x_pos = 2),
    "yPos" => set_y_pos test(crate::nbt::Tag::Int(3) => y_pos = 3),
    "zPos" => set_z_pos test(crate::nbt::Tag::Int(4) => z_pos = 4),
    "Status" => set_status test(crate::nbt::Tag::String("empty".to_string()) => status = ChunkStatus::Empty),
    "LastUpdate" => set_last_update test(crate::nbt::Tag::Long(5) => last_update = 5),
    if feature = "chunk_section" "sections" => set_sections test(crate::nbt::Tag::List(crate::nbt::List::from(vec![])) => sections = crate::nbt::List::from(vec![])),
    if feature = "block_entity" "block_entities" => set_block_entities test(crate::nbt::Tag::List(crate::nbt::List::from(vec![])) => block_entities = Some(crate::nbt::List::from(vec![]))),
] ? [
    ChunkStatus,
    if feature = "chunk_section" Section,
    if feature = "block_entity" BlockEntity,
],
if feature = "chunk_section" Section: [
    "Y" => set_y test(1i8 => y = 1),
    "block_states" => set_block_states test(std::collections::HashMap::from_iter([
        ("palette".to_string(), crate::nbt::Tag::List(vec![].into())),
        ("data".to_string(), crate::nbt::Tag::LongArray(vec![].into()))
    ]) => block_states = BlockStates {
        palette: vec![].into(),
        data: Some(vec![].into()),
    }),
    "biomes" => set_biomes test(std::collections::HashMap::from_iter([
        ("palette".to_string(), crate::nbt::Tag::List(vec![].into())),
        ("data".to_string(), crate::nbt::Tag::LongArray(vec![].into()))
    ]) => biomes = Biomes {
        palette: vec![].into(),
        data: Some(vec![].into()),
    }),
    "block_light" => set_block_light test(crate::nbt::Tag::ByteArray(vec![].into()) => block_light = Some(vec![].into())),
    "sky_light" => set_sky_light test(crate::nbt::Tag::ByteArray(vec![].into()) => sky_light = Some(vec![].into())),
] ? [
    BlockStates,
    Biomes,
],
Biomes: [
    "palette" => set_palette
    test(crate::nbt::List::from(
        vec![
            "a".to_string().into(),
            "b".to_string().into()
        ]
    ) => palette = crate::nbt::List::from(vec![
        "a".to_string(),
        "b".to_string()
    ])),
    "data" => set_data
    test(crate::nbt::Array::from(
        vec![
            1i64,2
        ]
    ) => data = Some(crate::nbt::Array::from(vec![
        1,2
    ]))),
],
if feature = "chunk_section" BlockStates: [
    "palette" => set_palette test(crate::nbt::List::from(vec![]) => palette = crate::nbt::List::from(vec![])),
    "data" => set_data test(crate::nbt::Array::from(vec![1i64]) => data = Some(crate::nbt::Array::from(vec![1i64]))),
] ? [
    BlockState,
],
if feature = "chunk_section" BlockState: [
    "Name" => set_name test("a".to_string() => name = "a".to_string()),
    "Properties" => set_properties test(std::collections::HashMap::new() => properties = Some(std::collections::HashMap::new())),
],
);
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

#[cfg(test)]
mod tests {
    use crate::{
        data::{
            chunk::{ChunkData, ChunkStatus},
            file_format::anvil::ChunkInfo,
        },
        nbt::Tag,
    };
    use test_case::test_case;

    use super::{load_chunk, ChunkStatusError, LoadChunkDataError};

    #[test_case(Tag::String("empty".to_string()) => Ok(ChunkStatus::Empty); "empty")]
    #[test_case(Tag::String("structure_starts".to_string()) => Ok(ChunkStatus::StructureStarts); "structure_starts")]
    #[test_case(Tag::String("structure_references".to_string()) => Ok(ChunkStatus::StructureReferences); "structure_references")]
    #[test_case(Tag::String("biomes".to_string()) => Ok(ChunkStatus::Biomes); "biomes")]
    #[test_case(Tag::String("noise".to_string()) => Ok(ChunkStatus::Noise); "noise")]
    #[test_case(Tag::String("surface".to_string()) => Ok(ChunkStatus::Surface); "surface")]
    #[test_case(Tag::String("carvers".to_string()) => Ok(ChunkStatus::Carvers); "carvers")]
    #[test_case(Tag::String("liquid_carvers".to_string()) => Ok(ChunkStatus::LiquidCarvers); "liquid_carvers")]
    #[test_case(Tag::String("features".to_string()) => Ok(ChunkStatus::Features); "features")]
    #[test_case(Tag::String("light".to_string()) => Ok(ChunkStatus::Light); "light")]
    #[test_case(Tag::String("spawn".to_string()) => Ok(ChunkStatus::Spawn); "spawn")]
    #[test_case(Tag::String("heightmaps".to_string()) => Ok(ChunkStatus::Heightmaps); "heightmaps")]
    #[test_case(Tag::String("full".to_string()) => Ok(ChunkStatus::Full); "full")]
    #[test_case(Tag::String("invalid".to_string()) => Err(ChunkStatusError::Nbt(crate::nbt::Error::InvalidValue)); "invalid")]
    #[test_case(Tag::Int(1) => Err(ChunkStatusError::Nbt(crate::nbt::Error::InvalidValue)); "invalid type")]
    fn test_chunk_status_try_from_tag(tag: Tag) -> Result<ChunkStatus, ChunkStatusError> {
        ChunkStatus::try_from(tag)
    }

    #[test_case(&[] => Err(LoadChunkDataError::ChunkDataLengthError); "empty")]
    #[test_case(&[0, 0, 0, 0] => Err(LoadChunkDataError::ChunkDataLengthError); "missing compression type")]
    #[test_case(&[0, 0, 0, 0, 0] => Err(LoadChunkDataError::ChunkDataLengthError); "no sectors")]
    #[test_case(&[0, 0, 0, 1, 0, 0] => Err(LoadChunkDataError::ChunkDataLengthError); "length to small")]
    #[test_case(&[0, 0, 0, 6, 1, 1] => Err(LoadChunkDataError::Compression(
        crate::compression::Error::Io(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "failed to fill whole buffer"))
    )); "Invalid data")]
    #[test_case(&valid_chunk_data() => Ok(ChunkData {
        data_version: 1234,
        x_pos: 1234,
        y_pos: 1234,
        z_pos: 1234,
        status: ChunkStatus::Full,
        last_update: 10,
        sections: crate::nbt::List::from(vec![]),
        block_entities: None
    }); "Success")]
    fn test_load_chunk_status(raw: &[u8]) -> Result<ChunkData, LoadChunkDataError> {
        load_chunk(
            raw,
            &ChunkInfo {
                offset: 2,
                sector_count: 0,
                timestamp: 0,
            },
        )
    }

    fn valid_chunk_data() -> Vec<u8> {
        const INT_ID: u8 = 3;
        const LONG_ID: u8 = 4;
        const STRING_ID: u8 = 8;
        const LIST_ID: u8 = 9;
        const COMPOUND_ID: u8 = 10;
        fn push_str(data: &mut Vec<u8>, string: &str) {
            data.extend((string.len() as i16).to_be_bytes());
            data.extend(string.as_bytes());
        }
        let mut data = Vec::new();
        data.extend([0, 0, 0, 0, 3]);
        data.extend([COMPOUND_ID, 0, 0, INT_ID]);
        push_str(&mut data, "DataVersion");
        data.extend(1234i32.to_be_bytes());
        data.push(INT_ID);
        push_str(&mut data, "xPos");
        data.extend(1234i32.to_be_bytes());
        data.push(INT_ID);
        push_str(&mut data, "yPos");
        data.extend(1234i32.to_be_bytes());
        data.push(INT_ID);
        push_str(&mut data, "zPos");
        data.extend(1234i32.to_be_bytes());
        data.push(STRING_ID);
        push_str(&mut data, "Status");
        push_str(&mut data, "full");
        data.push(LONG_ID);
        push_str(&mut data, "LastUpdate");
        data.extend(10_i64.to_be_bytes());
        data.push(LIST_ID);
        push_str(&mut data, "sections");
        data.push(COMPOUND_ID);
        data.extend(0_i32.to_be_bytes());
        let data_len = (data.len() as u32).to_be_bytes();
        data[0..4].copy_from_slice(&data_len);
        data
    }
}
