use std::{fmt::Display, path::PathBuf};

use clap::{command, Parser, Subcommand, ValueEnum};

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
    #[arg(short, long, default_value = "off")]
    pub log_level: LogLevel,
}

#[derive(Debug, Subcommand)]
pub enum Action {
    /// Search for large amounts of items in a small area
    SearchDupeStashes(SearchDupeStashes),
    /// Find inventories of a specific type
    FindInventories(crate::find_inventories::config::SearchEntity),
    #[cfg(feature = "experimental")]
    ReadLevelDat,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum LogLevel {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl From<LogLevel> for log::LevelFilter {
    fn from(value: LogLevel) -> Self {
        use log::LevelFilter::*;
        match value {
            LogLevel::Off => Off,
            LogLevel::Error => Error,
            LogLevel::Warn => Warn,
            LogLevel::Info => Info,
            LogLevel::Debug => Debug,
            LogLevel::Trace => Trace,
        }
    }
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        log::LevelFilter::fmt(&(*self).into(), f)
    }
}

#[cfg(test)]
mod tests {

    use log::LevelFilter;
    use test_case::test_case;
    use super::*;

    #[test_case(LogLevel::Off => LevelFilter::Off; "Off")]
    #[test_case(LogLevel::Error => LevelFilter::Error; "Error")]
    #[test_case(LogLevel::Warn => LevelFilter::Warn; "Warn")]
    #[test_case(LogLevel::Info => LevelFilter::Info; "Info")]
    #[test_case(LogLevel::Debug => LevelFilter::Debug; "Debug")]
    #[test_case(LogLevel::Trace => LevelFilter::Trace; "Trace")]
    fn test_level_filer_from_log_level(level: LogLevel) -> LevelFilter {
        level.into()
    }

    #[test_case(LogLevel::Off => "OFF"; "Off")]
    #[test_case(LogLevel::Error => "ERROR"; "Error")]
    #[test_case(LogLevel::Warn => "WARN"; "Warn")]
    #[test_case(LogLevel::Info => "INFO"; "Info")]
    #[test_case(LogLevel::Debug => "DEBUG"; "Debug")]
    #[test_case(LogLevel::Trace => "TRACE"; "Trace")]
    fn test_level_filer_from_log_level_display(level: LogLevel) -> String {
        level.to_string()
    }

}
