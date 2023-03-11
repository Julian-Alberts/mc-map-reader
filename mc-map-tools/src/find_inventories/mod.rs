use std::{
    fs::File,
    ops::Deref,
    path::{Path, PathBuf},
};

use mc_map_reader::{nbt_data::chunk::ChunkData, LoadMcSave};
use wildmatch::WildMatch;

use self::config::SearchEntity;

pub mod config;

pub fn main(world_dir: &Path, args: &SearchEntity) {
    let wildcards = args.entity_ids.as_ref();
    let wildcards = compile_wildcards(wildcards.unwrap_or(&vec![String::from("*")]).as_slice());
    let dim: Option<PathBuf> = args.dimension.into();
    let dim = dim.as_deref();
    let regions = mc_map_reader::files::get_region_files(world_dir, dim)
        .expect("Could not read region directory");

    let search_fn = if args.block_entity {
        &search_block_entity
    } else {
        &search_block_entity
    };

    regions.into_iter().for_each(|r| {
        let file = File::open(r).expect("Could not open file");
        let region = mc_map_reader::Loader
            .load_from_bytes(file)
            .expect("Error reading file");
        region
            .chunks()
            .iter()
            .for_each(|chunk| search_fn(chunk, &wildcards))
    })
}

fn search_block_entity(chunk_data: &ChunkData, wildcards: &[WildMatch]) {
    let Some(block_entities) = chunk_data.block_entities() else {
        return
    };

    block_entities
        .iter()
        .filter(|be| wildcards.iter().any(|w| w.matches(be.id())))
        .for_each(|be| {
            println!(
                "Found {} at x:{} y:{} z:{}",
                be.id(),
                be.x(),
                be.y(),
                be.z()
            )
        })
}

fn compile_wildcards(wildcards: &[String]) -> Vec<WildMatch> {
    wildcards
        .iter()
        .map(Deref::deref)
        .map(WildMatch::new)
        .collect()
}
