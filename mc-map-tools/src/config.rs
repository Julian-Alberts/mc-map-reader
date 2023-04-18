use std::io::Read;

use serde::Deserialize;
use thiserror::Error;

use crate::search_dupe_stashes::config::SearchDupeStashesConfig;

#[derive(Debug, PartialEq, Deserialize, Default)]
pub struct Config {
    pub search_dupe_stashes: SearchDupeStashesConfig,
}

impl Config {
    pub fn new<R>(reader: R) -> Result<Self, ConfigLoadError>
    where
        R: Read,
    {
        let config = serde_json::from_reader(reader)?;
        Ok(config)
    }
}

#[derive(Debug, Error)]
pub enum ConfigLoadError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_config() {
        let config = Config::new(r#"{"search_dupe_stashes": {"groups": {}}}"#.as_bytes()).unwrap();
        assert_eq!(
            config,
            Config {
                search_dupe_stashes: SearchDupeStashesConfig {
                    groups: HashMap::new(),
                }
            }
        );
    }
}
