use std::{collections::HashMap, io::Read};

use crate::{
    data::*,
    nbt::{self, Tag},
};

use super::{header::ChunkInfo, CHUNK_ALLIGNMENT};

pub fn load_chunk(raw: &[u8], chunk_info: &ChunkInfo) -> crate::load::Result<ChunkData> {
    let offset = ((chunk_info.offset - 2) * CHUNK_ALLIGNMENT) as usize;
    let chunk_data = &raw[offset..];
    let chunk_len = u32::from_be_bytes(chunk_data[..4].try_into().expect("Length does not match"));
    let compression = chunk_data[4].into();
    let data = &chunk_data[5..chunk_len as usize];

    let data = decompress(data, &compression)?;
    let tag = crate::nbt::parse(data.as_slice())?;
    nbt_to_chunk_data(tag)
}

fn nbt_to_chunk_data(tag: Tag) -> crate::load::Result<ChunkData> {
    let mut cdb = ChunkDataBuilder::default();
    let root = if let Tag::Compound(root) = tag {
        root
    } else {
        return Err(crate::Error::NBT(nbt::Error::InvalidValue));
    };

    for (key, value) in root.iter() {
        match key.as_str() {
            "DataVersion" => {
                cdb.data_version(*value.get_as_i32()?);
            }
            "xPos" => {
                cdb.x_pos(*value.get_as_i32()?);
            }
            "yPos" => {
                cdb.y_pos(*value.get_as_i32()?);
            }
            "zPos" => {
                cdb.z_pos(*value.get_as_i32()?);
            }
            "Status" => {
                cdb.status(value.get_as_string()?.as_str().try_into()?);
            }
            "LastUpdate" => {
                cdb.last_update(*value.get_as_i64()?);
            }
            "sections" => {
                cdb.sections(nbt_to_sections(value)?);
            }
            _ => {}
        }
    }
    Ok(cdb.try_build().unwrap())
}

fn nbt_to_sections(nbt_sections: &Tag) -> crate::load::Result<Vec<Section>> {
    let Tag::List(nbt_sections) = nbt_sections else {
        return Err(nbt::Error::InvalidValue.into());
    };
    let mut sections = Vec::with_capacity(nbt_sections.len());
    for section in nbt_sections.iter() {
        let Tag::Compound(section) = section else {
            return Err(nbt::Error::InvalidValue.into());
        };
        sections.push(nbt_to_section(section)?)
    }
    Ok(sections)
}

fn nbt_to_section(section: &HashMap<String, Tag>) -> crate::load::Result<Section> {
    let mut section_builder = SectionBuilder::default();
    for (key, value) in section {
        match key.as_str() {
            "Y" => section_builder.y(*value.get_as_i8()?),
            "block_states" => section_builder.block_states(nbt_to_block_states(value)?),
            "biomes" => section_builder.biomes(nbt_to_biomes(value)?),
            _ => &mut section_builder,
        };
    }
    Ok(section_builder.try_build().map_err(MissingData::from)?)
}

fn nbt_to_biomes(biomes: &Tag) -> crate::load::Result<Biomes> {
    let Tag::Compound(biomes) = biomes else {
        return Err(nbt::Error::InvalidValue.into())
    };
    let mut bb = BiomesBuilder::default();
    for (key, value) in biomes {
        match key.as_str() {
            "palette" => bb.palette(nbt_to_biome_palette(value)?),
            "data" => bb.data(value.get_as_vec_i64()?.clone()),
            _ => &mut bb,
        };
    }
    Ok(bb.try_build().map_err(MissingData::from)?)
}

fn nbt_to_biome_palette(nbt_palette: &Tag) -> crate::load::Result<Vec<String>> {
    let Tag::List(nbt_palette) = nbt_palette else {
        return Err(nbt::Error::InvalidValue.into())
    };
    let list = nbt_palette
        .iter()
        .map(Tag::get_as_string)
        .map(|s| s.map(String::clone))
        .collect::<Result<_, _>>()?;
    Ok(list)
}

fn nbt_to_block_states(block_states: &Tag) -> crate::load::Result<BlockStates> {
    let Tag::Compound(block_states) = block_states else {
        return Err(nbt::Error::InvalidValue.into())
    };
    let mut block_state_builder = BlockStatesBuilder::default();
    for (key, value) in block_states {
        match key.as_str() {
            "palette" => block_state_builder.palette(nbt_to_block_state_palette(value)?),
            "data" => block_state_builder.data(value.get_as_vec_i64()?.clone()),
            _ => &mut block_state_builder,
        };
    }
    Ok(block_state_builder.try_build().map_err(MissingData::from)?)
}

fn nbt_to_block_state_palette(nbt_palette: &Tag) -> crate::load::Result<Vec<BlockState>> {
    let Tag::List(nbt_palette) = nbt_palette else {
        return Err(nbt::Error::InvalidValue.into());
    };
    let mut palette = Vec::with_capacity(nbt_palette.len());
    for palette_item in nbt_palette.iter() {
        palette.push(nbt_to_block_state_palette_item(palette_item)?)
    }
    Ok(palette)
}

fn nbt_to_block_state_palette_item(palette_item: &Tag) -> crate::load::Result<BlockState> {
    let Tag::Compound(palette_item) = palette_item else {
        return Err(nbt::Error::InvalidValue.into())
    };
    let mut block_state_builder = BlockStateBuilder::default();
    for (key, value) in palette_item {
        match key.as_str() {
            "Name" => block_state_builder.name(value.get_as_string()?.clone()),
            "Properties" => block_state_builder.properties(if let Tag::Compound(p) = value {
                p.clone()
            } else {
                return Err(nbt::Error::InvalidValue.into());
            }),
            _ => &mut block_state_builder,
        };
    }
    Ok(block_state_builder.try_build().map_err(MissingData::from)?)
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
