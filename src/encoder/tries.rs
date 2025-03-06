use std::{collections::HashMap, str::Chars};

use super::{substring::SubstringCount, Substring};

pub struct TrieSubstringCounts {
    nodes: HashMap<char, TrieNode>,
    length: usize,
}

#[derive(Debug)]
struct TrieNode {
    count: Option<SubstringCount>,
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

    pub fn insert(&mut self, substring: Substring, count: usize) {
        if let Some((first_char, rest_chars)) = self.start_search(substring.as_str()) {
            let root = self.nodes.entry(first_char).or_insert(TrieNode::new());
            let leaf = root.make_children(rest_chars);

            if leaf.update_count(substring, count) == 0 {
                self.length += 1;
            }
        }
    }

    pub fn get_count_mut(&mut self, substring: &Substring) -> Option<&mut usize> {
        let (first_char, rest_chars) = self.start_search(substring.as_str())?;

        let mut current = self.nodes.get_mut(&first_char)?;
        for char in rest_chars {
            current = current.get_child_mut(&char)?;
        }
        current.count.as_mut().map(|v| &mut v.count)
    }

    pub fn find_match(&self, text: &str) -> Option<&str> {
        let (first_char, rest_chars) = self.start_search(text)?;

        let mut current = self.nodes.get(&first_char)?;
        let mut best_match: Option<&SubstringCount> = current.count.as_ref();

        for next_char in rest_chars {
            if let Some(child) = current.get_child(&next_char) {
                best_match = child.count.as_ref().or(best_match);
                current = child;
            } else {
                break;
            }
        }
        best_match.map(|v| v.value.as_str())
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Substring, &usize)> {
        TrieIterator::new(self).map(|v| (&v.value, &v.count))
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
            count: None,
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

    fn get_child(&self, char: &char) -> Option<&TrieNode> {
        self.children.get(char)
    }

    fn get_child_mut(&mut self, char: &char) -> Option<&mut TrieNode> {
        self.children.get_mut(char)
    }

    fn update_count(&mut self, value: Substring, count: usize) -> usize {
        let old_count = self.count.replace(SubstringCount::new(value, count));
        old_count.map_or(0, |v| v.count)
    }
}

struct TrieIterator<'a> {
    stack: Vec<&'a TrieNode>,
}

impl<'a> TrieIterator<'a> {
    fn new(trie: &'a TrieSubstringCounts) -> Self {
        let mut stack = Vec::with_capacity(trie.len());
        for node in trie.nodes.values() {
            stack.push(node);
        }
        Self { stack }
    }
}

impl<'a> Iterator for TrieIterator<'a> {
    type Item = &'a SubstringCount;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(current) = self.stack.pop() {
            for child in current.children.values() {
                self.stack.push(child);
            }
            if current.count.is_some() {
                return current.count.as_ref();
            }
        }
        None
    }
}

#[cfg(test)]
mod insertion_tests {
    use super::*;

    #[test]
    fn insert_empty_string() {
        let mut counts = TrieSubstringCounts::new();
        counts.insert("".into(), 10);

        assert_eq!(0, counts.len());
        assert_eq!(None, counts.get_count_mut(&"".into()));
    }

    #[test]
    fn insert_single_char() {
        let mut counts = TrieSubstringCounts::new();
        counts.insert("a".into(), 10);

        assert_eq!(1, counts.len());
        assert_eq!(Some(&mut 10), counts.get_count_mut(&"a".into()));
        assert_eq!(None, counts.get_count_mut(&"ab".into()));
    }

    #[test]
    fn insert_long_string() {
        let mut counts = TrieSubstringCounts::new();
        counts.insert("abcd".into(), 10);

        assert_eq!(1, counts.len());
        assert_eq!(None, counts.get_count_mut(&"ab".into()));
        assert_eq!(None, counts.get_count_mut(&"abc".into()));
        assert_eq!(Some(&mut 10), counts.get_count_mut(&"abcd".into()));
    }

    #[test]
    fn insert_same_string_twice_replaces_old_value() {
        let mut counts = TrieSubstringCounts::new();
        counts.insert("abcd".into(), 10);
        counts.insert("abcd".into(), 20);

        assert_eq!(1, counts.len());
        assert_eq!(Some(&mut 20), counts.get_count_mut(&"abcd".into()));
    }

