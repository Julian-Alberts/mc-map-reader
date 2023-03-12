use std::{fs::File, path::PathBuf};

use serde::Deserialize;
use thiserror::Error;

use crate::search_dupe_stashes::config::SearchDupeStashesConfig;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Config {
    pub search_dupe_stashes: SearchDupeStashesConfig,
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
        Self {
            search_dupe_stashes: SearchDupeStashesConfig::default(),
        }
    }
}

#[derive(Debug, Error)]
pub enum ConfigLoadError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    JSON(#[from] serde_json::Error),
}
