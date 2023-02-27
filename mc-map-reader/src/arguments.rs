use std::path::PathBuf;

use clap::{Parser, command};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub filter: Option<FilterSubcommand>,
    /// Path to the Minecraft Save
    pub save_directory: PathBuf,
}

#[derive(Debug, Subcommand)]
pub enum FilterSubcommand {
    
}

#[derive(Debug, Clone)]
pub enum Filter {
    BlockEntity,
    BlockEntityByName(String),
    Inventory,
}


