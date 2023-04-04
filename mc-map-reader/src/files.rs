use std::path::{Path, PathBuf};

/// Return a list of all region files in the given area.
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
        chunk1_x..=chunk2_x
    } else {
        chunk2_x..=chunk1_x
    };
    let z_axis_values = if chunk1_z < chunk2_z {
        chunk1_z..=chunk2_z
    } else {
        chunk2_z..=chunk1_z
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

/// Return a list of all region files.
pub fn get_region_files(
    world_dir: &Path,
    dimension_directory: Option<&Path>,
) -> std::io::Result<Vec<PathBuf>> {
    let mut region_dir = PathBuf::from(world_dir);
    if let Some(dimension) = dimension_directory {
        region_dir.push(dimension)
    }
    region_dir.push("region");
    std::fs::read_dir(region_dir)?
        .map(|entry| entry.map(|e| e.path()))
        .collect::<Result<_, _>>()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use test_case::test_case;

    fn get_test_world_dir() -> PathBuf {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources");
        path.push("tests");
        path
    }

    #[test_case(true; "Test dimension")]
    #[test_case(false; "No dimension")]
    fn get_all_region_files(test_dim: bool) {
        let (world_dir, dimension) = if test_dim {
            let mut dir = get_test_world_dir();
            dir.pop();
            (dir, Some(PathBuf::from("tests")))
        } else {
            (get_test_world_dir(), None)
        };
        let actual = super::get_region_files(&world_dir, dimension.as_ref().map(AsRef::as_ref))
            .unwrap()
            .into_iter()
            .map(|path| path.file_name().unwrap().to_str().unwrap().to_owned())
            .collect::<Vec<_>>();
        let mut expected = Vec::new();
        for x in -2..=2 {
            for z in -2..=2 {
                expected.push(format!("r.{x}.{z}.mca"))
            }
        }
        dbg!(&actual);
        dbg!(&expected);
        assert!(expected.iter().all(|file_name| actual.contains(file_name)));
        assert!(actual.iter().all(|file_name| expected.contains(file_name)));
    }

    #[test_case(10, 10, 42, 42, &[(0, 0), (1,0), (0,1), (1,1)], false; "Four region files")]
    #[test_case(42, 42, 10, 10, &[(0, 0), (1,0), (0,1), (1,1)], false; "Four region files inputs reversed")]
    #[test_case(10, 10, 42, 42, &[(0, 0), (1,0), (0,1), (1,1)], true; "Four region files test dimension")]
    #[test_case(64, 64, 96, 96, &[(2, 2)], false; "Region files out ouf range")]
    #[test_case(-10, -10, 10, 10, &[(0, 0), (-1,0), (0,-1), (-1,-1)], false; "Negative coordinates")]
    fn get_files_in_area(
        x1: i64,
        z1: i64,
        x2: i64,
        z2: i64,
        expected: &'static [(i64, i64)],
        test_dim: bool,
    ) {
        let (world_dir, dimension) = if test_dim {
            let mut dir = get_test_world_dir();
            dir.pop();
            (dir, Some(PathBuf::from("tests")))
        } else {
            (get_test_world_dir(), None)
        };
        let actual = super::get_region_files_in_area(
            &world_dir,
            dimension.as_ref().map(AsRef::as_ref),
            x1,
            z1,
            x2,
            z2,
        );
        let expected = expected
            .iter()
            .map(|(x, z)| {
                let mut path = get_test_world_dir();
                path.push("region");
                path.push(format!("r.{x}.{z}.mca"));
                path
            })
            .collect::<Vec<_>>();
        assert_eq!(expected.len(), actual.len());
        assert!(expected.iter().all(|file_name| actual.contains(file_name)));
        assert!(actual.iter().all(|file_name| expected.contains(file_name)));
    }
}
