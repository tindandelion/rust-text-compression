use std::{collections::HashMap, str::Chars};

pub struct TrieSubstringCounts {
    nodes: HashMap<char, TrieNode>,
    length: usize,
}

#[derive(Debug)]
struct TrieNode {
    count: usize,
    children: HashMap<char, TrieNode>,
}

impl TrieSubstringCounts {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            length: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn insert(&mut self, substring: &str, count: usize) {
        if let Some((first_char, rest_chars)) = self.start_search(substring) {
            let root = self.nodes.entry(first_char).or_insert(TrieNode::new());
            let leaf = root.make_children(rest_chars);

            if leaf.update_count(count) == 0 {
                self.length += 1;
            }
        }
    }

    pub fn get_count_mut(&mut self, substring: &str) -> Option<&mut usize> {
        let (first_char, rest_chars) = self.start_search(substring)?;

        let mut current = self.nodes.get_mut(&first_char)?;
        for char in rest_chars {
            current = current.children.get_mut(&char)?;
        }
        if current.count > 0 {
            Some(&mut current.count)
        } else {
            None
        }
    }

    pub fn find_match(&self, text: &str) -> Option<&str> {
        let (first_char, mut rest_chars) = self.start_search(text)?;
        None
    }

    fn start_search<'a>(&self, text: &'a str) -> Option<(char, Chars<'a>)> {
        let mut chars = text.chars();
        let first_char = chars.next()?;
        Some((first_char, chars))
    }
}

impl TrieNode {
    fn new() -> Self {
        Self {
            count: 0,
            children: HashMap::new(),
        }
    }

    fn make_children(&mut self, chars: impl Iterator<Item = char>) -> &mut TrieNode {
        let mut current = self;
        for next_char in chars {
            current = current.children.entry(next_char).or_insert(TrieNode::new());
        }
        current
    }

    fn update_count(&mut self, count: usize) -> usize {
        let old_count = self.count;
        self.count = count;
        old_count
    }
}

#[cfg(test)]
mod insertion_tests {
    use super::*;

    #[test]
    fn insert_empty_string() {
        let mut counts = TrieSubstringCounts::new();
        counts.insert("", 10);

        assert_eq!(0, counts.len());
        assert_eq!(None, counts.get_count_mut(""));
    }

    #[test]
    fn insert_single_char() {
        let mut counts = TrieSubstringCounts::new();
        counts.insert("a", 10);

        assert_eq!(1, counts.len());
        assert_eq!(Some(10), counts.get_count_mut("a").copied());
        assert_eq!(None, counts.get_count_mut("ab"));
    }

    #[test]
    fn insert_long_string() {
        let mut counts = TrieSubstringCounts::new();
        counts.insert("abcd", 10);

        assert_eq!(1, counts.len());
        assert_eq!(None, counts.get_count_mut("ab"));
        assert_eq!(None, counts.get_count_mut("abc"));
        assert_eq!(Some(10), counts.get_count_mut("abcd").copied());
    }

    #[test]
    fn insert_same_string_twice_replaces_old_value() {
        let mut counts = TrieSubstringCounts::new();
        counts.insert("abcd", 10);
        counts.insert("abcd", 20);

        assert_eq!(1, counts.len());
        assert_eq!(Some(20), counts.get_count_mut("abcd").copied());
    }

    #[test]
    fn insert_prefix_of_existing_string() {
        let mut counts = TrieSubstringCounts::new();
        counts.insert("abcd", 10);
        counts.insert("abc", 20);

        assert_eq!(2, counts.len());
        assert_eq!(Some(20), counts.get_count_mut("abc").copied());
        assert_eq!(Some(10), counts.get_count_mut("abcd").copied());
    }

    #[test]
    fn insert_different_strings() {
        let mut counts = TrieSubstringCounts::new();
        counts.insert("abc", 10);
        counts.insert("def", 20);

        assert_eq!(2, counts.len());
        assert_eq!(Some(10), counts.get_count_mut("abc").copied());
        assert_eq!(Some(20), counts.get_count_mut("def").copied());
    }
}

#[cfg(test)]
mod find_match_tests {
    use super::*;

    #[test]
    fn find_match_in_empty_trie() {
        let counts = TrieSubstringCounts::new();
        assert_eq!(None, counts.find_match("abc"));
    }

    #[test]
    fn find_match_for_empty_string() {
        let mut counts = TrieSubstringCounts::new();
        counts.insert("abc", 10);
        assert_eq!(None, counts.find_match(""));
    }

    #[test]
    #[ignore]
    fn find_match_for_substring() {
        let mut counts = TrieSubstringCounts::new();
        counts.insert("abc", 10);

        assert_eq!(Some("abc"), counts.find_match("abcd"));
    }
}
