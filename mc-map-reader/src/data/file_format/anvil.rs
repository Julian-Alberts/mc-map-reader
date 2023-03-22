//! Anvil save file.

use getset::Getters;

use crate::data::chunk::ChunkData;

/// Anvil save file.
/// [Minecraft Wiki](https://minecraft.fandom.com/wiki/Anvil_file_format)
#[derive(Debug, Getters)]
#[get = "pub"]
pub struct AnvilSave {
    /// The header of the save file.
    header: McRegionHeader,
    /// The chunks in the save file.
    chunks: Vec<ChunkData>,
}

impl AnvilSave {
    /// Create a new Anvil save file.
    pub fn new(header: McRegionHeader, chunks: Vec<ChunkData>) -> Self {
        Self { header, chunks }
    }
}

const CHUNKS_PER_FILE: usize = 1024;
const CHUNK_OFFSET_LENGTH: usize = 4;
const CHUNK_OFFSETS_START: usize = 0;
const CHUNK_OFFSETS_SIZE: usize = CHUNK_OFFSET_LENGTH * CHUNKS_PER_FILE;
/// The size of the header in bytes.
pub const MC_REGION_HEADER_SIZE: usize = 8192;
/// The message that is displayed when the header is invalid.
pub const INVALID_HEADER_MESSAGE: &str = "Invalid Header";

/// Header of a region or anvil file.
/// [Minecraft Wiki](https://minecraft.fandom.com/wiki/Region_file_format#Header)
#[derive(Debug)]
pub struct McRegionHeader {
    chunks: [Option<ChunkInfo>; CHUNKS_PER_FILE],
}

/// Information about a chunk in a region file.
#[derive(Debug)]
pub struct ChunkInfo {
    /// Length of the chunk in sectors of 4096 bytes.
    pub sector_count: u8,
    /// The offset of the chunk in the file. In sectors of 4096 bytes. Including the header.
    pub offset: u32,
    /// The timestamp when the chunk was last saved.
    pub timestamp: u32,
}

impl McRegionHeader {
    /// Get the chunk information.
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
    /// Get the offset
    pub fn get_offset(&self) -> u32 {
        self.offset
    }

    /// Get the timestamp
    pub fn get_timestamp(&self) -> u32 {
        self.timestamp
    }
}
