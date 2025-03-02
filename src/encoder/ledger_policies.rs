use super::{
    substring::Substring, substring_counts::SubstringCounts, substring_ledger::LedgerPolicy,
};

pub struct CaptureAll;

pub struct LimitLedgerSize {
    max_size: usize,
}

impl LimitLedgerSize {
    pub fn with_max_size(max_size: usize) -> Self {
        Self { max_size }
    }

    fn calc_merge_threshold(&self, counts: &SubstringCounts) -> usize {
        let free_space = self.calc_free_space(counts);
        if free_space <= 0 {
            usize::MAX
        } else {
            self.max_size.div_ceil(free_space)
        }
    }

    fn is_full(&self, counts: &SubstringCounts) -> bool {
        counts.len() >= self.max_size
    }

    fn should_cleanup(&self, counts: &SubstringCounts) -> bool {
        self.calc_free_space(counts) < 2
    }

    fn calc_median_count(&self, counts: &SubstringCounts) -> usize {
        let mut counts = counts.values().collect::<Vec<_>>();
        if counts.len() == 1 {
            return *counts[0];
        }
        counts.sort();
        *counts[counts.len() / 2 - 1]
    }

    fn calc_free_space(&self, counts: &SubstringCounts) -> usize {
        self.max_size - counts.len()
    }
}

impl LedgerPolicy for CaptureAll {
    fn cleanup(&self, _counts: &mut SubstringCounts) {}

    fn should_merge(&self, _x: &Substring, _y: &Substring, _substrings: &SubstringCounts) -> bool {
        true
    }
}

impl LedgerPolicy for LimitLedgerSize {
    fn cleanup(&self, counts: &mut SubstringCounts) {
        if self.should_cleanup(counts) {
            let median_count = self.calc_median_count(counts);
            counts.retain(|_, count| *count >= median_count);
        }
    }

    fn should_merge(&self, x: &Substring, y: &Substring, counts: &SubstringCounts) -> bool {
        if self.is_full(counts) {
            return false;
        }

        let threshold = self.calc_merge_threshold(&counts);
        let x_count = counts.get(x).unwrap();
        let y_count = counts.get(y).unwrap();
        x_count >= threshold && y_count >= threshold
    }
}

#[cfg(test)]
mod limit_dictionary_size_tests {
    use super::*;

    mod merging {

        use super::*;

        #[test]
        fn should_merge_when_both_counts_are_bigger_than_threshold() {
            let x = Substring::from("x");
            let y = Substring::from("y");
            let policy = LimitLedgerSize { max_size: 4 };
            let mut counts = SubstringCounts::new();

            counts.insert(x.clone(), 3);
            counts.insert(y.clone(), 3);

            assert!(policy.should_merge(&x, &y, &counts));
            assert!(policy.should_merge(&y, &x, &counts));
        }

        #[test]
        fn should_merge_when_count_is_equal_to_threshold() {
            let x = Substring::from("x");
            let y = Substring::from("y");
            let policy = LimitLedgerSize { max_size: 4 };
            let mut counts = SubstringCounts::new();

            counts.insert(x.clone(), 2);
            counts.insert(y.clone(), 3);

            assert!(policy.should_merge(&x, &y, &counts));
            assert!(policy.should_merge(&y, &x, &counts));
        }

        #[test]
        fn should_not_merge_when_at_least_one_count_is_less_than_threshold() {
            let x = Substring::from("x");
            let y = Substring::from("y");
            let policy = LimitLedgerSize { max_size: 4 };
            let mut counts = SubstringCounts::new();

            counts.insert(x.clone(), 1);
            counts.insert(y.clone(), 3);

            assert!(!policy.should_merge(&x, &y, &counts));
            assert!(!policy.should_merge(&y, &x, &counts));
        }

        #[test]
        fn should_not_merge_when_dict_is_full() {
            /*
                Do not merge strings when the dictionary is full, regardless of their counts
            */
            let x = Substring::from("x");
            let y = Substring::from("y");
            let policy = LimitLedgerSize { max_size: 2 };
            let mut counts = SubstringCounts::new();

            counts.insert(x.clone(), usize::MAX);
            counts.insert(y.clone(), usize::MAX);

            assert!(!policy.should_merge(&x, &y, &counts));
            assert!(!policy.should_merge(&y, &x, &counts));
        }

