use getset::Getters;

use crate::nbt_data::chunk::ChunkData;

pub use super::mc_region::header;

#[derive(Debug, Getters)]
#[get = "pub"]
pub struct AnvilSave {
    header: header::McRegionHeader,
    chunks: Vec<ChunkData>,
}

impl AnvilSave {
    pub fn new(header: header::McRegionHeader, chunks: Vec<ChunkData>) -> Self {
        Self { header, chunks }
    }
}
