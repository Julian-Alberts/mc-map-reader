use std::collections::HashMap;

use serde::Deserialize;

type Nbt = serde_json::value::Map<String, serde_json::Value>;

#[derive(Debug, PartialEq, Deserialize)]
pub struct SearchDupeStashesConfig {
    pub groups: HashMap<String, Group>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Group {
    pub items: Vec<GroupEntry>,
    pub threshold: usize,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct GroupEntry {
    pub id: Option<Wildcard>,
    pub nbt: Option<Nbt>,
    #[serde(default = "default_multiplier")]
    pub multiplier: usize,
}

#[inline]
const fn default_multiplier() -> usize {
    1
}

#[derive(Debug, PartialEq)]
pub struct Wildcard(wildmatch::WildMatch);

impl Default for SearchDupeStashesConfig {
    fn default() -> Self {
        serde_json::from_str(include_str!(
            "../../default-search-dupe-stashes-config.json"
        ))
        .expect("Invalid default config")
    }
}

impl From<&str> for Wildcard {
    fn from(value: &str) -> Self {
        Self(wildmatch::WildMatch::new(value))
    }
}

impl<'de> Deserialize<'de> for Wildcard {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Ok(value.as_str().into())
    }
}

impl Group {
    pub fn matches(&self, item: &mc_map_reader::data::item::Item) -> bool {
        self.items.iter().any(|entry| entry.matches(item))
    }
}

impl GroupEntry {
    pub fn matches(&self, item: &mc_map_reader::data::item::Item) -> bool {
        self.matches_id(item) && self.matches_nbt(item)
    }

    fn matches_id(&self, item: &mc_map_reader::data::item::Item) -> bool {
        let Some(id) = &self.id else { return true };
        id.0.matches(&item.id)
    }

