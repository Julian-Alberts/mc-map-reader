
#[cfg(all(feature = "parallel", feature = "region_file"))]
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use thiserror::Error;

use crate::data;
#[cfg(feature="level_dat")]
use crate::{
    compression,
    data::file_format::level_dat::{self, LevelDat},
};
#[cfg(feature="region_file")]
use {
    crate::data::file_format::anvil::{self, AnvilSave},
    std::io::Read
};

#[cfg(feature =  "region_file")]
/// Errors that can occur when loading a region.
#[derive(Error, Debug)]
pub enum RegionLoadError {
    /// Some data in the region file could not be decompressed.
    #[error(transparent)]
    Decode(crate::compression::Error),
    /// Some data in the region file is not valid NBT.
    #[error(transparent)]
    NBT(#[from] crate::nbt::Error),
    #[error(transparent)]
    /// Error while reading from the region file.
    Io(#[from] std::io::Error),
    /// Error while loading the data of a chunk.
    #[error(transparent)]
    LoadChunkData(#[from] data::chunk::LoadChunkDataError),
}

/// Errors that can occur when loading a level.dat file.
#[derive(Error, Debug)]
pub enum LevelDatLoadError {
    /// Some data in the level.dat file is not valid NBT.
    #[error(transparent)]
    NBT(#[from] crate::nbt::Error),
    /// Some data in the level.dat file could not be decompressed.
    #[error(transparent)]
    Compression(crate::compression::Error),
    #[cfg(feature = "level_dat")]
    /// Some data in the level.dat file is not valid.
    #[error(transparent)]
    LevelDat(#[from] data::file_format::level_dat::LevelDatError),
}

#[cfg(feature = "level_dat")]
/// Parse a level.dat file.
pub fn parse_level_dat(data: &[u8]) -> std::result::Result<level_dat::LevelDat, LevelDatLoadError> {
    let data = compression::decompress(data, &compression::Compression::GZip)
        .map_err(LevelDatLoadError::Compression)?;
    let data = crate::nbt::parse(data.as_slice())?.get_as_map()?.remove("Data").ok_or(crate::nbt::Error::InvalidValue)?;
    LevelDat::try_from(data).map_err(LevelDatLoadError::LevelDat)
}

#[cfg(feature = "region_file")]
/// Load a region file.
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
        .filter(|chunk_info| 
            ignore_saved_before.map_or(true, |ignore_saved_before| chunk_info.timestamp as i32 >= ignore_saved_before)
        )
        .map(|chunk| data::chunk::load_chunk(&raw_chunk_data, chunk))
        .collect::<std::result::Result<_, _>>()?;

    Ok(AnvilSave::new(header, chunks))
}
