use std::{collections::HashMap, str::Chars};

use crate::encoder::{substring::SubstringCount, Substring};

use super::SubstringCounts;

pub struct Trie {
    nodes: HashMap<char, TrieNode>,
    length: usize,
}

pub struct TrieSubstringCounts {
    trie: Trie,
}

#[derive(Debug)]
struct NodeValue<V> {
    key: Substring,
    value: V,
}

#[derive(Debug)]
struct TrieNode {
    value: Option<NodeValue<usize>>,
    children: HashMap<char, TrieNode>,
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

impl Trie {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            length: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    fn insert(&mut self, substring: Substring, count: usize) {
        if let Some((first_char, rest_chars)) = start_search(substring.as_str()) {
            let root = self.nodes.entry(first_char).or_insert(TrieNode::new());
            let leaf = root.make_children(rest_chars);

            if leaf.update_value(substring, count).is_none() {
                self.length += 1;
            }
        }
    }

    fn get_mut(&mut self, substring: &Substring) -> Option<&mut usize> {
        let (first_char, rest_chars) = start_search(substring.as_str())?;

        let mut current = self.nodes.get_mut(&first_char)?;
        for char in rest_chars {
            current = current.get_child_mut(&char)?;
        }
        current.value.as_mut().map(|v| &mut v.value)
    }

    fn get(&self, substring: &Substring) -> Option<&usize> {
        let (first_char, rest_chars) = start_search(substring.as_str())?;

        let mut current = self.nodes.get(&first_char)?;
        for char in rest_chars {
            current = current.get_child(&char)?;
        }
        current.value.as_ref().map(|v| &v.value)
    }

    fn find_match(&self, text: &str) -> Option<(&Substring, &usize)> {
        let (first_char, rest_chars) = start_search(text)?;

        let mut current = self.nodes.get(&first_char)?;
        let mut best_match: Option<&NodeValue<usize>> = current.value.as_ref();

        for next_char in rest_chars {
            if let Some(child) = current.get_child(&next_char) {
                best_match = child.value.as_ref().or(best_match);
                current = child;
            } else {
                break;
            }
        }
        best_match.map(|v| (&v.key, &v.value))
    }

    fn iter(&self) -> impl Iterator<Item = (&Substring, &usize)> {
        TrieIterator::new(self)
    }

    fn retain<F>(&mut self, f: F)
    where
        F: Fn(&Substring, &usize) -> bool,
    {
        self.length = RetainIf::new(self).run(f);
    }
}

impl TrieSubstringCounts {
    pub fn new() -> Self {
        Self { trie: Trie::new() }
    }
}

fn start_search<'a>(text: &'a str) -> Option<(char, Chars<'a>)> {
    let mut chars = text.chars();
    let first_char = chars.next()?;
    Some((first_char, chars))
}

impl TrieNode {
    fn new() -> Self {
        Self {
            value: None,
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

    fn update_value(&mut self, key: Substring, value: usize) -> Option<usize> {
        self.value
            .replace(NodeValue { key, value })
            .map(|v| v.value)
    }
}

struct TrieIterator<'a> {
    stack: Vec<&'a TrieNode>,
}

impl<'a> TrieIterator<'a> {
    fn new(trie: &'a Trie) -> Self {
        let mut stack = Vec::with_capacity(trie.length);
        stack.extend(trie.nodes.values());
        Self { stack }
    }
}

impl<'a> Iterator for TrieIterator<'a> {
    type Item = (&'a Substring, &'a usize);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(current) = self.stack.pop() {
            self.stack.extend(current.children.values());
            if let Some(count) = current.value.as_ref() {
                return Some((&count.key, &count.value));
            }
        }
        None
    }
}

struct RetainIf<'a> {
    stack: Vec<&'a mut TrieNode>,
}

impl<'a> RetainIf<'a> {
    fn new(trie: &'a mut Trie) -> Self {
        let mut stack = Vec::with_capacity(trie.length);
        stack.extend(trie.nodes.values_mut());
        Self { stack }
    }

    fn run<F>(&mut self, condition: F) -> usize
    where
        F: Fn(&Substring, &usize) -> bool,
    {
        let mut new_length: usize = 0;
        while let Some(current) = self.stack.pop() {
            self.stack.extend(current.children.values_mut());

            if let Some(count) = current.value.as_mut() {
                let should_retain = condition(&count.key, &count.value);
                if !should_retain {
                    current.value = None;
                } else {
                    new_length += 1;
                }
            }
        }
        new_length
    }
}

#[cfg(test)]
mod insertion_tests {
    use super::*;

    #[test]
    fn insert_single_char() {
        let mut counts = TrieSubstringCounts::new();
        counts.insert("a".into(), 10);

        assert_eq!(1, counts.len());
        assert_contains_substring(&mut counts, "a", 10);
        assert_does_not_contain_substring(&mut counts, "ab");
    }

    #[test]
    fn insert_long_string() {
        let mut counts = TrieSubstringCounts::new();
        counts.insert("abcd".into(), 10);

        assert_eq!(1, counts.len());
        assert_does_not_contain_substring(&mut counts, "ab");
        assert_does_not_contain_substring(&mut counts, "abc");
        assert_contains_substring(&mut counts, "abcd", 10);
    }