    #[test]
    fn insert_prefix_of_existing_string() {
        let mut counts = TrieSubstringCounts::new();
        counts.insert("abcd".into(), 10);
        counts.insert("abc".into(), 20);

        assert_eq!(2, counts.len());
        assert_eq!(Some(&mut 20), counts.get_count_mut(&"abc".into()));
        assert_eq!(Some(&mut 10), counts.get_count_mut(&"abcd".into()));
    }

    #[test]
    fn insert_different_strings() {
        let mut counts = TrieSubstringCounts::new();
        counts.insert("abc".into(), 10);
        counts.insert("def".into(), 20);

        assert_eq!(2, counts.len());
        assert_eq!(Some(&mut 10), counts.get_count_mut(&"abc".into()));
        assert_eq!(Some(&mut 20), counts.get_count_mut(&"def".into()));
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
        counts.insert("abc".into(), 10);
        assert_eq!(None, counts.find_match(""));
    }

    #[test]
    fn find_match_for_substrings() {
        let mut counts = TrieSubstringCounts::new();
        counts.insert("abc".into(), 10);
        counts.insert("abcde".into(), 30);

        assert_eq!(None, counts.find_match("ab"));
        assert_eq!(Some("abc"), counts.find_match("abcd"));
        assert_eq!(Some("abcde"), counts.find_match("abcde"));
    }

    #[test]
    fn finds_longest_possible_match() {
        let mut counts = TrieSubstringCounts::new();
        counts.insert("abc".into(), 10);
        counts.insert("abcd".into(), 20);

        let best_match = counts.find_match("abcd");
        assert_eq!(Some("abcd"), best_match);
    }

    #[test]
    fn finds_match_in_different_branches() {
        let mut counts = TrieSubstringCounts::new();
        counts.insert("abc".into(), 10);
        counts.insert("abcd".into(), 10);
        counts.insert("def".into(), 20);
        counts.insert("abx".into(), 30);

        assert_eq!(Some("abc"), counts.find_match("abc"));
        assert_eq!(Some("abcd"), counts.find_match("abcde"));
        assert_eq!(Some("def"), counts.find_match("def"));
        assert_eq!(Some("abx"), counts.find_match("abx"));
        assert_eq!(None, counts.find_match("xyz"));
    }

    #[test]
    fn finds_match_with_extra_characters() {
        let mut counts = TrieSubstringCounts::new();
        counts.insert("hello".into(), 10);

        assert_eq!(Some("hello"), counts.find_match("hello world"));
        assert_eq!(Some("hello"), counts.find_match("hello!"));
    }

    #[test]
    fn finds_match_with_multiple_possibilities() {
        let mut counts = TrieSubstringCounts::new();
        counts.insert("a".into(), 10);
        counts.insert("ab".into(), 20);
        counts.insert("abc".into(), 30);
        counts.insert("abcd".into(), 40);

        assert_eq!(Some("abcd"), counts.find_match("abcdef"));
        assert_eq!(Some("abc"), counts.find_match("abc"));
        assert_eq!(Some("ab"), counts.find_match("abxyz"));
        assert_eq!(Some("a"), counts.find_match("a"));
    }

    #[test]
    fn finds_match_with_unicode() {
        let mut counts = TrieSubstringCounts::new();
        counts.insert("こんにちは".into(), 10);
        counts.insert("世界".into(), 20);

        assert_eq!(Some("こんにちは"), counts.find_match("こんにちは世界"));
        assert_eq!(Some("世界"), counts.find_match("世界"));
    }
}

#[cfg(test)]
mod iterator_tests {
    use super::*;

    #[test]
    fn empty_trie() {
        let counts = TrieSubstringCounts::new();
        let strings = counts.iter().collect::<Vec<_>>();
        assert!(strings.is_empty());
    }

    #[test]
    fn iterate_over_entries() {
        let mut counts = TrieSubstringCounts::new();
        counts.insert("abc".into(), 10);
        counts.insert("abx".into(), 10);
        counts.insert("abcd".into(), 20);
        counts.insert("abcde".into(), 30);
        counts.insert("def".into(), 40);

        let mut entries = counts.iter().collect::<Vec<_>>();
        entries.sort_by_key(|(s, _)| s.to_string());

        assert_eq!(
            vec![
                (&"abc".into(), &10),
                (&"abcd".into(), &20),
                (&"abcde".into(), &30),
                (&"abx".into(), &10),
                (&"def".into(), &40),
            ],
            entries
        );
    }
}
