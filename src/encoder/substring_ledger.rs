use std::collections::BTreeMap;

use crate::substring_dictionary::SubstringDictionary;

use super::encoder_spec::EncoderSpec;
use super::substring::Substring;

pub struct SubstringLedger {
    substrings: BTreeMap<Substring, u32>,
}

struct EncodingImpact {
    substring: Substring,
    compression_gain: usize,
}

impl SubstringLedger {
    pub fn new() -> Self {
        Self {
            substrings: BTreeMap::new(),
        }
    }

    pub fn insert_new(&mut self, substr: Substring) {
        self.substrings.insert(substr, 1);
    }

    // TODO: Convert to Option<&Substring>
    pub fn find_longest_match(&self, text: &str) -> Option<Substring> {
        self.substrings
            .keys()
            .find(|&substr| substr.matches_start(text))
            .map(|substr| substr.clone())
    }

    pub fn increment_count(&mut self, substr: &Substring) {
        let count = self
            .substrings
            .get_mut(substr)
            .expect(format!("Substring [{}] not found", substr.to_string()).as_str());
        *count += 1;
    }

    pub fn get_most_impactful_strings(self, encoder_spec: &EncoderSpec) -> SubstringDictionary {
        let impacts = self.calculate_impacts(encoder_spec);
        let mut most_impactful: Vec<Substring> = impacts
            .into_iter()
            .map(|impact| impact.substring)
            .take(encoder_spec.num_strings)
            .collect();
        most_impactful.sort();
        SubstringDictionary::new(most_impactful.into_iter().map(|s| s.0).collect())
    }

    fn calculate_impacts(self, encoder_spec: &EncoderSpec) -> Vec<EncodingImpact> {
        let mut impacts: Vec<EncodingImpact> = self
            .substrings
            .into_iter()
            .map(|(substring, count)| {
                let compression_gain = encoder_spec.compression_gain(&substring.0, count as usize);
                EncodingImpact {
                    substring,
                    compression_gain,
                }
            })
            .filter(|impact| impact.compression_gain > 0)
            .collect();
        impacts.sort_by(|a, b| b.compression_gain.cmp(&a.compression_gain));
        impacts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_longest_match_when_found() {
        let mut dict = SubstringLedger::new();
        dict.insert_new(substring("a"));
        dict.insert_new(substring("aa"));
        dict.insert_new(substring("aaa"));
        dict.insert_new(substring("b"));

        let found = dict.find_longest_match("aaa");
        assert_eq!(Some(substring("aaa")), found);

        let found = dict.find_longest_match("aab");
        assert_eq!(Some(substring("aa")), found);

        let found = dict.find_longest_match("bba");
        assert_eq!(Some(substring("b")), found);
    }

    #[test]
    fn find_longest_match_when_not_found() {
        let mut dict = SubstringLedger::new();
        dict.insert_new(substring("a"));
        dict.insert_new(substring("aa"));
        dict.insert_new(substring("aaa"));
        dict.insert_new(substring("b"));

        let found = dict.find_longest_match("ccc");
        assert_eq!(None, found);
    }

    #[test]
    fn most_impactful_substring_found_by_string_length() {
        /*
         * For equal counts, longer strings make bigger impact on compression.
         */
        let mut dict = SubstringLedger::new();
        dict.insert_new(substring("a"));
        dict.insert_new(substring("aa"));
        dict.insert_new(substring("aaaaa"));
        dict.insert_new(substring("b"));

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
        dict.insert_new(substring("a"));

        dict.insert_new(substring("b"));
        dict.increment_count(&substring("b"));

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
        dict.insert_new(substring("a"));
        dict.insert_new(substring("aaa"));

        dict.insert_new(substring("b"));
        dict.increment_count(&substring("b"));
        dict.increment_count(&substring("b"));
        dict.increment_count(&substring("b"));

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

        dict.insert_new(substring("aaa"));

        dict.insert_new(substring("a"));
        dict.insert_new(substring("aa"));
        dict.increment_count(&substring("aa"));
        dict.increment_count(&substring("aa"));

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

        dict.insert_new(substring("aaaaa"));

        dict.insert_new(substring("aaa"));
        dict.increment_count(&substring("aaa"));

        let most_impactful = dict.get_most_impactful_strings(&EncoderSpec {
            num_strings: 1,
            encoded_size: 2,
        });
        assert_eq!(vec!["aaaaa"], most_impactful.to_vec());
    }

    #[test]
    fn most_impactful_substrings_ordered_by_length() {
        let mut dict = SubstringLedger::new();
        dict.insert_new(substring("b"));
        dict.insert_new(substring("aaaaaa"));

        dict.insert_new(substring("aa"));
        dict.increment_count(&substring("aa"));
        dict.increment_count(&substring("aa"));
        dict.increment_count(&substring("aa"));

        let most_impactful = dict.get_most_impactful_strings(&EncoderSpec {
            num_strings: 2,
            encoded_size: 1,
        });
        assert_eq!(vec!["aaaaaa", "aa"], most_impactful.to_vec());
    }

    fn substring(s: &str) -> Substring {
        Substring(s.to_string())
    }

    // TODO: Increment count of a non-existing substring
    // TODO: Inserting already existing substring
}
