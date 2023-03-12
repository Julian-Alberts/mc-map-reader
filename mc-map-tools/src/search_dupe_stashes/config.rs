use std::collections::HashMap;

use serde::Deserialize;

type Nbt = serde_json::value::Map<String, serde_json::Value>;

#[derive(Debug, PartialEq, Deserialize)]
pub struct SearchDupeStashesConfig {
    pub groups: Vec<Group>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Group {
    pub name: String,
    pub items: Vec<Item>,
    pub threshold: usize,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Item {
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

impl Item {
    pub fn matches(&self, item: &mc_map_reader::nbt_data::block_entity::Item) -> bool {
        self.matches_id(item) && self.matches_nbt(item)
    }

    fn matches_id(&self, item: &mc_map_reader::nbt_data::block_entity::Item) -> bool {
        let Some(id) = &self.id else {
            return true
        };
        id.0.matches(item.id())
    }

    fn matches_nbt(&self, item: &mc_map_reader::nbt_data::block_entity::Item) -> bool {
        let Some(required_nbt) = &self.nbt else {
            return true
        };
        let item_nbt = if let Some(item_nbt) = item.tag() {
            item_nbt
        } else {
            return required_nbt.len() == 0;
        };
        filter_nbt_eq_to_item_nbt(required_nbt, item_nbt)
    }
}

fn filter_nbt_eq_to_item_nbt(
    required_nbt: &serde_json::Map<String, serde_json::Value>,
    item_nbt: &std::collections::HashMap<String, mc_map_reader::nbt::Tag>,
) -> bool {
    use mc_map_reader::nbt::Tag as NbtValue;
    use serde_json::Value as JsonValue;
    required_nbt.iter().all(|(required_key, required_value)| {
        let item_value = item_nbt.get(required_key);

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
                    && required_value.as_i64().expect("Error converting number") as i8
                        == *item_value
            }
            (JsonValue::Number(required_value), Some(NbtValue::Double(item_value))) => {
                required_value.is_f64()
                    && required_value.as_f64().expect("Error converting number") as f64
                        == *item_value
            }
            (JsonValue::Number(required_value), Some(NbtValue::Float(item_value))) => {
                required_value.is_f64()
                    && required_value.as_f64().expect("Error converting number") as f32
                        == *item_value
            }
            (JsonValue::Number(required_value), Some(NbtValue::Int(item_value))) => {
                required_value.is_i64()
                    && required_value.as_i64().expect("Error converting number") as i32
                        == *item_value
            }
            (JsonValue::Number(required_value), Some(NbtValue::Long(item_value))) => {
                required_value.is_i64()
                    && required_value.as_i64().expect("Error converting number") as i64
                        == *item_value
            }
            (JsonValue::Number(required_value), Some(NbtValue::Short(item_value))) => {
                required_value.is_i64()
                    && required_value.as_i64().expect("Error converting number") as i16
                        == *item_value
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
    })
}
