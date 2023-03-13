mod arguments;
mod config;
mod find_inventories;
mod paths;
mod quadtree;
mod search_dupe_stashes;
mod read_level_dat;

use std::{fs::File, io::Read, path::PathBuf};

use arguments::Action;
use clap::Parser;
use config::Config;

use crate::arguments::Args;

fn main() {
    let args = Args::parse();
    let config = if let Some(config_file) = args.config_file {
        Config::try_from(config_file).expect("Failed to load config")
    } else {
        let path: PathBuf = paths::Files::ConfigFile.into();
        if path.exists() {
            Config::try_from(path).expect("Invalid config file")
        } else {
            Config::default()
        }
    };

    match args.action {
        Action::SearchDupeStashes(data) => {
            search_dupe_stashes::main(args.save_directory.as_path(), data, config)
        }
        Action::FindInventories(sub_args) => {
            find_inventories::main(args.save_directory.as_path(), &sub_args)
        },
        Action::ReadLevelDat => read_level_dat::main(args.save_directory.as_path())
    }
}

fn read_file(mut region_file: File) -> std::io::Result<Vec<u8>> {
    let mut buf = Vec::default();
    region_file.read_to_end(&mut buf)?;
    Ok(buf)
}
