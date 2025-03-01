use std::collections::BTreeMap;

use super::Substring;

pub struct SubstringCounts(pub BTreeMap<Substring, usize>);

impl SubstringCounts {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, substr: &Substring) -> Option<usize> {
        self.0.get(substr).map(|&count| count)
    }

    pub fn values(&self) -> Vec<usize> {
        self.0.values().cloned().collect()
    }

    pub fn remove_less_than(&mut self, threshold: usize) {
        self.0.retain(|_, count| *count >= threshold);
    }

    pub fn insert(&mut self, substring: Substring, count: usize) {
        self.0.insert(substring, count);
    }

    pub fn to_map(self) -> BTreeMap<Substring, usize> {
        self.0
    }
}
