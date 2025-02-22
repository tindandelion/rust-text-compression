use crate::encoder::Substring;

#[derive(Clone)]
struct TableEntry {
    substring: Substring,
    hits: std::cell::Cell<usize>,
}

pub struct EncodingTable {
    entries: Vec<TableEntry>,
}

impl EncodingTable {
    pub fn new(substrings: Vec<Substring>) -> Self {
        let entries = substrings
            .into_iter()
            .map(|s| TableEntry {
                substring: s,
                hits: std::cell::Cell::new(0),
            })
            .collect();
        Self { entries }
    }

    pub fn find_match(&self, text: &str) -> Option<(usize, &str)> {
        let result = self
            .entries
            .iter()
            .enumerate()
            .find(|(_, entry)| entry.substring.matches_start(text));

        if let Some((_, entry)) = result {
            entry.hits.set(entry.hits.get() + 1);
        }

        result.map(|(i, entry)| (i, entry.substring.as_str()))
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
