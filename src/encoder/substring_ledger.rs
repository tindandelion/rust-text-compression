use std::{cmp::Ordering, collections::HashMap};

use crate::substring_dictionary::SubstringDictionary;

use super::encoder_spec::EncoderSpec;

pub struct SubstringLedger {
    substrings: HashMap<Substring, u32>,
}

#[derive(PartialEq, Eq, Hash)]
struct Substring(String);

struct EncodingImpact<'a> {
    substring: &'a Substring,
    compression_gain: usize,
}

impl Substring {
    fn new(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl SubstringLedger {
    pub fn new() -> Self {
        Self {
            substrings: HashMap::new(),
        }
    }

    pub fn insert_new(&mut self, str: &str) {
        self.substrings.insert(Substring::new(str), 1);
    }

    pub fn find_longest_match(&self, s: &str) -> Option<String> {
        self.values()
            .iter()
            .find(|&&k| s.starts_with(k))
            .map(|s| s.to_string())
    }

    // TODO: Convert argument to Substring
    pub fn increment_count(&mut self, str: &str) {
        let count = self
            .substrings
            .get_mut(&Substring::new(str))
            .expect(format!("Substring [{}] not found", str).as_str());
        *count += 1;
    }

    pub fn values(&self) -> Vec<&String> {
        let mut keys: Vec<_> = self.substrings.keys().collect();
        keys.sort_by(|a, b| compare_substrings2(a, b));
        keys.into_iter().map(|k| &k.0).collect()
    }

    pub fn get_most_impactful_strings(&self, encoder_spec: &EncoderSpec) -> SubstringDictionary {
        let impacts = self.calculate_impacts(encoder_spec);
        let mut most_impactful: Vec<String> = impacts
            .into_iter()
            .map(|impact| impact.substring.0.clone())
            .take(encoder_spec.num_strings)
            .collect();
        most_impactful.sort_by(|a, b| compare_substrings(a, b));
        SubstringDictionary::new(most_impactful)
    }

    fn calculate_impacts(&self, encoder_spec: &EncoderSpec) -> Vec<EncodingImpact<'_>> {
        let mut impacts: Vec<EncodingImpact> = self
            .substrings
            .iter()
            .map(|(substring, &count)| EncodingImpact {
                substring,
                compression_gain: encoder_spec.compression_gain(&substring.0, count as usize),
            })
            .filter(|impact| impact.compression_gain > 0)
            .collect();
        impacts.sort_by(|a, b| b.compression_gain.cmp(&a.compression_gain));
        impacts
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

fn compare_substrings2(a: &Substring, b: &Substring) -> Ordering {
    let by_length = (b.0.len()).cmp(&a.0.len());
    if by_length.is_eq() {
        a.0.cmp(&b.0)
    } else {
        by_length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_longest_match_when_found() {
        let mut dict = SubstringLedger::new();
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
        let mut dict = SubstringLedger::new();
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
        /*
         * For equal counts, longer strings make bigger impact on compression.
         */
        let mut dict = SubstringLedger::new();
        dict.insert_new("a");
        dict.insert_new("aa");
        dict.insert_new("aaaaa");
        dict.insert_new("b");

        let most_impactful = dict.get_most_impactful_strings(&EncoderSpec {
            num_strings: 1,
            encoded_size: 0,
        });
        assert_eq!(vec!["aaaaa"], most_impactful.to_vec());
    }

    #[test]
    fn most_impactful_substring_found_by_count() {
        /*
         * For equal string lengths, more frequent substrings make bigger impact on compression.
         */
        let mut dict = SubstringLedger::new();
        dict.insert_new("a");

        dict.insert_new("b");
        dict.increment_count("b");

        let most_impactful = dict.get_most_impactful_strings(&EncoderSpec {
            num_strings: 1,
            encoded_size: 0,
        });
        assert_eq!(vec!["b"], most_impactful.to_vec());
    }

    #[test]
    fn most_impactful_substring_found_by_count_and_string_length() {
        /*
         * The string has more impact, when the total length of all its occurrences is bigger.
         */
        let mut dict = SubstringLedger::new();
        dict.insert_new("a");
        dict.insert_new("aaa");

        dict.insert_new("b");
        dict.increment_count("b");
        dict.increment_count("b");
        dict.increment_count("b");

        let most_impactful = dict.get_most_impactful_strings(&EncoderSpec {
            num_strings: 1,
            encoded_size: 0,
        });
        assert_eq!(vec!["b"], most_impactful.to_vec());
    }

    #[test]
    fn most_impactful_substring_removes_short_strings() {
        /*
         * Short strings are not encoded, because their encoded size is bigger than or equal to the string's length itself.
         */
        let mut dict = SubstringLedger::new();

        dict.insert_new("aaa");

        dict.insert_new("a");
        dict.insert_new("aa");
        dict.increment_count("aa");
        dict.increment_count("aa");

        let most_impactful = dict.get_most_impactful_strings(&EncoderSpec {
            num_strings: 1,
            encoded_size: 2,
        });
        assert_eq!(vec!["aaa"], most_impactful.to_vec());
    }

    #[test]
    fn most_impactful_substring_takes_total_encoded_size_into_account() {
        /*
         * A longer, but less frequent string can have more impact on compression,
         * than a shorter, but more frequent string.
         * Consider the following example:
         *
         * "aaaaa" - 1 occurrence, 5 bytes
         * "aaa" - 2 occurrence, 3 bytes
         *
         * When replacing "aaaaa" with its encoded form, we'll replace 1 5-byte string with 2 bytes (gain of 3 bytes).
         * When replacing "aaa" with its encoded form, we'll replace 2 3-bytes string with 2 2-byte encoded versions.
         * In that case, we gain (2 * 3 - 2 * 2) = 2 bytes.
         *
         * So, "aaaaa" has more impact on compression, even though it's less frequent.
         */
        let mut dict = SubstringLedger::new();

        dict.insert_new("aaaaa");

        dict.insert_new("aaa");
        dict.increment_count("aaa");

        let most_impactful = dict.get_most_impactful_strings(&EncoderSpec {
            num_strings: 1,
            encoded_size: 2,
        });
        assert_eq!(vec!["aaaaa"], most_impactful.to_vec());
    }

    #[test]
    fn most_impactful_substrings_ordered_by_length() {
        let mut dict = SubstringLedger::new();
        dict.insert_new("b");
        dict.insert_new("aaaaaa");

        dict.insert_new("aa");
        dict.increment_count("aa");
        dict.increment_count("aa");
        dict.increment_count("aa");

        let most_impactful = dict.get_most_impactful_strings(&EncoderSpec {
            num_strings: 2,
            encoded_size: 1,
        });
        assert_eq!(vec!["aaaaaa", "aa"], most_impactful.to_vec());
    }

    // TODO: Increment count of a non-existing substring
    // TODO: Inserting already existing substring
}
