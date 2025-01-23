use std::collections::BTreeMap;

use crate::substring_dictionary::SubstringDictionary;

use super::encoder_spec::EncoderSpec;
use super::substring::Substring;

type SubstringMap = BTreeMap<Substring, u32>;

pub struct SubstringLedger {
    substrings: SubstringMap,
    policy: TestLedgerPolicy,
}

struct EncodingImpact {
    substring: Substring,
    compression_gain: usize,
}

struct TestLedgerPolicy {
    max_entries: usize,
}

impl TestLedgerPolicy {
    fn cleanup(&self, substrings: &mut SubstringMap) {
        if substrings.len() < self.max_entries {
            return;
        }
        substrings.retain(|_, count| *count > 1);
    }

    fn should_merge(&self, x: &Substring, y: &Substring, substrings: &SubstringMap) -> bool {
        let x_count = substrings.get(x).unwrap();
        let y_count = substrings.get(y).unwrap();
        return *x_count == 1 && *y_count == 1;
    }
}

impl SubstringLedger {
    pub fn with_policy(policy: TestLedgerPolicy) -> Self {
        Self {
            substrings: BTreeMap::new(),
            policy,
        }
    }

    pub fn new() -> Self {
        Self::with_policy(TestLedgerPolicy { max_entries: 100 })
    }

    pub fn insert_new(&mut self, substr: Substring) {
        self.policy.cleanup(&mut self.substrings);
        self.substrings.insert(substr, 1);
    }