    fn matches_nbt(&self, item: &mc_map_reader::data::item::Item) -> bool {
        let Some(required_nbt) = &self.nbt else {
            return true;
        };
        let item_nbt = if let Some(item_nbt) = &item.tag {
            item_nbt
        } else {
            return required_nbt.is_empty();
        };
        filter_nbt_eq_to_item_nbt(required_nbt, item_nbt)
    }
}

fn filter_nbt_eq_to_item_nbt(
    required_nbt: &serde_json::Map<String, serde_json::Value>,
    item_nbt: &std::collections::HashMap<String, mc_map_reader::nbt::Tag>,
) -> bool {
    required_nbt.iter().all(|(required_key, required_value)| {
        let item_value = item_nbt.get(required_key);
        cmp_value(required_value, item_value)
    })
}

fn cmp_value(
    required_value: &serde_json::Value,
    item_value: Option<&mc_map_reader::nbt::Tag>,
) -> bool {
    use mc_map_reader::nbt::Tag as NbtValue;
    use serde_json::Value as JsonValue;
    match (required_value, item_value) {
        (JsonValue::Array(_), Some(NbtValue::IntArray(_))) => {
            unimplemented!()
        }
        (JsonValue::Array(_), Some(NbtValue::ByteArray(_))) => {
            unimplemented!()
        }
        (JsonValue::Array(_), Some(NbtValue::LongArray(_))) => {
            unimplemented!()
        }
        (JsonValue::Array(_), Some(NbtValue::List(_))) => {
            unimplemented!()
        }
        (JsonValue::Bool(required_value), Some(NbtValue::Byte(item_value))) => {
            *required_value == ((item_value & 1) == 1)
        }
        (JsonValue::Number(required_value), Some(NbtValue::Byte(item_value))) => {
            required_value.is_i64()
                && required_value.as_i64().expect("Error converting number") == *item_value as i64
        }
        (JsonValue::Number(required_value), Some(NbtValue::Double(item_value))) => {
            required_value.is_f64()
                && required_value.as_f64().expect("Error converting number") == *item_value
        }
        (JsonValue::Number(required_value), Some(NbtValue::Float(item_value))) => {
            required_value.is_f64()
                && required_value.as_f64().expect("Error converting number") == *item_value as f64
        }
        (JsonValue::Number(required_value), Some(NbtValue::Int(item_value))) => {
            required_value.is_i64()
                && required_value.as_i64().expect("Error converting number") == *item_value as i64
        }
        (JsonValue::Number(required_value), Some(NbtValue::Long(item_value))) => {
            required_value.is_i64()
                && required_value.as_i64().expect("Error converting number") == *item_value
        }
        (JsonValue::Number(required_value), Some(NbtValue::Short(item_value))) => {
            required_value.is_i64()
                && required_value.as_i64().expect("Error converting number") == *item_value as i64
        }
        (JsonValue::Object(required_value), Some(NbtValue::Compound(item_value))) => {
            filter_nbt_eq_to_item_nbt(required_value, item_value)
        }
        (JsonValue::String(required_value), Some(NbtValue::String(item_value))) => {
            required_value == item_value
        }
        (JsonValue::Null, None) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::search_dupe_stashes::config::default_multiplier;

    use super::{Group, GroupEntry, Wildcard};
    use mc_map_reader::{
        data::item::Item as McItem,
        nbt::{Array, List, Tag},
    };
    use serde_json::json;
    use test_case::test_case;

    #[test]
    fn test_default_multiplier() {
        assert_eq!(default_multiplier(), 1)
    }

    #[test]
    fn test_default_search_dupe_stashes_config() {
        // This test is just to make sure that the default config is valid
        super::SearchDupeStashesConfig::default();
    }

    #[test]
    fn test_wildcard() {
        let wildcard = Wildcard::from("fo*ar");
        assert_eq!(wildcard.0, wildmatch::WildMatch::new("fo*ar"));
    }

    #[test_case(Some("foo*") => true; "Does match")]
    #[test_case(Some("foo") => false; "Does not match")]
    #[test_case(None => true; "No pattern")]
    fn test_group_entry_matches_id(id: Option<&str>) -> bool {
        let entry = super::GroupEntry {
            id: id.map(Wildcard::from),
            nbt: None,
            multiplier: 1,
        };
        let item = mc_map_reader::data::item::Item {
            id: "foobar".to_string(),
            count: 1,
            tag: None,
        };
        entry.matches_id(&item)
    }

    #[test_case(Group {
        items: vec![
            GroupEntry { id: Some(Wildcard::from("item")), nbt: None, multiplier: 1 }
        ],
        threshold: 1
    }, McItem { id: String::from("item"), tag: None, count: 1 } => true; "Is Equals single")]
    #[test_case(Group {
        items: vec![
            GroupEntry { id: Some(Wildcard::from("test")), nbt: None, multiplier: 1 },
            GroupEntry { id: Some(Wildcard::from("item")), nbt: None, multiplier: 1 }
        ],
        threshold: 1
    }, McItem { id: String::from("item"), tag: None, count: 1 } => true; "Is Equals multiple")]
    #[test_case(Group {
        items: vec![
            GroupEntry { id: Some(Wildcard::from("item2")), nbt: None, multiplier: 1 }
        ],
        threshold: 1
    }, McItem { id: String::from("item"), tag: None, count: 1 } => false; "Is Not Equals single")]
    #[test_case(Group {
        items: vec![
            GroupEntry { id: Some(Wildcard::from("test")), nbt: None, multiplier: 1 },
            GroupEntry { id: Some(Wildcard::from("item2")), nbt: None, multiplier: 1 }
        ],
        threshold: 1
    }, McItem { id: String::from("item"), tag: None, count: 1 } => false; "Is not equals multiple")]
    fn test_group_matches(group: Group, item: McItem) -> bool {
        group.matches(&item)
    }

    #[test_case(None, None => true; "Nbt not required")]
    #[test_case(json!({}).as_object(), None => true; "Required Nbt is empty")]
    #[test_case(json!({"a": 1}).as_object(), None => false; "Required Nbt is not empty")]
    #[test_case(json!({"a": 1}).as_object(), Some(HashMap::from_iter([
        ("a".to_string(), mc_map_reader::nbt::Tag::Int(1))
    ])) => true; "Objects with single entry")]
    #[test_case(json!({"a": 1, "b": "test"}).as_object(), Some(HashMap::from_iter([
        ("a".to_string(), mc_map_reader::nbt::Tag::Int(1)),
        ("b".to_string(), mc_map_reader::nbt::Tag::String("test".to_string()))
    ])) => true; "Objects with multiple entries")]
    fn test_group_entry_matches_nbt(
        required_nbt: Option<&serde_json::Map<String, serde_json::Value>>,
        item_nbt: Option<std::collections::HashMap<String, mc_map_reader::nbt::Tag>>,
    ) -> bool {
        let entry = super::GroupEntry {
            id: None,
            nbt: required_nbt.map(Clone::clone),
            multiplier: 1,
        };
        let item = mc_map_reader::data::item::Item {
            id: "foobar".to_string(),
            count: 1,
            tag: item_nbt,
        };
        entry.matches_nbt(&item)
    }

    #[test_case(None, None, "foobar", None => true; "No id or nbt required")]
    #[test_case(Some("foo*"), None, "foobar", None => true; "Id matches")]
    #[test_case(Some("foo*"), None, "bar", None => false; "Id does not match")]
    #[test_case(None, json!({}).as_object(), "foobar", None => true; "Nbt is empty")]
    #[test_case(None, json!({"a": 1}).as_object(), "foobar", None => false; "Nbt is not empty")]
    fn test_group_entry_matches(
        id: Option<&str>,
        required_nbt: Option<&serde_json::Map<String, serde_json::Value>>,
        item_id: &str,
        item_nbt: Option<std::collections::HashMap<String, mc_map_reader::nbt::Tag>>,
    ) -> bool {
        let entry = super::GroupEntry {
            id: id.map(Wildcard::from),
            nbt: required_nbt.map(Clone::clone),
            multiplier: 1,
        };
        let item = mc_map_reader::data::item::Item {
            id: item_id.to_string(),
            count: 1,
            tag: item_nbt,
        };
        entry.matches(&item)
    }

    #[test]
    #[should_panic(expected = "not implemented")]
    fn test_cmp_array_with_int_array() {
        super::cmp_value(&json!([]), Some(&Tag::IntArray(Array::from(vec![]))));
    }

    #[test]
    #[should_panic(expected = "not implemented")]
    fn test_cmp_array_with_byte_array() {
        super::cmp_value(&json!([]), Some(&Tag::ByteArray(Array::from(vec![]))));
    }

    #[test]
    #[should_panic(expected = "not implemented")]
    fn test_cmp_array_with_long_array() {
        super::cmp_value(&json!([]), Some(&Tag::LongArray(Array::from(vec![]))));
    }

    #[test]
    #[should_panic(expected = "not implemented")]
    fn test_cmp_array_with_list() {
        super::cmp_value(&json!([]), Some(&Tag::List(List::from(vec![]))));
    }

    #[test_case(json!(true), Some(&Tag::Byte(1)) => true; "Bool equals true")]
    #[test_case(json!(false), Some(&Tag::Byte(0)) => true; "Bool equals false")]
    #[test_case(json!(false), Some(&Tag::Byte(1)) => false; "Bool not equals")]
    #[test_case(json!(23), Some(&Tag::Byte(23)) => true; "Byte equals")]
    #[test_case(json!(23), Some(&Tag::Byte(32)) => false; "Byte not equals")]
    #[test_case(json!(23.), Some(&Tag::Double(23.)) => true; "Double equals")]
    #[test_case(json!(23.), Some(&Tag::Double(32.)) => false; "Double not equals")]
    #[test_case(json!(23.), Some(&Tag::Float(23.)) => true; "Float equals")]
    #[test_case(json!(23.), Some(&Tag::Float(32.)) => false; "Float not equals")]
    #[test_case(json!(23), Some(&Tag::Int(23)) => true; "Int equals")]
    #[test_case(json!(23), Some(&Tag::Int(32)) => false; "Int not equals")]
    #[test_case(json!(23), Some(&Tag::Long(23)) => true; "Long equals")]
    #[test_case(json!(23), Some(&Tag::Long(32)) => false; "Long not equals")]
    #[test_case(json!(23), Some(&Tag::Short(23)) => true; "Short equals")]
    #[test_case(json!(23), Some(&Tag::Short(32)) => false; "Short not equals")]
    #[test_case(json!(null), None => true; "Null equals")]
    #[test_case(json!(null), Some(&Tag::Int(32)) => false; "Null not equals")]
    fn test_cmp_json_with_nbt(json: serde_json::Value, nbt: Option<&Tag>) -> bool {
        super::cmp_value(&json, nbt)
    }
}
