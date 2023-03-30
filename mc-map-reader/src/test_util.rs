use std::collections::HashMap;

use crate::nbt::Tag;

pub type TestDataProvider<T> = dyn Fn() -> T;

pub fn without(f: &TestDataProvider<HashMap<String, Tag>>, key: &str) -> HashMap<String, Tag> {
    let mut map = f();
    map.remove(key);
    map
}
