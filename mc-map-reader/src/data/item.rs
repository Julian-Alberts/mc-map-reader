use std::collections::HashMap;

use jbe::Builder;

use crate::nbt::Tag;

/// Representation of an item.
/// [Minecraft Wiki](https://minecraft.fandom.com/wiki/Player.dat_format#Item_structure)
#[derive(Debug, Builder, Clone, PartialEq)]
pub struct Item {
    /// Internal item ID
    pub id: String,
    pub tag: Option<HashMap<String, Tag>>,
    /// Stack size
    pub count: i8,
}

/// Representation of an item inside a slot. This type is used if something takes more than one item.
#[derive(Debug, Builder, Clone, PartialEq)]
pub struct ItemWithSlot {
    /// Slot ID
    pub slot: i8,
    /// Item
    pub item: Item,
}
