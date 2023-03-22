#![warn(missing_docs)]
#![deny(clippy::unwrap_used)]
#![deny(unused_unsafe)]
#![deny(clippy::undocumented_unsafe_blocks)]

//! # mc-map-tools
//! A collection of tools for working with Minecraft maps.
//! ## Features
//! ### SearchDupeStashes
//! Search for stashes of duplicate items.
//! ### FindInventories (experimental)
//! Find inventories of a specific type.
//! ### ReadLevelDat (experimental)
//! Read the level.dat file. This feature is currently pretty useless.

mod arguments;
mod config;
mod find_inventories;
mod paths;
mod quadtree;
#[cfg(feature = "experimental")]
mod read_level_dat;
mod search_dupe_stashes;

use std::{fs::File, io::Read, path::PathBuf};

use arguments::Action;
use clap::Parser;
use config::Config;

use crate::arguments::Args;

fn main() {
    let args = Args::parse();
    setup_logger(args.log_level.into());
    let config = if let Some(config_file) = args.config_file {
        log::info!("Reading config file :\"{config_file:#?}\"");
        Config::try_from(config_file).expect("Failed to load config")
    } else {
        let path: PathBuf = paths::Files::ConfigFile.into();
        if path.exists() {
            log::info!("Reading config file :\"{path:#?}\"");
            Config::try_from(path).expect("Invalid config file")
        } else {
            log::info!("Using default config");
            Config::default()
        }
    };
    log::debug!("Config: {config:?}");

    match args.action {
        Action::SearchDupeStashes(data) => {
            log::debug!("Running SearchDupeStashes with arguments: {data:?}");
            search_dupe_stashes::main(args.save_directory.as_path(), data, config)
        }
        Action::FindInventories(sub_args) => {
            find_inventories::main(args.save_directory.as_path(), &sub_args)
        }
        #[cfg(feature = "experimental")]
        Action::ReadLevelDat => read_level_dat::main(args.save_directory.as_path()),
    }
}

fn read_file(mut region_file: File) -> std::io::Result<Vec<u8>> {
    let mut buf = Vec::default();
    region_file.read_to_end(&mut buf)?;
    Ok(buf)
}

fn setup_logger(level: log::LevelFilter) {
    use simplelog::*;
    let mut logger : Vec<Box<dyn SharedLogger>> = Vec::new();
    logger.push(TermLogger::new(level, Config::default(), TerminalMode::Stderr, ColorChoice::Auto));
    
    let log_file = std::fs::File::create(paths::Files::LogFile.path());

    match log_file {
        Ok(file) => {
            logger.push(WriteLogger::new(level, Config::default(), file));
            CombinedLogger::init(logger).unwrap();
        }
        Err(e) => {
            CombinedLogger::init(logger).unwrap();
            log::info!("Error while opening log file: {e}");
        }
    }

    CombinedLogger::init(vec![
        TermLogger::new(level, Config::default(), simplelog::TerminalMode::Stderr, ColorChoice::Auto),
        WriteLogger::new(level, Config::default(), std::fs::File::create(paths::Files::LogFile.path()).unwrap())
    ]).unwrap();
}
