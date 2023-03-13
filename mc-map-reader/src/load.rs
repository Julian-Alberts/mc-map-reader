use std::{fs::OpenOptions, io::Read};

#[cfg(feature = "parallel")]
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use thiserror::Error;

use crate::{
    file_format::anvil::{self, AnvilSave},
    nbt_data,
};

pub struct Loader;

impl LoadMcSave<AnvilSave> for Loader {
    fn load(&self, path: &str) -> Result<AnvilSave> {
        let mut file = OpenOptions::new().read(true).write(false).open(path)?;
        self.load_from_bytes(&mut file)
    }

    fn load_from_bytes(&self, mut read: impl Read) -> Result<AnvilSave> {
        let mut raw_header = [0; anvil::header::MC_REGION_HEADER_SIZE];
        if read.read(&mut raw_header)? != anvil::header::MC_REGION_HEADER_SIZE {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                anvil::header::INVALID_HEADER_MESSAGE,
            )
            .into());
        }
        let header = anvil::header::McRegionHeader::from(raw_header);
        let mut raw_chunk_data = Vec::default();
        read.read_to_end(&mut raw_chunk_data)?;

        #[cfg(feature = "parallel")]
        let chunk_info = header.get_chunk_info().par_iter();
        #[cfg(not(feature = "parallel"))]
        let chunk_info = header.get_chunk_info().iter();
        let chunks = chunk_info
            .filter_map(|ci| ci.as_ref())
            .map(|chunk| nbt_data::load::chunk::load_chunk(&raw_chunk_data, chunk))
            .collect::<Result<_>>()?;

        Ok(AnvilSave::new(header, chunks))
    }
}

pub trait LoadMcSave<S> {
    fn load(&self, path: &str) -> Result<S>;
    fn load_from_bytes(&self, read: impl Read) -> Result<S>;
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Decode(#[from] crate::compression::Error),
    #[error(transparent)]
    NBT(#[from] crate::nbt::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
