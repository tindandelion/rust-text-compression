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
        let count = self
            .strings
            .get_mut(str)
            .expect(format!("Substring [{}] not found", str).as_str());
        *count += 1;
    }

    pub fn values(&self) -> Vec<&String> {
        let mut keys: Vec<_> = self.strings.keys().collect();
        keys.sort_by(|a, b| compare_substrings(a, b));
        keys
    }

    pub fn get_most_impactful_strings(&self, max_size: usize) -> Vec<&String> {
        struct Impact<'a> {
            string: &'a String,
            total_size: usize,
        }

        let mut x: Vec<Impact> = self
            .strings
            .iter()
            .map(|(string, &count)| {
                let total_size = string.len() * (count as usize);
                Impact { string, total_size }
            })
            .collect();
        x.sort_by(|a, b| b.total_size.cmp(&a.total_size));

        let mut most_impactful: Vec<&String> = x
            .into_iter()
            .map(|impact| impact.string)
            .take(max_size)
            .collect();
        most_impactful.sort_by(|a, b| compare_substrings(a, b));
        most_impactful
    }
}

fn compare_substrings(a: &str, b: &str) -> Ordering {
    let by_length = (b.len()).cmp(&a.len());
    if by_length.is_eq() {
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

    // I have a dictionary of substrings and their counts.
    // I want to find the substring, excluding which I'd gain the maximum compression gain

    #[test]
    fn most_impactful_substring_found_by_string_length() {
        let mut dict = SubstringDictionary::new();
        dict.insert_new("a");
        dict.insert_new("aa");
        dict.insert_new("aaaaa");
        dict.insert_new("b");

        let most_impactful = dict.get_most_impactful_strings(1);
        assert_eq!(vec!["aaaaa"], most_impactful);
    }

    #[test]
    fn most_impactful_substring_found_by_count() {
        let mut dict = SubstringDictionary::new();
        dict.insert_new("a");
        dict.insert_new("b");
        dict.increment_count("b");

        let most_impactful = dict.get_most_impactful_strings(1);
        assert_eq!(vec!["b"], most_impactful);
    }

    #[test]
    fn most_impactful_substring_found_by_count_and_string_length() {
        let mut dict = SubstringDictionary::new();
        dict.insert_new("a");
        dict.insert_new("aaa");

        dict.insert_new("b");
        dict.increment_count("b");
        dict.increment_count("b");
        dict.increment_count("b");

        let most_impactful = dict.get_most_impactful_strings(1);
        assert_eq!(vec!["b"], most_impactful);
    }

    #[test]
    fn most_impactful_substrings_ordered_by_length() {
        let mut dict = SubstringDictionary::new();
        dict.insert_new("b");
        dict.insert_new("aaaaaa");

        dict.insert_new("aa");
        dict.increment_count("aa");
        dict.increment_count("aa");
        dict.increment_count("aa");

        let most_impactful = dict.get_most_impactful_strings(2);
        assert_eq!(vec!["aaaaaa", "aa"], most_impactful);
    }

    // TODO: Increment count of a non-existing substring
}
