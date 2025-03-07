use std::collections::BTreeMap;

use super::{substring::SubstringCount, Substring};

pub trait SubstringCounts {
    fn len(&self) -> usize;
    fn find_match(&self, text: &str) -> Option<SubstringCount>;
    fn get_count_mut(&mut self, substr: &Substring) -> Option<&mut usize>;
    fn contains_key(&self, substr: &Substring) -> bool;
    fn iter(&self) -> impl Iterator<Item = (&Substring, usize)>;
    fn retain<F>(&mut self, f: F)
    where
        F: Fn(&Substring, usize) -> bool;
    fn insert(&mut self, substring: Substring, count: usize);
}

pub fn btree() -> BTreeSubstringCounts {
    BTreeSubstringCounts::new()
}

pub struct BTreeSubstringCounts(BTreeMap<Substring, usize>);

impl BTreeSubstringCounts {
    fn new() -> Self {
        Self(BTreeMap::new())
    }
}

impl SubstringCounts for BTreeSubstringCounts {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn find_match(&self, text: &str) -> Option<SubstringCount> {
        self.0
            .iter()
            .find(|(substr, _)| substr.matches_start(text))
            .map(|(substr, count)| SubstringCount {
                value: substr.clone(),
                count: *count,
            })
    }

    fn get_count_mut(&mut self, substr: &Substring) -> Option<&mut usize> {
        self.0.get_mut(substr)
    }

    fn contains_key(&self, substr: &Substring) -> bool {
        self.0.contains_key(substr)
    }

    fn iter(&self) -> impl Iterator<Item = (&Substring, usize)> {
        self.0.iter().map(|(substr, count)| (substr, *count))
    }

    fn retain<F>(&mut self, f: F)
    where
        F: Fn(&Substring, usize) -> bool,
    {
        self.0.retain(|substr, count| f(substr, *count));
    }

    fn insert(&mut self, substring: Substring, count: usize) {
        self.0.insert(substring, count);
    }
}
