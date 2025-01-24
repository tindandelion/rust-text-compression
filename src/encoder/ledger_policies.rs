use super::{
    substring::Substring,
    substring_ledger::{LedgerPolicy, SubstringMap},
};

pub struct CaptureAll;

pub struct LimitDictionarySize {
    max_size: usize,
}

impl LimitDictionarySize {
    fn calc_merge_threshold(&self, substrings: &SubstringMap) -> usize {
        let free_space = self.calc_free_space(substrings);
        if free_space <= 0 {
            usize::MAX
        } else {
            self.max_size.div_ceil(free_space)
        }
    }

    fn is_full(&self, substrings: &SubstringMap) -> bool {
        substrings.len() >= self.max_size
    }

    fn should_cleanup(&self, substrings: &SubstringMap) -> bool {
        self.calc_free_space(substrings) < 2
    }

    fn calc_median_count(&self, substrings: &SubstringMap) -> usize {
        let mut counts = substrings.values().cloned().collect::<Vec<_>>();
        if counts.len() == 1 {
            return counts[0];
        }
        counts.sort();
        counts[counts.len() / 2 - 1]
    }

    fn calc_free_space(&self, substrings: &SubstringMap) -> usize {
        self.max_size - substrings.len()
    }
}

impl LedgerPolicy for CaptureAll {
    fn cleanup(&self, _substrings: &mut SubstringMap) {}

    fn should_merge(&self, _x: &Substring, _y: &Substring, _substrings: &SubstringMap) -> bool {
        true
    }
}

impl LedgerPolicy for LimitDictionarySize {
    fn cleanup(&self, substrings: &mut SubstringMap) {
        if self.should_cleanup(substrings) {
            let median_count = self.calc_median_count(substrings);
            substrings.retain(|_, count| *count >= median_count);
        }
    }

    fn should_merge(&self, x: &Substring, y: &Substring, substrings: &SubstringMap) -> bool {
        if self.is_full(substrings) {
            return false;
        }

        let threshold = self.calc_merge_threshold(substrings);
        let x_count = *substrings.get(x).unwrap();
        let y_count = *substrings.get(y).unwrap();
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
            let x = Substring::from_str("x");
            let y = Substring::from_str("y");
            let policy = LimitDictionarySize { max_size: 4 };
            let mut substrings = SubstringMap::new();

            substrings.insert(x.clone(), 3);
            substrings.insert(y.clone(), 3);

            assert!(policy.should_merge(&x, &y, &substrings));
            assert!(policy.should_merge(&y, &x, &substrings));
        }

        #[test]
        fn should_merge_when_count_is_equal_to_threshold() {
            let x = Substring::from_str("x");
            let y = Substring::from_str("y");
            let policy = LimitDictionarySize { max_size: 4 };
            let mut substrings = SubstringMap::new();

            substrings.insert(x.clone(), 2);
            substrings.insert(y.clone(), 3);

            assert!(policy.should_merge(&x, &y, &substrings));
            assert!(policy.should_merge(&y, &x, &substrings));
        }

        #[test]
        fn should_not_merge_when_at_least_one_count_is_less_than_threshold() {
            let x = Substring::from_str("x");
            let y = Substring::from_str("y");
            let policy = LimitDictionarySize { max_size: 4 };
            let mut substrings = SubstringMap::new();

            substrings.insert(x.clone(), 1);
            substrings.insert(y.clone(), 3);

            assert!(!policy.should_merge(&x, &y, &substrings));
            assert!(!policy.should_merge(&y, &x, &substrings));
        }

        #[test]
        fn should_not_merge_when_dict_is_full() {
            /*
                Do not merge strings when the dictionary is full, regardless of their counts
            */
            let x = Substring::from_str("x");
            let y = Substring::from_str("y");
            let policy = LimitDictionarySize { max_size: 2 };
            let mut substrings = SubstringMap::new();

            substrings.insert(x.clone(), usize::MAX);
            substrings.insert(y.clone(), usize::MAX);

            assert!(!policy.should_merge(&x, &y, &substrings));
            assert!(!policy.should_merge(&y, &x, &substrings));
        }

