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

#[cfg(test)]
mod tests {

    #[test]
    fn item_get_tag_none() {
        let mut item = super::ItemBuilder::default();
        item.with_id("minecraft:stone".to_string())
            .set_count(1);
        let item = item.build();
        assert_eq!(item.tag(), None);
    }

    #[test]
    fn item_get_tag_some() {
        let mut item = super::ItemBuilder::default();
        let mut tag = std::collections::HashMap::new();
        tag.insert("test".to_string(), crate::nbt::Tag::Int(1));
        item.with_id("minecraft:stone".to_string())
            .with_tag(tag.clone())
            .set_count(1);
        let item = item.build();
        assert_eq!(item.tag(), Some(&tag));
    }

}
