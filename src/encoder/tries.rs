use std::{collections::HashMap, str::Chars};

pub struct TrieSubstringCounts {
    nodes: HashMap<char, TrieNode>,
    length: usize,
}

#[derive(Debug)]
struct TrieNode {
    substring: Option<String>,
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

    pub fn insert(&mut self, substring: String, count: usize) {
        if let Some((first_char, rest_chars)) = self.start_search(&substring) {
            let root = self.nodes.entry(first_char).or_insert(TrieNode::new());
            let leaf = root.make_children(rest_chars);

            if leaf.update_count(substring, count) == 0 {
                self.length += 1;
            }
        }
    }

    pub fn get_count_mut(&mut self, substring: &str) -> Option<&mut usize> {
        let (first_char, rest_chars) = self.start_search(substring)?;

        let mut current = self.nodes.get_mut(&first_char)?;
        for char in rest_chars {
            current = current.get_child_mut(&char)?;
        }

        if current.count > 0 {
            Some(&mut current.count)
        } else {
            None
        }
    }

    pub fn find_match(&self, text: &str) -> Option<&str> {
        let (first_char, rest_chars) = self.start_search(text)?;

        let mut current = self.nodes.get(&first_char)?;
        let mut best_match: Option<&str> = current.substring.as_deref();

        for next_char in rest_chars {
            if let Some(child) = current.get_child(&next_char) {
                best_match = child.substring.as_deref().or(best_match);
                current = child;
            } else {
                break;
            }
        }
        best_match
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &usize)> {
        TrieIterator::new(self)
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
            substring: None,
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

    fn update_count(&mut self, value: String, count: usize) -> usize {
        let old_count = self.count;
        self.substring = Some(value);
        self.count = count;
        old_count
    }
}

struct TrieIterator<'a> {
    trie: &'a TrieSubstringCounts,
    stack: Vec<&'a TrieNode>,
}

impl<'a> TrieIterator<'a> {
    fn new(trie: &'a TrieSubstringCounts) -> Self {
        let mut stack = Vec::with_capacity(trie.len());
        for node in trie.nodes.values() {
            stack.push(node);
        }
        Self { trie, stack }
    }
}

