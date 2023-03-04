use std::{fs::File, path::PathBuf};

use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Config {
    search_pube_stashes: Option<SearchDupeStashesConfig>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct SearchDupeStashesConfig {
    pub items: Vec<Item>,
    pub scan_containers: Vec<ScanContainers>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Item {
    id: String,
    warning_threshold: Option<usize>,
    alarm_threshold: Option<usize>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct ScanContainers {
    id: String,
    tag: Option<Vec<String>>,
}

impl TryFrom<PathBuf> for Config {
    type Error = ConfigLoadError;
    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        let file = File::open(value)?;
        let config = serde_json::from_reader(file)?;
        Ok(config)
    }
}

impl Default for Config {
    fn default() -> Self {
        serde_json::from_str(include_str!("../default_config.json"))
            .expect("Invalid default config")
    }
}

#[derive(Debug, Error)]
pub enum ConfigLoadError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    JSON(#[from] serde_json::Error),
}
