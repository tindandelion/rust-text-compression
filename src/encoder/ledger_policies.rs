use super::{substring_counts::SubstringCounts, substring_ledger::LedgerPolicy};

pub struct CaptureAll;

pub struct LimitLedgerSize {
    max_size: usize,
}

impl LimitLedgerSize {
    pub fn with_max_size(max_size: usize) -> Self {
        Self { max_size }
    }

    fn calc_merge_threshold(&self, counts: &impl SubstringCounts) -> usize {
        let free_space = self.calc_free_space(counts);
        if free_space <= 0 {
            usize::MAX
        } else {
            self.max_size.div_ceil(free_space)
        }
    }

    fn is_full(&self, counts: &impl SubstringCounts) -> bool {
        counts.len() >= self.max_size
    }

    fn should_cleanup(&self, counts: &impl SubstringCounts) -> bool {
        self.calc_free_space(counts) < 2
    }

    fn calc_median_count(&self, counts: &impl SubstringCounts) -> usize {
        let mut counts = counts.iter().map(|(_, count)| count).collect::<Vec<_>>();
        if counts.len() == 1 {
            return counts[0];
        }
        counts.sort();
        counts[counts.len() / 2 - 1]
    }

    fn calc_free_space(&self, counts: &impl SubstringCounts) -> usize {
        self.max_size - counts.len()
    }
}

impl LedgerPolicy for CaptureAll {
    fn cleanup(&self, _counts: &mut impl SubstringCounts) {}

    fn should_merge(
        &self,
        _x_count: usize,
        _y_count: usize,
        _substrings: &impl SubstringCounts,
    ) -> bool {
        true
    }
}

impl LedgerPolicy for LimitLedgerSize {
    fn cleanup(&self, counts: &mut impl SubstringCounts) {
        if self.should_cleanup(counts) {
            let median_count = self.calc_median_count(counts);
            counts.retain(|_, count| count >= median_count);
        }
    }

    fn should_merge(&self, x_count: usize, y_count: usize, counts: &impl SubstringCounts) -> bool {
        if self.is_full(counts) {
            return false;
        }

        let threshold = self.calc_merge_threshold(counts);
        x_count >= threshold && y_count >= threshold
    }
}

#[cfg(test)]
mod limit_dictionary_size_tests {
    use super::*;

    mod merging {

        use crate::encoder::substring_counts;

        use super::*;

        #[test]
        fn should_merge_when_both_counts_are_bigger_than_threshold() {
            let policy = LimitLedgerSize { max_size: 4 };
            let mut counts = substring_counts::default();
            counts.insert("x".into(), 1);
            counts.insert("y".into(), 10);

            assert!(policy.should_merge(3, 3, &counts));
        }

        #[test]
        fn should_merge_when_count_is_equal_to_threshold() {
            let policy = LimitLedgerSize { max_size: 4 };
            let mut counts = substring_counts::default();
            counts.insert("x".into(), 1);
            counts.insert("y".into(), 10);

            assert!(policy.should_merge(2, 3, &counts));
            assert!(policy.should_merge(3, 2, &counts));
        }

        #[test]
        fn should_not_merge_when_at_least_one_count_is_less_than_threshold() {
            let policy = LimitLedgerSize { max_size: 4 };
            let mut counts = substring_counts::default();
            counts.insert("x".into(), 1);
            counts.insert("y".into(), 10);

            assert!(!policy.should_merge(1, 3, &counts));
            assert!(!policy.should_merge(3, 1, &counts));
        }

        #[test]
        fn should_not_merge_when_dict_is_full() {
            /*
                Do not merge strings when the dictionary is full, regardless of their counts
            */
            let policy = LimitLedgerSize { max_size: 2 };
            let mut counts = substring_counts::default();

            counts.insert("x".into(), 1);
            counts.insert("y".into(), 100);

            assert!(!policy.should_merge(usize::MAX, usize::MAX, &counts));
        }

