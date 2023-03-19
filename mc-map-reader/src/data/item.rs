use std::collections::HashMap;

use getset::*;
use jbe::Builder;

use crate::nbt::Tag;

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct Item {
    #[get = "pub"]
    id: String,
    tag: Option<HashMap<String, Tag>>,
    #[get_copy = "pub"]
    count: i8,
}

impl Item {
    pub fn tag(&self) -> Option<&HashMap<String, Tag>> {
        self.tag.as_ref()
    }
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct ItemWithSlot {
    #[get_copy = "pub"]
    slot: i8,
    #[get = "pub"]
    item: Item,
}