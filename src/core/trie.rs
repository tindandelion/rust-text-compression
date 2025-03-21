use std::{collections::HashMap, str::Chars};

use crate::encoder::Substring;

pub struct Trie<V> {
    nodes: HashMap<char, TrieNode<V>>,
    length: usize,
}

#[derive(Debug)]
struct NodeValue<V> {
    key: Substring,
    value: V,
}

#[derive(Debug)]
struct TrieNode<V> {
    value: Option<NodeValue<V>>,
    children: HashMap<char, TrieNode<V>>,
}

impl<V> Trie<V> {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            length: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn insert(&mut self, key: Substring, value: V) {
        if let Some((first_char, rest_chars)) = start_search(key.as_str()) {
            let root = self.nodes.entry(first_char).or_insert(TrieNode::new());
            let leaf = root.make_children(rest_chars);

            if leaf.update_value(key, value).is_none() {
                self.length += 1;
            }
        }
    }

    pub fn get_mut(&mut self, substring: &Substring) -> Option<&mut V> {
        let (first_char, rest_chars) = start_search(substring.as_str())?;

        let mut current = self.nodes.get_mut(&first_char)?;
        for char in rest_chars {
            current = current.get_child_mut(&char)?;
        }
        current.value.as_mut().map(|v| &mut v.value)
    }

    pub fn get(&self, substring: &Substring) -> Option<&V> {
        let (first_char, rest_chars) = start_search(substring.as_str())?;

        let mut current = self.nodes.get(&first_char)?;
        for char in rest_chars {
            current = current.get_child(&char)?;
        }
        current.value.as_ref().map(|v| &v.value)
    }

    pub fn find_match(&self, text: &str) -> Option<(&Substring, &V)> {
        let (first_char, rest_chars) = start_search(text)?;

        let mut current = self.nodes.get(&first_char)?;
        let mut best_match: Option<&NodeValue<V>> = current.value.as_ref();

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

    pub fn iter(&self) -> impl Iterator<Item = (&Substring, &V)> {
        TrieIterator::new(self)
    }

    pub fn retain<F>(&mut self, f: F)
    where
        F: Fn(&Substring, &V) -> bool,
    {
        self.length = RetainIf::new(self).run(f);
    }
}

impl<V> TrieNode<V> {
    fn new() -> Self {
        Self {
            value: None,
            children: HashMap::new(),
        }
    }

    fn make_children(&mut self, chars: impl Iterator<Item = char>) -> &mut TrieNode<V> {
        let mut current = self;
        for next_char in chars {
            current = current.children.entry(next_char).or_insert(TrieNode::new());
        }
        current
    }

    fn get_child(&self, char: &char) -> Option<&TrieNode<V>> {
        self.children.get(char)
    }

    fn get_child_mut(&mut self, char: &char) -> Option<&mut TrieNode<V>> {
        self.children.get_mut(char)
    }

    fn update_value(&mut self, key: Substring, value: V) -> Option<V> {
        self.value
            .replace(NodeValue { key, value })
            .map(|v| v.value)
    }
}

fn start_search<'a>(text: &'a str) -> Option<(char, Chars<'a>)> {
    let mut chars = text.chars();
    let first_char = chars.next()?;
    Some((first_char, chars))
}

struct TrieIterator<'a, V> {
    stack: Vec<&'a TrieNode<V>>,
}

impl<'a, V> TrieIterator<'a, V> {
    fn new(trie: &'a Trie<V>) -> Self {
        let mut stack = Vec::with_capacity(trie.length);
        stack.extend(trie.nodes.values());
        Self { stack }
    }
}

impl<'a, V> Iterator for TrieIterator<'a, V> {
    type Item = (&'a Substring, &'a V);

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

struct RetainIf<'a, V> {
    stack: Vec<&'a mut TrieNode<V>>,
}

impl<'a, V> RetainIf<'a, V> {
    fn new(trie: &'a mut Trie<V>) -> Self {
        let mut stack = Vec::with_capacity(trie.length);
        stack.extend(trie.nodes.values_mut());
        Self { stack }
    }

    fn run<F>(&mut self, condition: F) -> usize
    where
        F: Fn(&Substring, &V) -> bool,
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
        let mut trie = Trie::new();
        trie.insert("a".into(), 10);

        assert_eq!(1, trie.len());
        assert_contains_substring(&mut trie, "a", 10);
        assert_does_not_contain_substring(&mut trie, "ab");
    }

    #[test]
    fn insert_long_string() {
        let mut trie = Trie::new();
        trie.insert("abcd".into(), 10);

        assert_eq!(1, trie.len());
        assert_does_not_contain_substring(&mut trie, "ab");
        assert_does_not_contain_substring(&mut trie, "abc");
        assert_contains_substring(&mut trie, "abcd", 10);
    }

    #[test]
    fn insert_same_string_twice_replaces_old_value() {
        let mut trie = Trie::new();
        trie.insert("abcd".into(), 10);
        trie.insert("abcd".into(), 20);

        assert_eq!(1, trie.len());
        assert_contains_substring(&mut trie, "abcd", 20);
    }

    #[test]
    fn insert_prefix_of_existing_string() {
        let mut trie = Trie::new();
        trie.insert("abcd".into(), 10);
        trie.insert("abc".into(), 20);

        assert_eq!(2, trie.len());
        assert_contains_substring(&mut trie, "abc", 20);
        assert_contains_substring(&mut trie, "abcd", 10);
    }

    #[test]
    fn insert_different_strings() {
        let mut trie = Trie::new();
        trie.insert("abc".into(), 10);
        trie.insert("def".into(), 20);

        assert_eq!(2, trie.len());
        assert_contains_substring(&mut trie, "abc", 10);
        assert_contains_substring(&mut trie, "def", 20);
    }

    fn assert_contains_substring(trie: &mut Trie<i32>, key: &str, mut value: i32) {
        assert_eq!(Some(&mut value), trie.get_mut(&key.into()));
        assert_eq!(Some(&value), trie.get(&key.into()));
    }

    fn assert_does_not_contain_substring(trie: &mut Trie<i32>, key: &str) {
        assert_eq!(None, trie.get_mut(&key.into()));
        assert_eq!(None, trie.get(&key.into()));
    }
}

