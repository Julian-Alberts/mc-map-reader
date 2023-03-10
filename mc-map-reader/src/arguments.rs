use std::path::PathBuf;

use clap::{command, Parser, Subcommand};

use crate::search_dupe_stashes::args::SearchDupeStashes;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub action: Option<Action>,
    //pub config_file: Option<PathBuf>,
    /// Path to the Minecraft Save
    pub save_directory: PathBuf,
}

#[derive(Debug, Subcommand)]
pub enum Action {
    SearchDupeStashes(SearchDupeStashes),
    FindInventories(crate::find_inventories::config::SearchEntity),
}
