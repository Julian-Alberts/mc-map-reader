use clap::{Args, ValueEnum};

#[derive(Clone, PartialEq, Eq, PartialOrd, Args, Debug)]
pub struct SearchEntity {
    #[arg(short, long="entity-id")]
    pub entity_ids: Option<Vec<String>>,
    #[arg(short, long, value_enum, default_value_t = Dimension::Overworld)]
    pub dimension: Dimension,
    #[arg(short, long, default_value_t = false)]
    pub block_entity: bool
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Dimension {
    Overworld,
    Nether,
    End
}