//! Anvil save file.

use crate::data::chunk::ChunkData;

/// Anvil save file.
/// [Minecraft Wiki](https://minecraft.fandom.com/wiki/Anvil_file_format)
#[derive(Debug, PartialEq)]
pub struct AnvilSave {
    /// The header of the save file.
    pub header: McRegionHeader,
    /// The chunks in the save file.
    pub chunks: Vec<ChunkData>,
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
pub const INVALID_HEADER_MESSAGE: &str = "Invalid Header size";

/// Header of a region or anvil file.
/// [Minecraft Wiki](https://minecraft.fandom.com/wiki/Region_file_format#Header)
#[derive(Debug, PartialEq)]
pub struct McRegionHeader {
    chunks: [Option<ChunkInfo>; CHUNKS_PER_FILE],
}

/// Information about a chunk in a region file.
#[derive(Debug, Clone, PartialEq)]
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
            // This should never happen because the length of the array is always MC_REGION_HEADER_SIZE
            // If this is not the case this function can not be called.
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
            // This should never happen because the length of the array is always MC_REGION_HEADER_SIZE
            // If this is not the case this function can not be called.
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_create_anvil_save() {
        let header = McRegionHeader {
            chunks: (0..CHUNKS_PER_FILE)
                .map(|_| None)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        };
        let chunks = vec![];
        let anvil_save = AnvilSave::new(header, chunks);
        assert_eq!(
            anvil_save,
            AnvilSave {
                header: McRegionHeader {
                    chunks: (0..CHUNKS_PER_FILE)
                        .map(|_| None)
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap()
                },
                chunks: vec![]
            }
        );
    }

    #[test]
    fn test_get_chunk_info() {
        let header = McRegionHeader {
            chunks: (0..CHUNKS_PER_FILE)
                .map(|_| None)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        };
        let expect: [_; CHUNKS_PER_FILE] = (0..CHUNKS_PER_FILE)
            .map(|_| None)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        assert_eq!(header.get_chunk_info(), &expect);
    }

    #[test]
    fn test_chunk_info_get() {
        let chunk_info = ChunkInfo {
            sector_count: 23,
            offset: 32,
            timestamp: 42,
        };
        assert_eq!(chunk_info.get_offset(), 32);
        assert_eq!(chunk_info.get_timestamp(), 42)
    }

    #[test]
    fn test_mc_region_header_from_all_null() {
        let raw: [u8; MC_REGION_HEADER_SIZE] = [0; MC_REGION_HEADER_SIZE];
        assert_eq!(
            McRegionHeader::from(raw),
            McRegionHeader {
                chunks: (0..CHUNKS_PER_FILE)
                    .map(|_| None)
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            }
        );
    }

    #[test]
    fn test_mc_region_header_from() {
        let raw = (0..CHUNKS_PER_FILE as u32).fold(
            Vec::with_capacity(MC_REGION_HEADER_SIZE),
            |mut vec, index| {
                let bytes = index.to_be_bytes();
                vec.push(bytes[1]);
                vec.push(bytes[2]);
                vec.push(bytes[3]);
                match index {
                    0..=255 => vec.push(0b0001),
                    256..=511 => vec.push(0b0010),
                    512..=767 => vec.push(0b0100),
                    768..=1023 => vec.push(0b1000),
                    _ => panic!("Invalid index"),
                }
                vec
            },
        );
        let raw = (0..CHUNKS_PER_FILE as u32).fold(raw, |mut vec, time| {
            vec.extend(time.to_be_bytes().iter());
            vec
        });
        assert_eq!(raw.len(), MC_REGION_HEADER_SIZE);
        let raw: [u8; MC_REGION_HEADER_SIZE] = raw.try_into().unwrap();
        let actual = McRegionHeader::from(raw);
        for index in 0..CHUNKS_PER_FILE {
            assert_eq!(
                actual.chunks[index],
                Some(ChunkInfo {
                    offset: index as u32,
                    sector_count: match index {
                        0..=255 => 0b0001,
                        256..=511 => 0b0010,
                        512..=767 => 0b0100,
                        768..=1023 => 0b1000,
                        _ => panic!("Invalid index"),
                    },
                    timestamp: index as u32
                }),
                "Invalid chunk info at index {index}"
            );
        }
    }
}
