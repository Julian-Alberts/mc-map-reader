use std::collections::HashMap;

use jbe::Builder;

use crate::nbt::Tag;

// https://minecraft.fandom.com/wiki/Custom_dimension
#[derive(Debug, Builder, PartialEq)]
pub struct Dimension {
    pub dimension_type: String,
    pub generator: HashMap<String, Tag>,
}
