//! Data structures for Minecraft NBT data.

#[cfg(feature = "block_entity")]
pub mod block_entity;
#[cfg(feature = "region_file")]
pub mod chunk;
pub mod dimension;
pub mod entity;
pub mod file_format;
pub mod item;
mod load;
pub use load::FieldError;
