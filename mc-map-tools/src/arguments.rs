use std::path::PathBuf;

use clap::{command, Parser, Subcommand};

use crate::search_dupe_stashes::args::SearchDupeStashes;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub action: Action,
    /// Path to the Minecraft Save
    pub save_directory: PathBuf,
    /// Override the default config file
    #[arg(short, long)]
    pub config_file: Option<PathBuf>,
}

#[derive(Debug, Subcommand)]
pub enum Action {
    /// Search for large amounts of items in a small area
    SearchDupeStashes(SearchDupeStashes),
    /// Find inventories of a specific type
    FindInventories(crate::find_inventories::config::SearchEntity),
    ReadLevelDat,
}
