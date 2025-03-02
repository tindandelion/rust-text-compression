use crate::encoding_table::EncodingTable;

use super::{
    substring::Substring, substring_counts::SubstringCounts, substring_selector::SubstringSelector,
};

pub trait LedgerPolicy {
    fn should_merge(&self, x_count: usize, y_count: usize, substrings: &SubstringCounts) -> bool;
    fn cleanup(&self, substrings: &mut SubstringCounts);
}

pub struct SubstringLedger<LP: LedgerPolicy> {
    substrings: SubstringCounts,
    policy: LP,
}

impl<LP: LedgerPolicy> SubstringLedger<LP> {
    pub fn with_policy(policy: LP) -> Self {
        Self {
            substrings: SubstringCounts::new(),
            policy,
        }
    }

    pub fn len(&self) -> usize {
        self.substrings.len()
    }

    pub fn should_merge(&self, x: &Substring, y: &Substring) -> bool {
        let x_count = self.substrings.get(x).unwrap();
        let y_count = self.substrings.get(y).unwrap();
        self.policy.should_merge(x_count, y_count, &self.substrings)
    }

    // TODO: Convert to Option<&Substring>
    pub fn find_longest_match(&self, text: &str) -> Option<Substring> {
        self.substrings.find_match(text).cloned()
    }

    pub fn increment_count(&mut self, substr: &Substring) {
        let current_count = self.substrings.get_mut(substr);
        if let Some(count) = current_count {
            *count += 1;
        } else {
            self.policy.cleanup(&mut self.substrings);
            self.substrings.insert(substr.clone(), 1);
        }
    }

    pub fn build_encoding_table(self, selector: &SubstringSelector) -> EncodingTable {
        let most_impactful = selector.select_substrings(self.substrings.into_iter());
        EncodingTable::new(most_impactful)
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

            ledger.increment_count(&substring("a"));
            assert_eq!(vec![("a", 1)], ledger.entries());
        }

        #[test]
        fn subsequent_increment_count_updates_count() {
            let mut ledger = make_ledger();

            ledger.increment_count(&substring("a"));
            ledger.increment_count(&substring("a"));

            assert_eq!(vec![("a", 2)], ledger.entries());
        }
    }

    mod bookkeeping {
        use super::*;

        #[test]
        fn cleanup_ledger_according_to_policy_when_inserting_new_substring() {
            let mut ledger = make_ledger_with_policy(TestLedgerPolicy { max_entries: 3 });

            ledger.increment_count(&substring("a"));
            ledger.increment_count(&substring("b"));
            ledger.increment_count(&substring("x"));
            ledger.increment_count(&substring("x"));
            assert_eq!(vec![("a", 1), ("b", 1), ("x", 2)], ledger.entries());

            ledger.increment_count(&substring("y"));
            assert_eq!(vec![("x", 2), ("y", 1)], ledger.entries());
        }

        #[test]
        fn should_merge_substrings_whose_count_is_one() {
            let mut ledger = make_ledger_with_policy(TestLedgerPolicy { max_entries: 10 });
            ledger.increment_count(&substring("a"));
            ledger.increment_count(&substring("b"));

            ledger.increment_count(&substring("c"));
            ledger.increment_count(&substring("c"));

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
            ledger.increment_count(&substring("a"));
            ledger.increment_count(&substring("aa"));
            ledger.increment_count(&substring("aaa"));
            ledger.increment_count(&substring("b"));

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
            ledger.increment_count(&substring("a"));
            ledger.increment_count(&substring("aa"));
            ledger.increment_count(&substring("aaa"));
            ledger.increment_count(&substring("b"));

            let found = ledger.find_longest_match("ccc");
            assert_eq!(None, found);
        }
    }

    fn substring(s: &str) -> Substring {
        Substring::from(s)
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
        fn cleanup(&self, counts: &mut SubstringCounts) {
            if counts.len() < self.max_entries {
                return;
            }
            counts.retain(|_, count| *count > 1);
        }

        fn should_merge(&self, x_count: usize, y_count: usize, _counts: &SubstringCounts) -> bool {
            return x_count == 1 && y_count == 1;
        }
    }
}
