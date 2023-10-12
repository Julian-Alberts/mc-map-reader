//! File format module containing the data structures for the supported file formats.

#[cfg(feature = "region_file")]
pub mod anvil;
#[cfg(feature = "level_dat")]
pub mod level_dat;
pub mod player_dat;
