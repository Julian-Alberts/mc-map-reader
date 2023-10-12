use std::path::PathBuf;

use clap::{Args, ValueEnum};

#[derive(Clone, PartialEq, Eq, PartialOrd, Args, Debug)]
pub struct SearchEntity {
    #[arg(short, long = "entity-id")]
    pub entity_ids: Option<Vec<String>>,
    #[arg(short, long, value_enum, default_value_t = Dimension::Overworld)]
    pub dimension: Dimension,
    #[arg(short, long, default_value_t = false)]
    pub block_entity: bool,
}

impl From<Dimension> for Option<PathBuf> {
    fn from(value: Dimension) -> Self {
        match value {
            Dimension::End => Some(PathBuf::from("DIM1")),
            Dimension::Nether => Some(PathBuf::from("DIM")),
            Dimension::Overworld => None,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Dimension {
    Overworld,
    Nether,
    End,
}
