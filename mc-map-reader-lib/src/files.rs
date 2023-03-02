use std::path::{Path, PathBuf};

pub fn get_region_files_in_area(
    world_directory: &Path,
    dimension_directory: Option<&Path>,
    chunk1_x: i64,
    chunk1_z: i64,
    chunk2_x: i64,
    chunk2_z: i64,
) -> Vec<PathBuf> {
    let chunk1_x = chunk1_x >> 5;
    let chunk1_z = chunk1_z >> 5;
    let chunk2_x = chunk2_x >> 5;
    let chunk2_z = chunk2_z >> 5;

    let x_axis_values = if chunk1_x < chunk2_x {
        chunk1_x..chunk2_x
    } else {
        chunk2_x..chunk1_x
    };
    let z_axis_values = if chunk1_z < chunk2_z {
        chunk1_z..chunk2_z
    } else {
        chunk2_z..chunk1_z
    };
    x_axis_values
        .into_iter()
        .fold(Vec::new(), |mut vec, x| {
            vec.extend(z_axis_values.clone().map(|z| (x, z)));
            vec
        })
        .into_iter()
        .map(|(x, z)| {
            let mut region_file = PathBuf::from(world_directory);
            if let Some(dimension) = dimension_directory {
                region_file.push(dimension)
            }
            region_file.push(format!("region/r.{x}.{z}.mca"));
            region_file
        })
        .filter(|region_file| region_file.exists())
        .collect()
}

pub fn get_region_files(world_dir: &Path,dimension_directory: Option<&Path>) -> std::io::Result<Vec<PathBuf>> {
    let mut region_dir = PathBuf::from(world_dir);
    if let Some(dimension) = dimension_directory {
        region_dir.push(dimension)
    }
    region_dir.push("region");
    std::fs::read_dir(region_dir)?
        .map(|entry| entry.map(|e| e.path()))
        .collect::<Result<_, _>>()
}
