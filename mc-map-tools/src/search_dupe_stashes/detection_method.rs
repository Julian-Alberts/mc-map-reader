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

#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    use test_case::test_case;

    use crate::search_dupe_stashes::config::Group;

    use super::{Absolute, DetectionMethod};

    #[test_case(&[], "test", 42 => false; "No groups")]
    #[test_case(&[("test", 43)], "test", 42 => false; "Does not exceed max")]
    #[test_case(&[("test", 41)], "test", 42 => true; "Does exceed max")]
    #[test_case(&[("other", 312),("test", 41),("even more", 124)], "test", 42 => true; "Multiple")]
    fn absolute_detection_method(groups: &[(&str, usize)], key: &str, amount: usize) -> bool {
        let config = HashMap::from_iter(groups.iter().map(|(key, threshold)| {
            (key.to_string(),
            Group {
                items: Vec::default(),
                threshold: *threshold,
            })
        }));
        let abs = Absolute::new(&config);
        abs.exceeds_max(key, amount)
    }

}