        #[test]
        fn should_merge_with_fractional_threshold_rounds_to_upper_threshold_bound() {
            /*
               Given the dictionary of max_size = 7, and current size = 3 (threshold = 1.75)
               we should merge substrings whose counts are at least 2 (1.75 rounded up to 2)
            */
            let x = Substring::from_str("x");
            let y = Substring::from_str("y");
            let z = Substring::from_str("z");
            let policy = LimitDictionarySize { max_size: 7 };
            let mut substrings = SubstringMap::new();

            substrings.insert(x.clone(), 3);
            substrings.insert(y.clone(), 3);
            substrings.insert(z.clone(), 1);

            assert!(policy.should_merge(&x, &y, &substrings));
            assert!(policy.should_merge(&y, &x, &substrings));

            assert!(!policy.should_merge(&x, &z, &substrings));
            assert!(!policy.should_merge(&z, &x, &substrings));
        }
    }

    mod cleanup {
        use super::*;

        #[test]
        fn keeps_everything_when_dict_has_enough_space() {
            let x = Substring::from_str("x");
            let y = Substring::from_str("y");
            let policy = LimitDictionarySize { max_size: 10 };
            let mut substrings = SubstringMap::new();

            substrings.insert(x.clone(), 1);
            substrings.insert(y.clone(), 2);
            substrings.insert(y.clone(), 3);

            policy.cleanup(&mut substrings);
            assert!(substrings.contains_key(&x));
            assert!(substrings.contains_key(&y));
        }

        #[test]
        fn removes_all_substrings_when_not_enough_space() {
            /*
               When the there are less then 2 free slots, we should
               remove the substrings whose counts are less than median
            */
            let policy = LimitDictionarySize { max_size: 6 };
            let mut substrings = SubstringMap::new();

            substrings.insert(Substring::from_str("a"), 9);
            substrings.insert(Substring::from_str("b"), 1);
            substrings.insert(Substring::from_str("c"), 8);
            substrings.insert(Substring::from_str("x"), 3);
            substrings.insert(Substring::from_str("y"), 1);
            substrings.insert(Substring::from_str("z"), 2);

            policy.cleanup(&mut substrings);
            assert_eq!(vec!["a", "c", "x", "z"], get_substrings(&substrings));
        }

        #[test]
        fn keeps_everything_when_exactly_at_median() {
            let policy = LimitDictionarySize { max_size: 4 };
            let mut substrings = SubstringMap::new();

            // All substrings have count 2, which is the median
            substrings.insert(Substring::from_str("a"), 2);
            substrings.insert(Substring::from_str("b"), 2);
            substrings.insert(Substring::from_str("c"), 2);

            policy.cleanup(&mut substrings);
            assert_eq!(vec!["a", "b", "c"], get_substrings(&substrings));
        }

        #[test]
        fn handles_single_element_dictionary() {
            let policy = LimitDictionarySize { max_size: 2 };
            let mut substrings = SubstringMap::new();

            substrings.insert(Substring::from_str("a"), 1);

            policy.cleanup(&mut substrings);
            assert_eq!(vec!["a"], get_substrings(&substrings));
        }

        #[test]
        fn handles_empty_dictionary() {
            let policy = LimitDictionarySize { max_size: 2 };
            let mut substrings = SubstringMap::new();

            policy.cleanup(&mut substrings);
            assert_eq!(substrings.len(), 0);
        }

        #[test]
        fn removes_below_median_with_even_number_of_elements() {
            let policy = LimitDictionarySize { max_size: 5 };
            let mut substrings = SubstringMap::new();

            substrings.insert(Substring::from_str("a"), 1);
            substrings.insert(Substring::from_str("b"), 2);
            substrings.insert(Substring::from_str("c"), 3);
            substrings.insert(Substring::from_str("d"), 4);

            policy.cleanup(&mut substrings);
            assert_eq!(vec!["b", "c", "d"], get_substrings(&substrings));
        }

        #[test]
        fn preserves_substrings_at_median_counts() {
            let policy = LimitDictionarySize { max_size: 6 };
            let mut substrings = SubstringMap::new();

            substrings.insert(Substring::from_str("a"), 1);
            substrings.insert(Substring::from_str("b"), 2); // median
            substrings.insert(Substring::from_str("c"), 2); // median
            substrings.insert(Substring::from_str("d"), 2); // median
            substrings.insert(Substring::from_str("e"), 3);

            policy.cleanup(&mut substrings);
            assert_eq!(vec!["b", "c", "d", "e"], get_substrings(&substrings));
        }

        fn get_substrings(substrings: &SubstringMap) -> Vec<&str> {
            substrings
                .keys()
                .into_iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
        }
    }
}
