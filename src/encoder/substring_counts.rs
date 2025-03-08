mod btree;
mod tries;

use btree::BTreeSubstringCounts;
use tries::TrieSubstringCounts;

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

pub type DefaultSubstringCounts = TrieSubstringCounts;

pub fn default() -> DefaultSubstringCounts {
    DefaultSubstringCounts::new()
}

#[cfg(test)]
pub mod util {
    use super::*;

    pub fn get_sorted_counts(counts: &impl SubstringCounts) -> Vec<(&Substring, usize)> {
        let mut counts = counts.iter().collect::<Vec<_>>();
        counts.sort_by_key(|(substr, _)| *substr);
        counts
    }
}
