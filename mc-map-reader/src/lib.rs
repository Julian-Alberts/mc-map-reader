#![deny(clippy::unwrap_used)]
#![deny(unused_unsafe)]
#![deny(clippy::undocumented_unsafe_blocks)]

//! This crate provides a way to read Minecraft saves.

pub mod data;
mod load;
pub use load::*;
mod compression;
pub mod files;
pub mod nbt;
#[cfg(test)]
pub mod test_util;
