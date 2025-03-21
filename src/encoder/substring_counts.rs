use crate::{
    encoder::{substring::SubstringCount, Substring},
    trie::Trie,
};

pub struct SubstringCounts {
    trie: Trie<usize>,
}

impl SubstringCounts {
    pub fn new() -> Self {
        Self { trie: Trie::new() }
    }

    pub fn len(&self) -> usize {
        self.trie.len()
    }

    pub fn insert(&mut self, substring: Substring, count: usize) {
        self.trie.insert(substring, count);
    }

    pub fn get_count_mut(&mut self, substring: &Substring) -> Option<&mut usize> {
        self.trie.get_mut(substring)
    }

    pub fn contains_key(&self, substring: &Substring) -> bool {
        self.trie.get(substring).is_some()
    }

    pub fn find_match(&self, text: &str) -> Option<SubstringCount> {
        self.trie
            .find_match(text)
            .map(|(key, value)| SubstringCount {
                value: key.clone(),
                count: *value,
            })
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Substring, usize)> {
        self.trie.iter().map(|(key, value)| (key, *value))
    }

    pub fn retain<F>(&mut self, f: F)
    where
        F: Fn(&Substring, usize) -> bool,
    {
        self.trie.retain(|key, value| f(key, *value));
    }

    #[cfg(test)]
    pub fn get_sorted_counts(&self) -> Vec<(&Substring, usize)> {
        let mut counts = self.iter().collect::<Vec<_>>();
        counts.sort_by_key(|(substr, _)| *substr);
        counts
    }
}