#[cfg(test)]
mod find_match_tests {
    use super::*;

    #[test]
    fn find_match_in_empty_trie() {
        let trie: Trie<i32> = Trie::new();
        assert_eq!(None, trie.find_match("abc"));
    }

    #[test]
    fn find_match_for_empty_string() {
        let mut trie = Trie::new();
        trie.insert("abc".into(), 10);
        assert_eq!(None, trie.find_match(""));
    }

    #[test]
    fn find_match_for_substrings() {
        let mut trie = Trie::new();
        trie.insert("abc".into(), 10);
        trie.insert("abcde".into(), 30);

        assert_eq!(None, trie.find_match("ab"));
        assert_eq!(Some((&"abc".into(), &10)), trie.find_match("abcd"));
        assert_eq!(Some((&"abcde".into(), &30)), trie.find_match("abcde"));
    }

    #[test]
    fn finds_longest_possible_match() {
        let mut trie = Trie::new();
        trie.insert("abc".into(), 10);
        trie.insert("abcd".into(), 20);

        let best_match = trie.find_match("abcd");
        assert_eq!(Some((&"abcd".into(), &20)), best_match);
    }

    #[test]
    fn finds_match_in_different_branches() {
        let mut trie = Trie::new();
        trie.insert("abc".into(), 10);
        trie.insert("abcd".into(), 10);
        trie.insert("def".into(), 20);
        trie.insert("abx".into(), 30);

        assert_eq!(Some((&"abc".into(), &10)), trie.find_match("abc"));
        assert_eq!(Some((&"abcd".into(), &10)), trie.find_match("abcde"));
        assert_eq!(Some((&"def".into(), &20)), trie.find_match("def"));
        assert_eq!(Some((&"abx".into(), &30)), trie.find_match("abx"));
        assert_eq!(None, trie.find_match("xyz"));
    }

    #[test]
    fn finds_match_with_extra_characters() {
        let mut trie = Trie::new();
        trie.insert("hello".into(), 10);

        assert_eq!(Some((&"hello".into(), &10)), trie.find_match("hello world"));
        assert_eq!(Some((&"hello".into(), &10)), trie.find_match("hello!"));
    }

    #[test]
    fn finds_match_with_multiple_possibilities() {
        let mut trie = Trie::new();
        trie.insert("a".into(), 10);
        trie.insert("ab".into(), 20);
        trie.insert("abc".into(), 30);
        trie.insert("abcd".into(), 40);

        assert_eq!(Some((&"abcd".into(), &40)), trie.find_match("abcdef"));
        assert_eq!(Some((&"abc".into(), &30)), trie.find_match("abc"));
        assert_eq!(Some((&"ab".into(), &20)), trie.find_match("abxyz"));
        assert_eq!(Some((&"a".into(), &10)), trie.find_match("a"));
    }

    #[test]
    fn finds_match_with_unicode() {
        let mut trie = Trie::new();
        trie.insert("こんにちは".into(), 10);
        trie.insert("世界".into(), 20);

        assert_eq!(
            Some((&"こんにちは".into(), &10)),
            trie.find_match("こんにちは世界")
        );
        assert_eq!(Some((&"世界".into(), &20)), trie.find_match("世界"));
    }
}

#[cfg(test)]
mod iterator_tests {
    use super::*;

    #[test]
    fn empty_trie() {
        let trie: Trie<i32> = Trie::new();
        let strings = trie.iter().collect::<Vec<_>>();
        assert!(strings.is_empty());
    }

    #[test]
    fn iterate_over_entries() {
        let mut trie = Trie::new();
        trie.insert("abc".into(), 10);
        trie.insert("abx".into(), 10);
        trie.insert("abcd".into(), 20);
        trie.insert("abcde".into(), 30);
        trie.insert("def".into(), 40);

        let mut entries = trie.iter().collect::<Vec<_>>();
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

#[cfg(test)]
mod retain_tests {
    use super::*;

    #[test]
    fn retain_entries() {
        let mut trie = Trie::new();
        trie.insert("abc".into(), 10);
        trie.insert("abx".into(), 10);
        trie.insert("abcd".into(), 20);
        trie.insert("xyz".into(), 30);

        trie.retain(|_, count| *count > 10);
        assert_eq!(2, trie.len());
        assert_eq!(vec!["abcd", "xyz"], collect_strings(&trie));
    }

    fn collect_strings(trie: &Trie<i32>) -> Vec<String> {
        let mut strings = trie.iter().map(|(s, _)| s.to_string()).collect::<Vec<_>>();
        strings.sort();
        strings
    }
}
