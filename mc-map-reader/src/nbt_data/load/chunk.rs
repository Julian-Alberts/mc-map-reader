use std::io::Read;

use crate::{
    file_format::mc_region::header::ChunkInfo,
    nbt::{self, Tag},
    nbt_data::chunk::*,
};

/// 1KiB
const KIB: u32 = 1024;
pub const CHUNK_ALLIGNMENT: u32 = KIB * 4;

pub fn load_chunk(raw: &[u8], chunk_info: &ChunkInfo) -> crate::load::Result<ChunkData> {
    let offset = ((chunk_info.offset - 2) * CHUNK_ALLIGNMENT) as usize;
    let chunk_data = &raw[offset..];
    let chunk_len = u32::from_be_bytes(chunk_data[..4].try_into().expect("Length does not match"));
    let compression = chunk_data[4].into();
    let data = &chunk_data[5..chunk_len as usize];

    let data = decompress(data, &compression)?;
    let tag = crate::nbt::parse(data.as_slice())?;
    let chunk_data = nbt_to_chunk_data(tag)?;
    Ok(chunk_data)
}

fn nbt_to_chunk_data(tag: Tag) -> Result<ChunkData, crate::nbt::Error> {
    let mut cdb = ChunkDataBuilder::default();
    let mut root = if let Tag::Compound(root) = tag {
        root
    } else {
        return Err(nbt::Error::InvalidValue);
    };

    add_data_to_builder!(cdb, root => [
        "DataVersion": set_data_version,
        "xPos": set_x_pos,
        "yPos": set_y_pos,
        "zPos": set_z_pos,
        "Status": set_status,
        "LastUpdate": set_last_update,
        "sections": set_sections feature = "chunk_section",
        "block_entities": set_block_entities feature = "block_entity"
    ]);
    Ok(cdb.try_build().map_err(MissingData::from)?)
}

#[cfg(feature = "chunk_section")]
impl TryFrom<Tag> for Section {
    type Error = crate::nbt::Error;
    fn try_from(section: Tag) -> Result<Self, Self::Error> {
        let mut section_builder = SectionBuilder::default();
        let mut section = section.get_as_map()?;
        add_data_to_builder!(section_builder, section => [
            "Y": set_y,
            "block_states": set_block_states,
            "biomes": set_biomes
        ]);
        Ok(section_builder.try_build().map_err(MissingData::from)?)
    }
}

impl TryFrom<Tag> for Biomes {
    type Error = crate::nbt::Error;
    fn try_from(biomes: Tag) -> Result<Self, Self::Error> {
        let mut bb = BiomesBuilder::default();
        let mut biomes = biomes.get_as_map()?;
        add_data_to_builder!(bb, biomes => [
            "palette": set_palette,
            "data": set_data
        ]);
        Ok(bb.try_build().map_err(MissingData::from)?)
    }
}

#[cfg(feature = "chunk_section")]
impl TryFrom<Tag> for BlockStates {
    type Error = crate::nbt::Error;
    fn try_from(block_states: Tag) -> Result<Self, Self::Error> {
        let mut block_states = block_states.get_as_map()?;
        let mut block_states_builder = BlockStatesBuilder::default();
        add_data_to_builder!(block_states_builder, block_states => [
            "palette": set_palette,
            "data": set_data
        ]);
        Ok(block_states_builder
            .try_build()
            .map_err(MissingData::from)?)
    }
}

#[cfg(feature = "chunk_section")]
impl TryFrom<Tag> for BlockState {
    type Error = crate::nbt::Error;
    fn try_from(palette_item: Tag) -> Result<Self, Self::Error> {
        let mut palette_item = palette_item.get_as_map()?;
        let mut block_state_builder = BlockStateBuilder::default();
        add_data_to_builder!(block_state_builder, palette_item => [
            "Name": set_name,
            "Properties": set_properties
        ]);
        Ok(block_state_builder.try_build().map_err(MissingData::from)?)
    }
}

fn decompress(data: &[u8], compression: &Compression) -> crate::load::Result<Vec<u8>> {
    let mut decompressed = Vec::new();
    match compression {
        Compression::GZip => unimplemented!(),
        Compression::Zlib => {
            compress::zlib::Decoder::new(data).read_to_end(&mut decompressed)?;
        }
        Compression::Uncompressed => unimplemented!(),
        Compression::Other => unimplemented!(),
    }
    Ok(decompressed)
}

pub enum Compression {
    GZip = 1,
    Zlib = 2,
    Uncompressed = 3,
    Other,
}

impl From<u8> for Compression {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::GZip,
            2 => Self::Zlib,
            3 => Self::Uncompressed,
            _ => Self::Other,
        }
    }
}
