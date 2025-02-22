use std::collections::BTreeMap;

use crate::encoding_table::EncodingTable;

use super::substring::Substring;

pub type SubstringMap = BTreeMap<Substring, usize>;

pub trait LedgerPolicy {
    fn should_merge(&self, x: &Substring, y: &Substring, substrings: &SubstringMap) -> bool;
    fn cleanup(&self, substrings: &mut SubstringMap);
}

pub trait SubstringSelector {
    fn select_substrings(&self, substrings: SubstringMap) -> Vec<Substring>;
}

pub struct SubstringLedger<LP: LedgerPolicy> {
    substrings: SubstringMap,
    policy: LP,
}

impl<LP: LedgerPolicy> SubstringLedger<LP> {
    pub fn with_policy(policy: LP) -> Self {
        Self {
            substrings: BTreeMap::new(),
            policy,
        }
    }

    pub fn len(&self) -> usize {
        self.substrings.len()
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

    pub fn build_encoding_table(
        mut self,
        selector: &impl SubstringSelector,
        capacity: usize,
    ) -> EncodingTable {
        self.substrings.retain(|_, count| *count > 1);
        let mut most_impactful = selector.select_substrings(self.substrings);
        most_impactful.sort();
        most_impactful.truncate(capacity);
        EncodingTable::new(most_impactful.into_iter().map(|s| s.0).collect())
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

    mod build_encoding_table {
        use super::*;
        use crate::{
            encoder::encoder_spec::EncoderSpec, substring_selectors::SelectByCompressionGain,
        };

        #[test]
        fn skip_single_occurrence() {
            let mut ledger = make_ledger();
            ledger.increment_count(substring("aaaaaa"));
            ledger.increment_count(substring("bb"));
            ledger.increment_count(substring("bb"));

            let encoder_spec = EncoderSpec {
                num_strings: 10,
                encoded_size: 1,
            };
            let most_impactful = ledger.build_encoding_table(&make_selector(&encoder_spec), 10);
            assert_eq!(vec!["bb"], most_impactful.to_vec());
        }

        #[test]
        fn order_substrings_by_length() {
            let mut ledger = make_ledger();
            ledger.increment_count(substring("b"));
            ledger.increment_count(substring("b"));

            ledger.increment_count(substring("aaaaaa"));
            ledger.increment_count(substring("aaaaaa"));

            ledger.increment_count(substring("aa"));
            ledger.increment_count(substring("aa"));
            ledger.increment_count(substring("aa"));
            ledger.increment_count(substring("aa"));
            ledger.increment_count(substring("aa"));

            let encoder_spec = EncoderSpec {
                num_strings: 3,
                encoded_size: 0,
            };
            let most_impactful = ledger.build_encoding_table(&make_selector(&encoder_spec), 3);
            assert_eq!(vec!["aaaaaa", "aa", "b"], most_impactful.to_vec());
        }

        #[test]
        fn limit_number_of_entries_by_capacity() {
            let mut ledger = make_ledger();
            ledger.increment_count(substring("b"));
            ledger.increment_count(substring("b"));

            ledger.increment_count(substring("aaaaaa"));
            ledger.increment_count(substring("aaaaaa"));

            ledger.increment_count(substring("aa"));
            ledger.increment_count(substring("aa"));

            let encoder_spec = EncoderSpec {
                num_strings: 3,
                encoded_size: 0,
            };
            let most_impactful = ledger.build_encoding_table(&make_selector(&encoder_spec), 2);
            assert_eq!(vec!["aaaaaa", "aa"], most_impactful.to_vec());
        }

        fn make_selector(encoder_spec: &EncoderSpec) -> SelectByCompressionGain {
            SelectByCompressionGain::new(encoder_spec.encoded_size)
        }
    }

    fn substring(s: &str) -> Substring {
        Substring(s.to_string())
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
