use std::{fs::OpenOptions, io::Read, mem::MaybeUninit, os::raw};

use thiserror::Error;

use crate::{
    data::ChunkData,
    file_format::anvil::{self, AnvilSave},
};

pub struct Loader;

impl LoadMcSave<AnvilSave> for Loader {
    fn load(&self, path: &str) -> Result<AnvilSave> {
        let mut file = OpenOptions::new().read(true).write(false).open(path)?;
        let mut raw_header = [0; anvil::header::MC_REGION_HEADER_SIZE];
        if file.read(&mut raw_header)? != anvil::header::MC_REGION_HEADER_SIZE {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                anvil::header::INVALID_HEADER_MESSAGE,
            )
            .into());
        }
        let header = anvil::header::McRegionHeader::from(raw_header);
        let mut raw_chunk_data = Vec::default();
        file.read_to_end(&mut raw_chunk_data)?;

        let mut chunks: [MaybeUninit<Option<ChunkData>>; 32 * 32] =
            unsafe { MaybeUninit::uninit().assume_init() };
        for chunk in &mut chunks {
            chunk.write(None);
        }
        let mut chunks: [Option<ChunkData>; 32 * 32] = unsafe { std::mem::transmute(chunks) };

        for (index, chunk) in
            header
                .get_chunk_info()
                .iter()
                .enumerate()
                .filter_map(|(index, ci)| {
                    if let Some(ci) = ci {
                        Some((index, ci))
                    } else {
                        None
                    }
                })
        {
            let c = anvil::load::load_chunk(&raw_chunk_data, chunk)?;
            chunks[index] = Some(c);
        }

        Ok(AnvilSave::new(header, chunks))
    }

    fn load_from_bytes(&self, bytes: &[u8]) -> Result<AnvilSave> {
        todo!()
    }
}

pub trait LoadMcSave<S> {
    fn load(&self, path: &str) -> Result<S>;
    fn load_from_bytes(&self, bytes: &[u8]) -> Result<S>;
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ZlibDecode(#[from] std::io::Error),
    #[error(transparent)]
    NBT(#[from] crate::nbt::Error),
    #[error(transparent)]
    ChunkStatus(#[from] crate::data::ChunkStatusError),
    #[error(transparent)]
    MissingData(#[from] crate::data::MissingData),
}