        #[test]
        fn should_merge_with_fractional_threshold_rounds_to_upper_threshold_bound() {
            /*
               Given the dictionary of max_size = 7, and current size = 3 (threshold = 1.75)
               we should merge substrings whose counts are at least 2 (1.75 rounded up to 2)
            */
            let policy = LimitLedgerSize { max_size: 7 };
            let mut counts = substring_counts::default();
            counts.insert("x".into(), 1);
            counts.insert("y".into(), 2);
            counts.insert("z".into(), 3);

            assert!(policy.should_merge(3, 3, &counts));
            assert!(!policy.should_merge(3, 1, &counts));
            assert!(!policy.should_merge(1, 3, &counts));
        }
    }

    mod cleanup {

        use crate::encoder::{substring_counts, Substring};

        use super::*;

        #[test]
        fn keeps_everything_when_dict_has_enough_space() {
            let x = Substring::from("x");
            let y = Substring::from("y");
            let policy = LimitLedgerSize { max_size: 10 };
            let mut counts = substring_counts::default();

            counts.insert(x.clone(), 1);
            counts.insert(y.clone(), 2);
            counts.insert(y.clone(), 3);

            policy.cleanup(&mut counts);
            assert!(counts.contains_key(&x));
            assert!(counts.contains_key(&y));
        }

        #[test]
        fn removes_all_substrings_when_not_enough_space() {
            /*
               When the there are less then 2 free slots, we should
               remove the substrings whose counts are less than median
            */
            let policy = LimitLedgerSize { max_size: 6 };
            let mut counts = substring_counts::default();

            counts.insert("a".into(), 9);
            counts.insert("b".into(), 1);
            counts.insert("c".into(), 8);
            counts.insert("x".into(), 3);
            counts.insert("y".into(), 1);
            counts.insert("z".into(), 2);

            policy.cleanup(&mut counts);
            assert_eq!(vec!["a", "c", "x", "z"], get_substrings(&counts));
        }

        #[test]
        fn keeps_everything_when_exactly_at_median() {
            let policy = LimitLedgerSize { max_size: 4 };
            let mut counts = substring_counts::default();

            // All substrings have count 2, which is the median
            counts.insert("a".into(), 2);
            counts.insert("b".into(), 2);
            counts.insert("c".into(), 2);

            policy.cleanup(&mut counts);
            assert_eq!(vec!["a", "b", "c"], get_substrings(&counts));
        }

        #[test]
        fn handles_single_element_dictionary() {
            let policy = LimitLedgerSize { max_size: 2 };
            let mut counts = substring_counts::default();

            counts.insert("a".into(), 1);

            policy.cleanup(&mut counts);
            assert_eq!(vec!["a"], get_substrings(&counts));
        }

        #[test]
        fn handles_empty_dictionary() {
            let policy = LimitLedgerSize { max_size: 2 };
            let mut counts = substring_counts::default();

            policy.cleanup(&mut counts);
            assert_eq!(counts.len(), 0);
        }

        #[test]
        fn removes_below_median_with_even_number_of_elements() {
            let policy = LimitLedgerSize { max_size: 5 };
            let mut counts = substring_counts::default();

            counts.insert("a".into(), 1);
            counts.insert("b".into(), 2);
            counts.insert("c".into(), 3);
            counts.insert("d".into(), 4);

            policy.cleanup(&mut counts);
            assert_eq!(vec!["b", "c", "d"], get_substrings(&counts));
        }

        #[test]
        fn preserves_substrings_at_median_counts() {
            let policy = LimitLedgerSize { max_size: 6 };
            let mut counts = substring_counts::default();

            counts.insert("a".into(), 1);
            counts.insert("b".into(), 2); // median
            counts.insert("c".into(), 2); // median
            counts.insert("d".into(), 2); // median
            counts.insert("e".into(), 3);

            policy.cleanup(&mut counts);
            assert_eq!(vec!["b", "c", "d", "e"], get_substrings(&counts));
        }

        fn get_substrings(substrings: &impl SubstringCounts) -> Vec<String> {
            substring_counts::util::get_sorted_counts(substrings)
                .iter()
                .map(|(s, _)| s.to_string())
                .collect::<Vec<_>>()
        }
    }
}