        #[test]
        fn should_merge_with_fractional_threshold_rounds_to_upper_threshold_bound() {
            /*
               Given the dictionary of max_size = 7, and current size = 3 (threshold = 1.75)
               we should merge substrings whose counts are at least 2 (1.75 rounded up to 2)
            */
            let x = Substring::from("x");
            let y = Substring::from("y");
            let z = Substring::from("z");
            let policy = LimitLedgerSize { max_size: 7 };
            let mut counts = SubstringCounts::new();

            counts.insert(x.clone(), 3);
            counts.insert(y.clone(), 3);
            counts.insert(z.clone(), 1);

            assert!(policy.should_merge(&x, &y, &counts));
            assert!(policy.should_merge(&y, &x, &counts));

            assert!(!policy.should_merge(&x, &z, &counts));
            assert!(!policy.should_merge(&z, &x, &counts));
        }
    }

    mod cleanup {

        use super::*;

        #[test]
        fn keeps_everything_when_dict_has_enough_space() {
            let x = Substring::from("x");
            let y = Substring::from("y");
            let policy = LimitLedgerSize { max_size: 10 };
            let mut counts = SubstringCounts::new();

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
            let mut counts = SubstringCounts::new();

            counts.insert(Substring::from("a"), 9);
            counts.insert(Substring::from("b"), 1);
            counts.insert(Substring::from("c"), 8);
            counts.insert(Substring::from("x"), 3);
            counts.insert(Substring::from("y"), 1);
            counts.insert(Substring::from("z"), 2);

            policy.cleanup(&mut counts);
            assert_eq!(vec!["a", "c", "x", "z"], get_substrings(counts));
        }

        #[test]
        fn keeps_everything_when_exactly_at_median() {
            let policy = LimitLedgerSize { max_size: 4 };
            let mut counts = SubstringCounts::new();

            // All substrings have count 2, which is the median
            counts.insert(Substring::from("a"), 2);
            counts.insert(Substring::from("b"), 2);
            counts.insert(Substring::from("c"), 2);

            policy.cleanup(&mut counts);
            assert_eq!(vec!["a", "b", "c"], get_substrings(counts));
        }

        #[test]
        fn handles_single_element_dictionary() {
            let policy = LimitLedgerSize { max_size: 2 };
            let mut counts = SubstringCounts::new();

            counts.insert(Substring::from("a"), 1);

            policy.cleanup(&mut counts);
            assert_eq!(vec!["a"], get_substrings(counts));
        }

        #[test]
        fn handles_empty_dictionary() {
            let policy = LimitLedgerSize { max_size: 2 };
            let mut counts = SubstringCounts::new();

            policy.cleanup(&mut counts);
            assert_eq!(counts.len(), 0);
        }

        #[test]
        fn removes_below_median_with_even_number_of_elements() {
            let policy = LimitLedgerSize { max_size: 5 };
            let mut counts = SubstringCounts::new();

            counts.insert(Substring::from("a"), 1);
            counts.insert(Substring::from("b"), 2);
            counts.insert(Substring::from("c"), 3);
            counts.insert(Substring::from("d"), 4);

            policy.cleanup(&mut counts);
            assert_eq!(vec!["b", "c", "d"], get_substrings(counts));
        }

        #[test]
        fn preserves_substrings_at_median_counts() {
            let policy = LimitLedgerSize { max_size: 6 };
            let mut counts = SubstringCounts::new();

            counts.insert(Substring::from("a"), 1);
            counts.insert(Substring::from("b"), 2); // median
            counts.insert(Substring::from("c"), 2); // median
            counts.insert(Substring::from("d"), 2); // median
            counts.insert(Substring::from("e"), 3);

            policy.cleanup(&mut counts);
            assert_eq!(vec!["b", "c", "d", "e"], get_substrings(counts));
        }

        fn get_substrings(substrings: SubstringCounts) -> Vec<String> {
            substrings
                .into_iter()
                .map(|(s, _)| s.to_string())
                .collect::<Vec<_>>()
        }
    }
}
