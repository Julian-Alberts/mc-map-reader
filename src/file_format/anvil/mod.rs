use crate::nbt_data::chunk::ChunkData;

pub use super::mc_region::header;

#[derive(Debug)]
pub struct AnvilSave {
    header: header::McRegionHeader,
    chunks: [Option<ChunkData>; 32 * 32],
}

impl AnvilSave {
    pub fn new(header: header::McRegionHeader, chunks: [Option<ChunkData>; 32 * 32]) -> Self {
        Self { header, chunks }
    }
}