    pub fn should_merge(&self, x: &Substring, y: &Substring) -> bool {
        self.policy.should_merge(x, y, &self.substrings)
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
            .filter(|(_, count)| *count > 1)
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

    #[cfg(test)]
    pub fn entries(&self) -> Vec<(&str, u32)> {
        self.substrings
            .iter()
            .map(|(substring, count)| (substring.as_str(), *count))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod bookkeeping {
        use super::*;

        #[test]
        fn cleanup_ledger_according_to_policy_when_inserting_new_substring() {
            let mut ledger = make_ledger_with_policy(TestLedgerPolicy { max_entries: 3 });

            ledger.insert_new(substring("a"));
            ledger.insert_new(substring("b"));
            ledger.insert_new(substring("x"));
            ledger.increment_count(&substring("x"));
            assert_eq!(vec![("a", 1), ("b", 1), ("x", 2)], ledger.entries());

            ledger.insert_new(substring("y"));
            assert_eq!(vec![("x", 2), ("y", 1)], ledger.entries());
        }

        #[test]
        fn should_merge_substrings_whose_count_is_one() {
            let mut ledger = make_ledger_with_policy(TestLedgerPolicy { max_entries: 10 });
            ledger.insert_new(substring("a"));
            ledger.insert_new(substring("b"));

            ledger.insert_new(substring("c"));
            ledger.increment_count(&substring("c"));

            assert!(ledger.should_merge(&substring("a"), &substring("b")));
            assert!(!ledger.should_merge(&substring("a"), &substring("c")));
            assert!(!ledger.should_merge(&substring("c"), &substring("b")));
        }
    }

    #[test]
    fn find_longest_match_when_found() {
        let mut ledger = make_ledger();
        ledger.insert_new(substring("a"));
        ledger.insert_new(substring("aa"));
        ledger.insert_new(substring("aaa"));
        ledger.insert_new(substring("b"));

        let found = ledger.find_longest_match("aaa");
        assert_eq!(Some(substring("aaa")), found);

        let found = ledger.find_longest_match("aab");
        assert_eq!(Some(substring("aa")), found);

        let found = ledger.find_longest_match("bba");
        assert_eq!(Some(substring("b")), found);
    }

    #[test]
    fn find_longest_match_when_not_found() {
        let mut ledger = make_ledger();
        ledger.insert_new(substring("a"));
        ledger.insert_new(substring("aa"));
        ledger.insert_new(substring("aaa"));
        ledger.insert_new(substring("b"));

        let found = ledger.find_longest_match("ccc");
        assert_eq!(None, found);
    }

    #[test]
    fn most_impactful_substring_found_by_string_length() {
        /*
         * For equal counts, longer strings make bigger impact on compression.
         */
        let mut ledger = make_ledger();
        insert_repeated_substring(&mut ledger, "a");
        insert_repeated_substring(&mut ledger, "aa");
        insert_repeated_substring(&mut ledger, "aaaaa");
        insert_repeated_substring(&mut ledger, "b");

        let most_impactful = ledger.get_most_impactful_strings(&EncoderSpec {
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
        let mut ledger = make_ledger();
        ledger.insert_new(substring("a"));

        ledger.insert_new(substring("b"));
        ledger.increment_count(&substring("b"));

        let most_impactful = ledger.get_most_impactful_strings(&EncoderSpec {
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
        let mut ledger = make_ledger();
        ledger.insert_new(substring("a"));
        ledger.insert_new(substring("aaa"));

        ledger.insert_new(substring("b"));
        ledger.increment_count(&substring("b"));
        ledger.increment_count(&substring("b"));
        ledger.increment_count(&substring("b"));

        let most_impactful = ledger.get_most_impactful_strings(&EncoderSpec {
            num_strings: 1,
            encoded_size: 0,
        });
        assert_eq!(vec!["b"], most_impactful.to_vec());
    }

    #[test]
    fn most_impactful_substring_removes_strings_shorter_than_encoded_representation() {
        /*
         * Short strings are not encoded, because their encoded size is bigger than or equal to the string's length itself.
         */
        let mut ledger = make_ledger();
        insert_repeated_substring(&mut ledger, "aaa");

        insert_repeated_substring(&mut ledger, "a");
        insert_repeated_substring(&mut ledger, "aa");
        ledger.increment_count(&substring("aa"));
        ledger.increment_count(&substring("aa"));

        let most_impactful = ledger.get_most_impactful_strings(&EncoderSpec {
            num_strings: 10,
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
        let mut ledger = make_ledger();

        insert_repeated_substring(&mut ledger, "aaaaa");

        insert_repeated_substring(&mut ledger, "aaa");
        ledger.increment_count(&substring("aaa"));

        let most_impactful = ledger.get_most_impactful_strings(&EncoderSpec {
            num_strings: 1,
            encoded_size: 2,
        });
        assert_eq!(vec!["aaaaa"], most_impactful.to_vec());
    }

    #[test]
    fn most_impactfult_strings_skip_single_occurence() {
        let mut ledger = make_ledger();
        ledger.insert_new(substring("aaaaaa"));

        ledger.insert_new(substring("bb"));
        ledger.increment_count(&substring("bb"));

        let most_impactful = ledger.get_most_impactful_strings(&EncoderSpec {
            num_strings: 10,
            encoded_size: 1,
        });
        assert_eq!(vec!["bb"], most_impactful.to_vec());
    }

    #[test]
    fn most_impactful_substrings_ordered_by_length() {
        let mut ledger = make_ledger();
        insert_repeated_substring(&mut ledger, "b");
        insert_repeated_substring(&mut ledger, "aaaaaa");

        insert_repeated_substring(&mut ledger, "aa");
        ledger.increment_count(&substring("aa"));
        ledger.increment_count(&substring("aa"));
        ledger.increment_count(&substring("aa"));

        let most_impactful = ledger.get_most_impactful_strings(&EncoderSpec {
            num_strings: 2,
            encoded_size: 1,
        });
        assert_eq!(vec!["aaaaaa", "aa"], most_impactful.to_vec());
    }

    fn substring(s: &str) -> Substring {
        Substring(s.to_string())
    }

    fn insert_repeated_substring(ledger: &mut SubstringLedger, s: &str) {
        let substr = substring(s);
        ledger.insert_new(substr.clone());
        ledger.increment_count(&substr);
    }

    fn make_ledger() -> SubstringLedger {
        SubstringLedger::new()
    }

    fn make_ledger_with_policy(policy: TestLedgerPolicy) -> SubstringLedger {
        SubstringLedger::with_policy(policy)
    }

    // TODO: Increment count of a non-existing substring
    // TODO: Inserting already existing substring
}
