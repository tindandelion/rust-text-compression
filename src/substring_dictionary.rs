use std::{cmp::Ordering, collections::HashMap};

pub struct SubstringDictionary {
    strings: HashMap<String, u32>,
}

impl SubstringDictionary {
    pub fn new() -> Self {
        Self {
            strings: HashMap::new(),
        }
    }

    pub fn insert_new(&mut self, str: &str) {
        self.strings.insert(str.to_string(), 1);
    }

    pub fn find_longest_match(&self, s: &str) -> Option<String> {
        self.values()
            .iter()
            .find(|&&k| s.starts_with(k))
            .map(|s| s.to_string())
    }

    pub fn increment_count(&mut self, str: &str) {
        let count = self.strings.get_mut(str).unwrap();
        *count += 1;
    }

    pub fn values(&self) -> Vec<&String> {
        let mut keys: Vec<_> = self.strings.keys().collect();
        keys.sort_by(|a, b| compare_substrings(a, b));
        keys
    }
}

fn compare_substrings(a: &str, b: &str) -> Ordering {
    let by_length = (b.len()).cmp(&a.len());
    if by_length == Ordering::Equal {
        a.cmp(b)
    } else {
        by_length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_longest_match_when_found() {
        let mut dict = SubstringDictionary::new();
        dict.insert_new("a");
        dict.insert_new("aa");
        dict.insert_new("aaa");
        dict.insert_new("b");

        let found = dict.find_longest_match("aaa");
        assert_eq!(Some("aaa".to_string()), found);

        let found = dict.find_longest_match("aab");
        assert_eq!(Some("aa".to_string()), found);

        let found = dict.find_longest_match("bba");
        assert_eq!(Some("b".to_string()), found);
    }

    #[test]
    fn find_longest_match_when_not_found() {
        let mut dict = SubstringDictionary::new();
        dict.insert_new("a");
        dict.insert_new("aa");
        dict.insert_new("aaa");
        dict.insert_new("b");

        let found = dict.find_longest_match("ccc");
        assert_eq!(None, found);
    }
}
