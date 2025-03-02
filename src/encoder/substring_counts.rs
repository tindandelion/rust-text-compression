use std::collections::BTreeMap;

use super::Substring;

pub struct SubstringCounts(BTreeMap<Substring, usize>);

impl SubstringCounts {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn find_match(&self, text: &str) -> Option<&Substring> {
        self.0.keys().find(|&substr| substr.matches_start(text))
    }

    pub fn get(&self, substr: &Substring) -> Option<usize> {
        self.0.get(substr).map(|&count| count)
    }

    pub fn get_mut(&mut self, substr: &Substring) -> Option<&mut usize> {
        self.0.get_mut(substr)
    }

    pub fn contains_key(&self, substr: &Substring) -> bool {
        self.0.contains_key(substr)
    }

    pub fn values(&self) -> impl Iterator<Item = &usize> {
        self.0.values()
    }

    pub fn into_iter(self) -> impl Iterator<Item = (Substring, usize)> {
        self.0.into_iter()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Substring, &usize)> {
        self.0.iter()
    }

    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&Substring, &mut usize) -> bool,
    {
        self.0.retain(f);
    }

    pub fn insert(&mut self, substring: Substring, count: usize) {
        self.0.insert(substring, count);
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }
}
