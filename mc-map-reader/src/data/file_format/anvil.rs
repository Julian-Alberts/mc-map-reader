use getset::Getters;

use crate::data::chunk::ChunkData;

#[derive(Debug, Getters)]
#[get = "pub"]
pub struct AnvilSave {
    header: McRegionHeader,
    chunks: Vec<ChunkData>,
}

impl AnvilSave {
    pub fn new(header: McRegionHeader, chunks: Vec<ChunkData>) -> Self {
        Self { header, chunks }
    }
}

const CHUNKS_PER_FILE: usize = 1024;
const CHUNK_OFFSET_LENGTH: usize = 4;
const CHUNK_OFFSETS_START: usize = 0;
const CHUNK_OFFSETS_SIZE: usize = CHUNK_OFFSET_LENGTH * CHUNKS_PER_FILE;
pub const MC_REGION_HEADER_SIZE: usize = 8192;
pub const INVALID_HEADER_MESSAGE: &str = "Invalid Header";

#[derive(Debug)]
pub struct McRegionHeader {
    chunks: [Option<ChunkInfo>; CHUNKS_PER_FILE],
}

#[derive(Debug)]
pub struct ChunkInfo {
    pub sector_count: u8,
    pub offset: u32,
    pub timestamp: u32,
}

impl McRegionHeader {
    pub fn get_chunk_info(&self) -> &[Option<ChunkInfo>; CHUNKS_PER_FILE] {
        &self.chunks
    }
}

impl From<[u8; MC_REGION_HEADER_SIZE]> for McRegionHeader {
    fn from(raw: [u8; MC_REGION_HEADER_SIZE]) -> Self {
        let chunk_offsets: [Option<_>; CHUNKS_PER_FILE] = raw
            [CHUNK_OFFSETS_START..CHUNK_OFFSETS_SIZE]
            .chunks(CHUNK_OFFSET_LENGTH)
            .map(|data| (u32::from_be_bytes([0, data[0], data[1], data[2]]), data[3]))
            .map(|(offset, sector_count)| {
                if offset == 0 && sector_count == 0 {
                    None
                } else {
                    Some((offset, sector_count))
                }
            })
            .collect::<Vec<_>>()
            .try_into()
            .expect(INVALID_HEADER_MESSAGE);
        let chunks: [Option<ChunkInfo>; CHUNKS_PER_FILE] = raw
            [CHUNK_OFFSETS_SIZE..MC_REGION_HEADER_SIZE]
            .chunks(CHUNK_OFFSET_LENGTH)
            .map(|timestamp| {
                u32::from_be_bytes([timestamp[0], timestamp[1], timestamp[2], timestamp[3]])
            })
            .zip(chunk_offsets.iter())
            .map(|(timestamp, offset_data)| {
                offset_data.map(|offset| ChunkInfo {
                    sector_count: offset.1,
                    offset: offset.0,
                    timestamp,
                })
            })
            .collect::<Vec<_>>()
            .try_into()
            .expect(INVALID_HEADER_MESSAGE);
        Self { chunks }
    }
}

impl ChunkInfo {
    pub fn get_offset(&self) -> u32 {
        self.offset
    }

    pub fn get_timestamp(&self) -> u32 {
        self.timestamp
    }
}

