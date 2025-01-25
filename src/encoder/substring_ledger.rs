use std::collections::BTreeMap;

use crate::substring_dictionary::SubstringDictionary;

use super::encoder_spec::EncoderSpec;
use super::substring::Substring;

pub type SubstringMap = BTreeMap<Substring, usize>;

pub trait LedgerPolicy {
    fn cleanup(&self, substrings: &mut SubstringMap);
    fn should_merge(&self, x: &Substring, y: &Substring, substrings: &SubstringMap) -> bool;
}

pub struct SubstringLedger<LP: LedgerPolicy> {
    substrings: SubstringMap,
    policy: LP,
}

struct EncodingImpact {
    substring: Substring,
    compression_gain: usize,
}

impl<LP: LedgerPolicy> SubstringLedger<LP> {
    pub fn with_policy(policy: LP) -> Self {
        Self {
            substrings: BTreeMap::new(),
            policy,
        }
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

    pub fn increment_count(&mut self, substr: Substring) {
        let current_count = self.substrings.get_mut(&substr);
        if let Some(count) = current_count {
            *count += 1;
        } else {
            self.policy.cleanup(&mut self.substrings);
            self.substrings.insert(substr, 1);
        }
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

    pub fn contains(&self, substr: &Substring) -> bool {
        self.substrings.contains_key(substr)
    }

    #[cfg(test)]
    pub fn entries(&self) -> Vec<(&str, usize)> {
        self.substrings
            .iter()
            .map(|(substring, count)| (substring.as_str(), *count))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod string_counting {
        use super::*;

        #[test]
        fn initial_increment_count_adds_to_ledger() {
            let mut ledger = make_ledger();

            ledger.increment_count(substring("a"));
            assert_eq!(vec![("a", 1)], ledger.entries());
        }

        #[test]
        fn subsequent_increment_count_updates_count() {
            let mut ledger = make_ledger();

            ledger.increment_count(substring("a"));
            ledger.increment_count(substring("a"));

            assert_eq!(vec![("a", 2)], ledger.entries());
        }
    }

    mod bookkeeping {
        use super::*;

        #[test]
        fn cleanup_ledger_according_to_policy_when_inserting_new_substring() {
            let mut ledger = make_ledger_with_policy(TestLedgerPolicy { max_entries: 3 });

            ledger.increment_count(substring("a"));
            ledger.increment_count(substring("b"));
            ledger.increment_count(substring("x"));
            ledger.increment_count(substring("x"));
            assert_eq!(vec![("a", 1), ("b", 1), ("x", 2)], ledger.entries());

            ledger.increment_count(substring("y"));
            assert_eq!(vec![("x", 2), ("y", 1)], ledger.entries());
        }

        #[test]
        fn should_merge_substrings_whose_count_is_one() {
            let mut ledger = make_ledger_with_policy(TestLedgerPolicy { max_entries: 10 });
            ledger.increment_count(substring("a"));
            ledger.increment_count(substring("b"));

            ledger.increment_count(substring("c"));
            ledger.increment_count(substring("c"));

            assert!(ledger.should_merge(&substring("a"), &substring("b")));
            assert!(!ledger.should_merge(&substring("a"), &substring("c")));
            assert!(!ledger.should_merge(&substring("c"), &substring("b")));
        }

        // TODO: Error handling for trying to merge non-existing substrings
    }

    mod find_longest_match {
        use super::*;

        #[test]
        fn match_found() {
            let mut ledger = make_ledger();
            ledger.increment_count(substring("a"));
            ledger.increment_count(substring("aa"));
            ledger.increment_count(substring("aaa"));
            ledger.increment_count(substring("b"));

            let found = ledger.find_longest_match("aaa");
            assert_eq!(Some(substring("aaa")), found);

            let found = ledger.find_longest_match("aab");
            assert_eq!(Some(substring("aa")), found);

            let found = ledger.find_longest_match("bba");
            assert_eq!(Some(substring("b")), found);
        }

        #[test]
        fn match_not_found() {
            let mut ledger = make_ledger();
            ledger.increment_count(substring("a"));
            ledger.increment_count(substring("aa"));
            ledger.increment_count(substring("aaa"));
            ledger.increment_count(substring("b"));

            let found = ledger.find_longest_match("ccc");
            assert_eq!(None, found);
        }
    }

    mod most_impactful_strings {
        use super::*;

        #[test]
        fn found_by_string_length() {
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
        fn found_by_count() {
            /*
             * For equal string lengths, more frequent substrings make bigger impact on compression.
             */
            let mut ledger = make_ledger();
            ledger.increment_count(substring("a"));

            ledger.increment_count(substring("b"));
            ledger.increment_count(substring("b"));

            let most_impactful = ledger.get_most_impactful_strings(&EncoderSpec {
                num_strings: 1,
                encoded_size: 0,
            });
            assert_eq!(vec!["b"], most_impactful.to_vec());
        }

        #[test]
        fn found_by_count_and_string_length() {
            /*
             * The string has more impact, when the total length of all its occurrences is bigger.
             */
            let mut ledger = make_ledger();
            ledger.increment_count(substring("a"));
            ledger.increment_count(substring("aaa"));

            ledger.increment_count(substring("b"));
            ledger.increment_count(substring("b"));
            ledger.increment_count(substring("b"));
            ledger.increment_count(substring("b"));

            let most_impactful = ledger.get_most_impactful_strings(&EncoderSpec {
                num_strings: 1,
                encoded_size: 0,
            });
            assert_eq!(vec!["b"], most_impactful.to_vec());
        }

        #[test]
        fn removes_strings_shorter_than_encoded_representation() {
            /*
             * Short strings are not encoded, because their encoded size is bigger than or equal to the string's length itself.
             */
            let mut ledger = make_ledger();
            insert_repeated_substring(&mut ledger, "aaa");

            insert_repeated_substring(&mut ledger, "a");
            insert_repeated_substring(&mut ledger, "aa");
            ledger.increment_count(substring("aa"));
            ledger.increment_count(substring("aa"));

            let most_impactful = ledger.get_most_impactful_strings(&EncoderSpec {
                num_strings: 10,
                encoded_size: 2,
            });
            assert_eq!(vec!["aaa"], most_impactful.to_vec());
        }

        #[test]
        fn takes_total_encoded_size_into_account() {
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
            ledger.increment_count(substring("aaa"));

            let most_impactful = ledger.get_most_impactful_strings(&EncoderSpec {
                num_strings: 1,
                encoded_size: 2,
            });
            assert_eq!(vec!["aaaaa"], most_impactful.to_vec());
        }

        #[test]
        fn skip_single_occurrence() {
            let mut ledger = make_ledger();
            ledger.increment_count(substring("aaaaaa"));

            ledger.increment_count(substring("bb"));
            ledger.increment_count(substring("bb"));

            let most_impactful = ledger.get_most_impactful_strings(&EncoderSpec {
                num_strings: 10,
                encoded_size: 1,
            });
            assert_eq!(vec!["bb"], most_impactful.to_vec());
        }

        #[test]
        fn ordered_by_length() {
            let mut ledger = make_ledger();
            insert_repeated_substring(&mut ledger, "b");
            insert_repeated_substring(&mut ledger, "aaaaaa");

            insert_repeated_substring(&mut ledger, "aa");
            ledger.increment_count(substring("aa"));
            ledger.increment_count(substring("aa"));
            ledger.increment_count(substring("aa"));

            let most_impactful = ledger.get_most_impactful_strings(&EncoderSpec {
                num_strings: 2,
                encoded_size: 1,
            });
            assert_eq!(vec!["aaaaaa", "aa"], most_impactful.to_vec());
        }
    }

    fn substring(s: &str) -> Substring {
        Substring(s.to_string())
    }

    fn insert_repeated_substring<LP: LedgerPolicy>(ledger: &mut SubstringLedger<LP>, s: &str) {
        let substr = substring(s);
        ledger.increment_count(substr.clone());
        ledger.increment_count(substr);
    }

    fn make_ledger() -> SubstringLedger<TestLedgerPolicy> {
        make_capped_ledger(usize::MAX)
    }

    fn make_capped_ledger(max_entries: usize) -> SubstringLedger<TestLedgerPolicy> {
        SubstringLedger::with_policy(TestLedgerPolicy { max_entries })
    }

    fn make_ledger_with_policy(policy: TestLedgerPolicy) -> SubstringLedger<TestLedgerPolicy> {
        SubstringLedger::with_policy(policy)
    }

    struct TestLedgerPolicy {
        max_entries: usize,
    }

    impl LedgerPolicy for TestLedgerPolicy {
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
}
