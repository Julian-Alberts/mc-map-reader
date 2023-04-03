use std::collections::HashMap;

use crate::nbt::Tag;

pub type TestDataProvider<T> = dyn Fn() -> T;

pub fn without(f: &TestDataProvider<HashMap<String, Tag>>, key: &str) -> HashMap<String, Tag> {
    let mut map = f();
    map.remove(key);
    map
}

pub fn with(mut map: HashMap<String, Tag>, key: &str, value: Tag) -> HashMap<String, Tag> {
    map.insert(key.to_string(), value);
    map
}

pub fn merge(mut map: HashMap<String, Tag>, other: HashMap<String, Tag>) -> HashMap<String, Tag> {
    map.extend(other);
    map
}
