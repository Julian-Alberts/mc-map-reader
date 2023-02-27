mod arguments;

use clap::Parser;
use mc_map_reader::LoadMcSave;

use crate::arguments::Args;

#[async_std::main]
async fn main() {
    let args = Args::parse();
    println!("{:#?}", args);
    let map = mc_map_reader::Loader
        .load("../server_world/region/r.1.1.mca")
        .unwrap();
    println!("{map:#?}")
}
