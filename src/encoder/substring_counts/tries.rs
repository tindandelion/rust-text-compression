use crate::{
    encoder::{substring::SubstringCount, Substring},
    trie::Trie,
};

use super::SubstringCounts;

pub struct TrieSubstringCounts {
    trie: Trie<usize>,
}

impl SubstringCounts for TrieSubstringCounts {
    fn len(&self) -> usize {
        self.trie.len()
    }

    fn insert(&mut self, substring: Substring, count: usize) {
        self.trie.insert(substring, count);
    }

    fn get_count_mut(&mut self, substring: &Substring) -> Option<&mut usize> {
        self.trie.get_mut(substring)
    }

    fn contains_key(&self, substring: &Substring) -> bool {
        self.trie.get(substring).is_some()
    }

    fn find_match(&self, text: &str) -> Option<SubstringCount> {
        self.trie
            .find_match(text)
            .map(|(key, value)| SubstringCount {
                value: key.clone(),
                count: *value,
            })
    }

    fn iter(&self) -> impl Iterator<Item = (&Substring, usize)> {
        self.trie.iter().map(|(key, value)| (key, *value))
    }

    fn retain<F>(&mut self, f: F)
    where
        F: Fn(&Substring, usize) -> bool,
    {
        self.trie.retain(|key, value| f(key, *value));
    }
}

impl TrieSubstringCounts {
    pub fn new() -> Self {
        Self { trie: Trie::new() }
    }
}
