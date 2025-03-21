use super::{Substring, Trie};

pub struct EncodingTable {
    entries: Vec<Substring>,
    search_index: Trie<usize>,
}

impl EncodingTable {
    pub fn new(substrings: Vec<Substring>) -> Self {
        let entries = substrings.clone().into_iter().collect();

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
        Some((index, substring.as_str()))
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn get(&self, index: usize) -> &str {
        self.entries[index].as_str()
    }

    #[cfg(test)]
    pub fn to_vec(&self) -> Vec<String> {
        self.entries.iter().map(|e| e.to_string()).collect()
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
        assert_eq!(Some((2, "aaaa")), found);
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

        assert_eq!("aaaa", table.get(1));
        assert_eq!("bb", table.get(3));
    }
}
