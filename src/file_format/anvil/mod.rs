pub mod load;
use crate::data::ChunkData;

pub use super::mc_region::header;

/// 1KiB
const KIB: u32 = 1024;
const CHUNK_ALLIGNMENT: u32 = KIB * 4;

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
