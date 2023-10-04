use crate::search_dupe_stashes::config::Group;
use std::collections::HashMap;

pub trait DetectionMethod {
    fn exceeds_max(&self, key: &str, amount: usize) -> bool;
}

pub struct Absolute<'a> {
    config: &'a HashMap<String, Group>,
}

impl<'a> Absolute<'a> {
    pub fn new(config: &'a HashMap<String, Group>) -> Self {
        Self { config }
    }
}

impl<'a> DetectionMethod for Absolute<'a> {
    fn exceeds_max(&self, key: &str, amount: usize) -> bool {
        let Some(group) = self.config.get(key) else {
            return false;
        };
        amount > group.threshold
    }
}
