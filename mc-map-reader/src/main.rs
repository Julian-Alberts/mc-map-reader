mod arguments;
mod config;
mod find_inventories;
mod quadtree;
mod search_dupe_stashes;

use std::{
    fs::File,
    io::Read,
};

use arguments::Action;
use clap::Parser;
use config::Config;

use crate::arguments::Args;

fn main() {
    let args = Args::parse();
    /*let config = if let Some(config) = args.config_file {
        match Config::try_from(config) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("{e}");
                return;
            }
        }
    } else {
        Config::default()
    };*/
    match args.action {
        Some(Action::SearchDupeStashes(data)) => {
            search_dupe_stashes::main(args.save_directory.as_path(), data, Config::default())
        }
        Some(Action::FindInventories(sub_args)) => {
            find_inventories::main(args.save_directory.as_path(), &sub_args)
        }
        None => {
            println!("done")
        }
    }
}

fn read_file(mut region_file: File) -> std::io::Result<Vec<u8>> {
    let mut buf = Vec::default();
    region_file.read_to_end(&mut buf)?;
    Ok(buf)
}