    #[test]
    fn insert_same_string_twice_replaces_old_value() {
        let mut counts = TrieSubstringCounts::new();
        counts.insert("abcd".into(), 10);
        counts.insert("abcd".into(), 20);

        assert_eq!(1, counts.len());
        assert_contains_substring(&mut counts, "abcd", 20);
    }

    #[test]
    fn insert_prefix_of_existing_string() {
        let mut counts = TrieSubstringCounts::new();
        counts.insert("abcd".into(), 10);
        counts.insert("abc".into(), 20);

        assert_eq!(2, counts.len());
        assert_contains_substring(&mut counts, "abc", 20);
        assert_contains_substring(&mut counts, "abcd", 10);
    }

    #[test]
    fn insert_different_strings() {
        let mut counts = TrieSubstringCounts::new();
        counts.insert("abc".into(), 10);
        counts.insert("def".into(), 20);

        assert_eq!(2, counts.len());
        assert_contains_substring(&mut counts, "abc", 10);
        assert_contains_substring(&mut counts, "def", 20);
    }

    fn assert_contains_substring(
        counts: &mut TrieSubstringCounts,
        substring: &str,
        mut count: usize,
    ) {
        assert_eq!(Some(&mut count), counts.get_count_mut(&substring.into()));
        assert!(counts.contains_key(&substring.into()));
    }

    fn assert_does_not_contain_substring(counts: &mut TrieSubstringCounts, substring: &str) {
        assert_eq!(None, counts.get_count_mut(&substring.into()));
        assert!(!counts.contains_key(&substring.into()));
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
        assert_eq!(substring_count("abc", 10), counts.find_match("abcd"));
        assert_eq!(substring_count("abcde", 30), counts.find_match("abcde"));
    }

    #[test]
    fn finds_longest_possible_match() {
        let mut counts = TrieSubstringCounts::new();
        counts.insert("abc".into(), 10);
        counts.insert("abcd".into(), 20);

        let best_match = counts.find_match("abcd");
        assert_eq!(substring_count("abcd", 20), best_match);
    }

    #[test]
    fn finds_match_in_different_branches() {
        let mut counts = TrieSubstringCounts::new();
        counts.insert("abc".into(), 10);
        counts.insert("abcd".into(), 10);
        counts.insert("def".into(), 20);
        counts.insert("abx".into(), 30);

        assert_eq!(substring_count("abc", 10), counts.find_match("abc"));
        assert_eq!(substring_count("abcd", 10), counts.find_match("abcde"));
        assert_eq!(substring_count("def", 20), counts.find_match("def"));
        assert_eq!(substring_count("abx", 30), counts.find_match("abx"));
        assert_eq!(None, counts.find_match("xyz"));
    }

    #[test]
    fn finds_match_with_extra_characters() {
        let mut counts = TrieSubstringCounts::new();
        counts.insert("hello".into(), 10);

        assert_eq!(
            substring_count("hello", 10),
            counts.find_match("hello world")
        );
        assert_eq!(substring_count("hello", 10), counts.find_match("hello!"));
    }

    #[test]
    fn finds_match_with_multiple_possibilities() {
        let mut counts = TrieSubstringCounts::new();
        counts.insert("a".into(), 10);
        counts.insert("ab".into(), 20);
        counts.insert("abc".into(), 30);
        counts.insert("abcd".into(), 40);

        assert_eq!(substring_count("abcd", 40), counts.find_match("abcdef"));
        assert_eq!(substring_count("abc", 30), counts.find_match("abc"));
        assert_eq!(substring_count("ab", 20), counts.find_match("abxyz"));
        assert_eq!(substring_count("a", 10), counts.find_match("a"));
    }

    #[test]
    fn finds_match_with_unicode() {
        let mut counts = TrieSubstringCounts::new();
        counts.insert("こんにちは".into(), 10);
        counts.insert("世界".into(), 20);

        assert_eq!(
            substring_count("こんにちは", 10),
            counts.find_match("こんにちは世界")
        );
        assert_eq!(substring_count("世界", 20), counts.find_match("世界"));
    }

    fn substring_count(value: &str, count: usize) -> Option<SubstringCount> {
        Some(SubstringCount {
            value: value.into(),
            count,
        })
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
                (&"abc".into(), 10),
                (&"abcd".into(), 20),
                (&"abcde".into(), 30),
                (&"abx".into(), 10),
                (&"def".into(), 40),
            ],
            entries
        );
    }
}

#[cfg(test)]
mod retain_tests {
    use super::*;

    #[test]
    fn retain_entries() {
        let mut counts = TrieSubstringCounts::new();
        counts.insert("abc".into(), 10);
        counts.insert("abx".into(), 10);
        counts.insert("abcd".into(), 20);
        counts.insert("xyz".into(), 30);

        counts.retain(|_, count| count > 10);
        assert_eq!(2, counts.len());
        assert_eq!(vec!["abcd", "xyz"], collect_strings(&counts));
    }

    fn collect_strings(counts: &TrieSubstringCounts) -> Vec<String> {
        let mut strings = counts
            .iter()
            .map(|(s, _)| s.to_string())
            .collect::<Vec<_>>();
        strings.sort();
        strings
    }
}
