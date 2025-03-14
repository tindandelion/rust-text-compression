use std::{collections::HashMap, str::Chars};

use crate::encoder::Substring;

pub struct Trie {
    nodes: HashMap<char, TrieNode>,
    length: usize,
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

    pub fn insert(&mut self, substring: Substring, count: usize) {
        if let Some((first_char, rest_chars)) = start_search(substring.as_str()) {
            let root = self.nodes.entry(first_char).or_insert(TrieNode::new());
            let leaf = root.make_children(rest_chars);

            if leaf.update_value(substring, count).is_none() {
                self.length += 1;
            }
        }
    }

    pub fn get_mut(&mut self, substring: &Substring) -> Option<&mut usize> {
        let (first_char, rest_chars) = start_search(substring.as_str())?;

        let mut current = self.nodes.get_mut(&first_char)?;
        for char in rest_chars {
            current = current.get_child_mut(&char)?;
        }
        current.value.as_mut().map(|v| &mut v.value)
    }

    pub fn get(&self, substring: &Substring) -> Option<&usize> {
        let (first_char, rest_chars) = start_search(substring.as_str())?;

        let mut current = self.nodes.get(&first_char)?;
        for char in rest_chars {
            current = current.get_child(&char)?;
        }
        current.value.as_ref().map(|v| &v.value)
    }

    pub fn find_match(&self, text: &str) -> Option<(&Substring, &usize)> {
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

    pub fn iter(&self) -> impl Iterator<Item = (&Substring, &usize)> {
        TrieIterator::new(self)
    }

    pub fn retain<F>(&mut self, f: F)
    where
        F: Fn(&Substring, &usize) -> bool,
    {
        self.length = RetainIf::new(self).run(f);
    }
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

fn start_search<'a>(text: &'a str) -> Option<(char, Chars<'a>)> {
    let mut chars = text.chars();
    let first_char = chars.next()?;
    Some((first_char, chars))
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
