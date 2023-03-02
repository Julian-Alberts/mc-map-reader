mod arguments;
mod config;
mod quadtree;
mod search_dupe_stashes;

use std::{
    fs::{File, OpenOptions},
    io::Read,
    path::{Path, PathBuf},
};

use arguments::{Action, Area};
use clap::Parser;
use config::Config;
use mc_map_reader_lib::{file_format::anvil::AnvilSave, LoadMcSave};

use crate::arguments::Args;

fn main() {
    let args = Args::parse();
    let config = if let Some(config) = args.config_file {
        match Config::try_from(config) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("{e}");
                return;
            }
        }
    } else {
        Config::default()
    };
    match args.action {
        Some(Action::SearchDupeStashes(data)) => {
            search_dupe_stashes::main(args.save_directory.as_path(), data, config)
        }
        None => {
            println!("done")
        }
    }
}

fn load_regions(world_dir: &Path) -> Vec<AnvilSave> {
    let files = mc_map_reader_lib::files::get_region_files(world_dir)
        .expect("Could not read region directory");
    load_selected_regions(files)
}

fn load_regions_in_area(world_dir: &Path, area: &Area) -> Vec<AnvilSave> {
    let files = mc_map_reader_lib::files::get_region_files_in_area(
        world_dir, area.x1, area.z1, area.x2, area.z2,
    );
    load_selected_regions(files)
}

fn load_selected_regions(files: Vec<PathBuf>) -> Vec<AnvilSave> {
    let mut futures = Vec::with_capacity(files.len());
    for region in files {
        let future = {
            let data = match OpenOptions::new().read(true).open(region) {
                Ok(file) => read_file(file),
                Err(e) => panic!("{e}"),
            };
            let data = match data {
                Ok(data) => data,
                Err(e) => panic!("{e}"),
            };
            match mc_map_reader_lib::Loader.load_from_bytes(&data[..]) {
                Ok(data) => data,
                Err(e) => panic!("{e}"),
            }
        };
        futures.push(future)
    }
    futures
}

fn read_file(mut region_file: File) -> std::io::Result<Vec<u8>> {
    let mut buf = Vec::default();
    region_file.read_to_end(&mut buf)?;
    Ok(buf)
}
