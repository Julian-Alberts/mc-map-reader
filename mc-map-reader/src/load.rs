use std::io::Read;

#[cfg(feature = "parallel")]
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use thiserror::Error;

use crate::{
    compression,
    data::file_format::anvil::{self, AnvilSave},
    data::{
        self,
        file_format::level_dat::{self, LevelDat},
    },
};

#[derive(Error, Debug)]
pub enum RegionLoadError {
    #[error(transparent)]
    Decode(crate::compression::Error),
    #[error(transparent)]
    NBT(#[from] crate::nbt::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    LoadChunkData(#[from] data::load::chunk::LoadChunkDataError),
}

#[derive(Error, Debug)]
pub enum LevelDatLoadError {
    #[error(transparent)]
    NBT(#[from] crate::nbt::Error),
    #[error(transparent)]
    Compression(crate::compression::Error),
    #[error(transparent)]
    LevelDat(#[from] data::load::file_format::level_dat::LevelDatError),
}

pub fn parse_level_dat(data: &[u8]) -> std::result::Result<level_dat::LevelDat, LevelDatLoadError> {
    let data = compression::decompress(data, &compression::Compression::GZip)
        .map_err(LevelDatLoadError::Compression)?;
    let data = crate::nbt::parse(data.as_slice())?;
    LevelDat::try_from(data).map_err(LevelDatLoadError::LevelDat)
}

pub fn load_region(
    mut read: impl Read,
    ignore_saved_before: Option<i32>,
) -> Result<AnvilSave, RegionLoadError> {
    let mut raw_header = [0; anvil::MC_REGION_HEADER_SIZE];
    if read.read(&mut raw_header)? != anvil::MC_REGION_HEADER_SIZE {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            anvil::INVALID_HEADER_MESSAGE,
        )
        .into());
    }
    let header = anvil::McRegionHeader::from(raw_header);
    let mut raw_chunk_data = Vec::default();
    read.read_to_end(&mut raw_chunk_data)?;

    #[cfg(feature = "parallel")]
    let chunk_info = header.get_chunk_info().par_iter();
    #[cfg(not(feature = "parallel"))]
    let chunk_info = header.get_chunk_info().iter();
    let chunks = chunk_info
        .filter_map(|ci| ci.as_ref())
        .filter(|chunk_info| {
            ignore_saved_before.is_none()
                || chunk_info.timestamp as i32 >= ignore_saved_before.unwrap()
        })
        .map(|chunk| data::load::chunk::load_chunk(&raw_chunk_data, chunk))
        .collect::<std::result::Result<_, _>>()?;

    Ok(AnvilSave::new(header, chunks))
}
