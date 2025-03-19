use crate::{encoder::Substring, trie::Trie};

#[derive(Clone)]
struct TableEntry {
    substring: Substring,
    hits: std::cell::Cell<usize>,
}

pub struct EncodingTable {
    entries: Vec<TableEntry>,
    search_index: Trie<usize>,
}

impl EncodingTable {
    pub fn new(mut substrings: Vec<Substring>) -> Self {
        substrings.sort();

        let entries = substrings
            .clone()
            .into_iter()
            .map(|s| TableEntry {
                substring: s,
                hits: std::cell::Cell::new(0),
            })
            .collect();

        let mut search_index = Trie::new();
        for (i, substring) in substrings.into_iter().enumerate() {
            search_index.insert(substring, i);
        }

        Self {
            entries,
            search_index,
        }
    }

    pub fn find_match(&self, text: &str) -> Option<(usize, &str)> {
        let (substring, &index) = self.search_index.find_match(text)?;

        let entry = &self.entries[index];
        entry.hits.set(entry.hits.get() + 1);
        Some((index, substring.as_str()))
    }

    pub fn unused_entries(&self) -> Vec<String> {
        self.entries
            .iter()
            .filter(|entry| entry.hits.get() == 0)
            .map(|entry| entry.substring.to_string())
            .collect()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn top(&self, n: usize) -> Vec<String> {
        self.entries[..n]
            .iter()
            .map(|e| e.substring.to_string())
            .collect()
    }

    pub fn bottom(&self, n: usize) -> Vec<String> {
        self.entries[self.entries.len() - n..]
            .iter()
            .map(|e| e.substring.to_string())
            .collect()
    }

    pub fn get(&self, index: usize) -> &str {
        self.entries[index].substring.as_str()
    }

    #[cfg(test)]
    pub fn to_vec(&self) -> Vec<String> {
        self.entries
            .iter()
            .map(|e| e.substring.to_string())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_the_longest_match_possible() {
        let table = EncodingTable::new(vec![
            "a".into(),
            "aa".into(),
            "aaaa".into(),
            "b".into(),
            "bb".into(),
        ]);

        let found = table.find_match("aaaabb");
        assert_eq!(Some((0, "aaaa")), found);
    }

    #[test]
    fn no_match_found() {
        let table = EncodingTable::new(vec!["a".into(), "aa".into()]);
        let miss = table.find_match("bcd");
        assert_eq!(None, miss);
    }

    #[test]
    fn get_substring_at_index() {
        let table = EncodingTable::new(vec!["a".into(), "aaaa".into(), "b".into(), "bb".into()]);

        assert_eq!("aaaa", table.get(0));
        assert_eq!("bb", table.get(1));
    }

    #[test]
    fn sorts_substrings_by_length_at_creation() {
        let table = EncodingTable::new(vec![
            "a".into(),
            "aa".into(),
            "aaa".into(),
            "b".into(),
            "bb".into(),
        ]);
        assert_eq!(vec!["aaa", "aa", "bb", "a", "b"], table.to_vec());
    }

    #[test]
    fn used_entries_count_at_start() {
        let table = EncodingTable::new(vec!["hello".into(), "world".into()]);
        assert_eq!(
            vec!["hello".to_string(), "world".to_string()],
            table.unused_entries()
        );
    }

    #[test]
    fn used_entries_count_after_match() {
        let table = EncodingTable::new(vec!["hello".into(), "world".into()]);

        table.find_match("hello");
        table.find_match("hello");

        assert_eq!(vec!["world".to_string()], table.unused_entries());
    }

    #[test]
    fn used_entries_count_after_no_match() {
        let table = EncodingTable::new(vec!["hello".into(), "world".into()]);

        table.find_match("brave");
        assert_eq!(
            vec!["hello".to_string(), "world".to_string()],
            table.unused_entries()
        );
    }

    #[test]
    fn used_entries_count_match_all() {
        let table = EncodingTable::new(vec!["hello".into(), "world".into()]);

        table.find_match("hello");
        table.find_match("world");
        assert_eq!(vec![] as Vec<String>, table.unused_entries());
    }
}