impl<'a> Iterator for TrieIterator<'a> {
    type Item = (&'a String, &'a usize);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(current) = self.stack.pop() {
            for child in current.children.values() {
                self.stack.push(child);
            }
            if let Some(substring) = &current.substring {
                return Some((substring, &current.count));
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
        counts.insert("".to_string(), 10);

        assert_eq!(0, counts.len());
        assert_eq!(None, counts.get_count_mut(""));
    }

    #[test]
    fn insert_single_char() {
        let mut counts = TrieSubstringCounts::new();
        counts.insert("a".to_string(), 10);

        assert_eq!(1, counts.len());
        assert_eq!(Some(&mut 10), counts.get_count_mut("a"));
        assert_eq!(None, counts.get_count_mut("ab"));
    }

    #[test]
    fn insert_long_string() {
        let mut counts = TrieSubstringCounts::new();
        counts.insert("abcd".to_string(), 10);

        assert_eq!(1, counts.len());
        assert_eq!(None, counts.get_count_mut("ab"));
        assert_eq!(None, counts.get_count_mut("abc"));
        assert_eq!(Some(&mut 10), counts.get_count_mut("abcd"));
    }

    #[test]
    fn insert_same_string_twice_replaces_old_value() {
        let mut counts = TrieSubstringCounts::new();
        counts.insert("abcd".to_string(), 10);
        counts.insert("abcd".to_string(), 20);

        assert_eq!(1, counts.len());
        assert_eq!(Some(&mut 20), counts.get_count_mut("abcd"));
    }

    #[test]
    fn insert_prefix_of_existing_string() {
        let mut counts = TrieSubstringCounts::new();
        counts.insert("abcd".to_string(), 10);
        counts.insert("abc".to_string(), 20);

        assert_eq!(2, counts.len());
        assert_eq!(Some(&mut 20), counts.get_count_mut("abc"));
        assert_eq!(Some(&mut 10), counts.get_count_mut("abcd"));
    }

    #[test]
    fn insert_different_strings() {
        let mut counts = TrieSubstringCounts::new();
        counts.insert("abc".to_string(), 10);
        counts.insert("def".to_string(), 20);

        assert_eq!(2, counts.len());
        assert_eq!(Some(&mut 10), counts.get_count_mut("abc"));
        assert_eq!(Some(&mut 20), counts.get_count_mut("def"));
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
        counts.insert("abc".to_string(), 10);
        assert_eq!(None, counts.find_match(""));
    }

    #[test]
    fn find_match_for_substrings() {
        let mut counts = TrieSubstringCounts::new();
        counts.insert("abc".to_string(), 10);
        counts.insert("abcde".to_string(), 30);

        assert_eq!(None, counts.find_match("ab"));
        assert_eq!(Some("abc"), counts.find_match("abcd"));
        assert_eq!(Some("abcde"), counts.find_match("abcde"));
    }

    #[test]
    fn finds_longest_possible_match() {
        let mut counts = TrieSubstringCounts::new();
        counts.insert("abc".to_string(), 10);
        counts.insert("abcd".to_string(), 20);

        let best_match = counts.find_match("abcd");
        assert_eq!(Some("abcd"), best_match);
    }

    #[test]
    fn finds_match_in_different_branches() {
        let mut counts = TrieSubstringCounts::new();
        counts.insert("abc".to_string(), 10);
        counts.insert("abcd".to_string(), 10);
        counts.insert("def".to_string(), 20);
        counts.insert("abx".to_string(), 30);

        assert_eq!(Some("abc"), counts.find_match("abc"));
        assert_eq!(Some("abcd"), counts.find_match("abcde"));
        assert_eq!(Some("def"), counts.find_match("def"));
        assert_eq!(Some("abx"), counts.find_match("abx"));
        assert_eq!(None, counts.find_match("xyz"));
    }

    #[test]
    fn finds_match_with_extra_characters() {
        let mut counts = TrieSubstringCounts::new();
        counts.insert("hello".to_string(), 10);

        assert_eq!(Some("hello"), counts.find_match("hello world"));
        assert_eq!(Some("hello"), counts.find_match("hello!"));
    }

    #[test]
    fn finds_match_with_multiple_possibilities() {
        let mut counts = TrieSubstringCounts::new();
        counts.insert("a".to_string(), 10);
        counts.insert("ab".to_string(), 20);
        counts.insert("abc".to_string(), 30);
        counts.insert("abcd".to_string(), 40);

        assert_eq!(Some("abcd"), counts.find_match("abcdef"));
        assert_eq!(Some("abc"), counts.find_match("abc"));
        assert_eq!(Some("ab"), counts.find_match("abxyz"));
        assert_eq!(Some("a"), counts.find_match("a"));
    }

    #[test]
    fn finds_match_with_unicode() {
        let mut counts = TrieSubstringCounts::new();
        counts.insert("こんにちは".to_string(), 10);
        counts.insert("世界".to_string(), 20);

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
        counts.insert("abc".to_string(), 10);
        counts.insert("abx".to_string(), 10);
        counts.insert("abcd".to_string(), 20);
        counts.insert("abcde".to_string(), 30);
        counts.insert("def".to_string(), 40);

        let mut entries = counts.iter().collect::<Vec<_>>();
        entries.sort_by_key(|(s, _)| s.to_string());

        assert_eq!(
            vec![
                (&"abc".to_string(), &10),
                (&"abcd".to_string(), &20),
                (&"abcde".to_string(), &30),
                (&"abx".to_string(), &10),
                (&"def".to_string(), &40),
            ],
            entries
        );
    }
}
