use std::collections::HashMap;

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
        let mut chars = substring.chars();

        if let Some(first_char) = chars.next() {
            let root = self.nodes.entry(first_char).or_insert(TrieNode::new());
            let leaf = root.make_children(chars);

            let old_count = leaf.count;
            leaf.count = count;
            if old_count == 0 {
                self.length += 1;
            }
        }
    }

    pub fn get(&self, substring: &str) -> Option<&usize> {
        let mut chars = substring.chars();

        let first_char = chars.next()?;
        let root = self.nodes.get(&first_char)?;
        root.traverse_children(chars)
            .map(|node| &node.count)
            .filter(|&count| *count > 0)
    }
}

impl TrieNode {
    fn new() -> Self {
        Self {
            count: 0,
            children: HashMap::new(),
        }
    }

    fn traverse_children(&self, chars: impl Iterator<Item = char>) -> Option<&TrieNode> {
        let mut current = self;
        for char in chars {
            current = current.children.get(&char)?;
        }
        Some(current)
    }

    fn make_children(&mut self, chars: impl Iterator<Item = char>) -> &mut TrieNode {
        let mut current = self;
        for next_char in chars {
            current = current.children.entry(next_char).or_insert(TrieNode::new());
        }
        current
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_empty_string() {
        let mut counts = TrieSubstringCounts::new();
        counts.insert("", 10);

        assert_eq!(0, counts.len());
        assert_eq!(None, counts.get(""));
    }

    #[test]
    fn insert_single_char() {
        let mut counts = TrieSubstringCounts::new();
        counts.insert("a", 10);

        assert_eq!(1, counts.len());
        assert_eq!(Some(&10), counts.get("a"));
        assert_eq!(None, counts.get("ab"));
    }

    #[test]
    fn insert_long_string() {
        let mut counts = TrieSubstringCounts::new();
        counts.insert("abcd", 10);

        assert_eq!(1, counts.len());
        assert_eq!(None, counts.get("ab"));
        assert_eq!(None, counts.get("abc"));
        assert_eq!(Some(&10), counts.get("abcd"));
    }

    #[test]
    fn insert_same_string_twice_replaces_old_value() {
        let mut counts = TrieSubstringCounts::new();
        counts.insert("abcd", 10);
        counts.insert("abcd", 20);

        assert_eq!(1, counts.len());
        assert_eq!(Some(&20), counts.get("abcd"));
    }

    #[test]
    fn insert_prefix_of_existing_string() {
        let mut counts = TrieSubstringCounts::new();
        counts.insert("abcd", 10);
        counts.insert("abc", 20);

        assert_eq!(2, counts.len());
        assert_eq!(Some(&20), counts.get("abc"));
        assert_eq!(Some(&10), counts.get("abcd"));
    }

    #[test]
    fn insert_different_strings() {
        let mut counts = TrieSubstringCounts::new();
        counts.insert("abc", 10);
        counts.insert("def", 20);

        assert_eq!(2, counts.len());
        assert_eq!(Some(&10), counts.get("abc"));
        assert_eq!(Some(&20), counts.get("def"));
    }
}
