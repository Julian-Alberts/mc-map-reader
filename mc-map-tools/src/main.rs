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
#[cfg(feature = "experimental")]
mod read_level_dat;
mod search_dupe_stashes;
mod file;

use std::{fs::File, io::Read, path::PathBuf};
use async_std::io::ReadExt;

use arguments::Action;
use clap::Parser;
use config::Config;

use crate::arguments::Args;

#[async_std::main]
async fn main() {
    let args = Args::parse();
    setup_logger(args.log_level.into());
    let config = if let Some(config_file) = args.config_file.map(File::open) {
        log::info!("Reading config file :\"{config_file:#?}\"");
        let config_file = config_file.expect("Failed to open config file");
        Config::new(config_file).expect("Failed to load config")
    } else {
        let path: PathBuf = paths::Files::ConfigFile.into();
        if path.exists() {
            log::info!("Reading config file :\"{path:#?}\"");
            Config::new(File::open(path).expect("Failed to open config file"))
                .expect("Invalid config file")
        } else {
            log::info!("Using default config");
            Config::default()
        }
    };
    log::debug!("Config: {config:?}");

    match args.action {
        Action::SearchDupeStashes(data) => {
            log::debug!("Running SearchDupeStashes with arguments: {data:?}");
            search_dupe_stashes::main(
                args.save_directory.as_path(),
                data,
                config,
                &mut std::io::stdout().lock(),
            ).await
        }
        Action::FindInventories(sub_args) => {
            find_inventories::main(args.save_directory.as_path(), &sub_args)
        }
        #[cfg(feature = "experimental")]
        Action::ReadLevelDat => read_level_dat::main(args.save_directory.as_path()),
    }
}

async fn read_file(mut region_file: async_std::fs::File) -> std::io::Result<Vec<u8>> {
    let mut buf = Vec::default();
    region_file.read_to_end(&mut buf).await?;
    Ok(buf)
}

fn setup_logger(level: log::LevelFilter) {
    use simplelog::*;
    let mut logger: Vec<Box<dyn SharedLogger>> = Vec::new();
    logger.push(TermLogger::new(
        level,
        Config::default(),
        TerminalMode::Stderr,
        ColorChoice::Auto,
    ));

    let log_file = std::fs::File::create(paths::Files::LogFile.path());

    let error = match log_file {
        Ok(file) => {
            logger.push(WriteLogger::new(level, Config::default(), file));
            None
        }
        Err(e) => Some(e),
    };

    CombinedLogger::init(logger).expect("Error while initializing logger");
    if let Some(e) = error {
        log::info!("Error while opening log file: {e}");
    }
}
