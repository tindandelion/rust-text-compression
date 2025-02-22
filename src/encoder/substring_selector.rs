use std::cmp::Ordering;

use super::{substring::Substring, substring_ledger::SubstringMap};

type ImpactComparator = fn(&EncodingImpact, &EncodingImpact) -> Ordering;

pub struct SubstringSelector {
    encoded_size: usize,
    impact_comparator: ImpactComparator,
}

struct EncodingImpact {
    substring: Substring,
    compression_gain: usize,
    count: usize,
}

impl SubstringSelector {
    fn new(encoded_size: usize, impact_comparator: ImpactComparator) -> Self {
        Self {
            encoded_size,
            impact_comparator,
        }
    }

    pub fn order_by_compression_gain(encoded_size: usize) -> Self {
        let comparator: ImpactComparator = |a, b| b.compression_gain.cmp(&a.compression_gain);
        Self::new(encoded_size, comparator)
    }

    pub fn order_by_frequency(encoded_size: usize) -> Self {
        let comparator: ImpactComparator = |a, b| b.count.cmp(&a.count);
        Self::new(encoded_size, comparator)
    }

    pub fn select_substrings(&self, substrings: SubstringMap) -> Vec<Substring> {
        let impacts = self.calculate_impacts(substrings);
        impacts.into_iter().map(|impact| impact.substring).collect()
    }

    fn calculate_impacts(&self, substrings: SubstringMap) -> Vec<EncodingImpact> {
        let mut impacts: Vec<EncodingImpact> = substrings
            .into_iter()
            .filter(|(_, count)| *count > 1)
            .map(|(substring, count)| {
                let compression_gain =
                    self.calc_compression_gain(substring.as_str(), count as usize);
                EncodingImpact {
                    substring,
                    compression_gain,
                    count,
                }
            })
            .filter(|impact| impact.compression_gain > 0)
            .collect();
        impacts.sort_by(self.impact_comparator);
        impacts
    }

    fn calc_compression_gain(&self, string: &str, count: usize) -> usize {
        let unencoded_total_size = string.len() * count;
        let encoded_total_size = self.encoded_size * count;
        unencoded_total_size
            .checked_sub(encoded_total_size)
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod string_filtering {
        use std::collections::BTreeMap;

        use super::*;

        const NO_SORTING: ImpactComparator = |_, _| Ordering::Equal;

        #[test]
        fn reject_strings_shorter_than_encoded_representation() {
            let encoded_size = 3;
            let selector = SubstringSelector::new(encoded_size, NO_SORTING);
            let substrings: SubstringMap =
                BTreeMap::from([("aaaa".into(), 2), ("aaa".into(), 2), ("a".into(), 2)]);

            let selected = selector.select_substrings(substrings);
            assert_eq!(vec!["aaaa"], to_strings(selected));
        }

        #[test]
        fn reject_single_occurrences() {
            let encoded_size = 2;
            let selector = SubstringSelector::new(encoded_size, NO_SORTING);
            let substrings: SubstringMap =
                BTreeMap::from([("aaaa".into(), 1), ("aaa".into(), 2), ("a".into(), 2)]);

            let selected = selector.select_substrings(substrings);
            assert_eq!(vec!["aaa"], to_strings(selected));
        }
    }

    mod order_by_compression_gain {
        use std::collections::BTreeMap;

        use super::*;

        #[test]
        fn select_by_string_length() {
            /*
             * For equal counts, longer strings make bigger impact on compression.
             */
            let selector = SubstringSelector::order_by_compression_gain(0);
            let substrings: SubstringMap =
                BTreeMap::from([("bb".into(), 2), ("aa".into(), 2), ("aaaaa".into(), 2)]);

            let selected = selector.select_substrings(substrings);
            assert_eq!(vec!["aaaaa", "aa", "bb"], to_strings(selected));
        }

        #[test]
        fn select_by_count() {
            /*
             * For equal string lengths, more frequent substrings make bigger impact on compression.
             */
            let selector = SubstringSelector::order_by_compression_gain(0);
            let substrings: SubstringMap = BTreeMap::from([("aa".into(), 2), ("bb".into(), 3)]);

            let selected = selector.select_substrings(substrings);
            assert_eq!(vec!["bb", "aa"], to_strings(selected));
        }

        #[test]
        fn select_by_count_and_string_length() {
            /*
             * The string has more impact, when the total length of all its occurrences is bigger.
             */
            let selector = SubstringSelector::order_by_compression_gain(0);
            let substrings: SubstringMap =
                BTreeMap::from([("a".into(), 2), ("aaa".into(), 2), ("b".into(), 8)]);

            let selected = selector.select_substrings(substrings);
            assert_eq!(vec!["b", "aaa", "a"], to_strings(selected));
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
            let selector = SubstringSelector::order_by_compression_gain(0);
            let substrings: SubstringMap = BTreeMap::from([("aaaaa".into(), 2), ("aaa".into(), 3)]);

            let selected = selector.select_substrings(substrings);
            assert_eq!(vec!["aaaaa", "aaa"], to_strings(selected));
        }
    }

    mod order_by_frequency {
        use std::collections::BTreeMap;

        use super::*;

        #[test]
        fn order_by_occurrence_frequency() {
            let selector = SubstringSelector::order_by_frequency(0);
            let substrings: SubstringMap =
                BTreeMap::from([("a".into(), 3), ("b".into(), 5), ("aaa".into(), 2)]);

            let selected = selector.select_substrings(substrings);
            assert_eq!(vec!["b", "a", "aaa"], to_strings(selected));
        }
    }

    fn to_strings(substrings: Vec<Substring>) -> Vec<String> {
        substrings.into_iter().map(|s| s.to_string()).collect()
    }
}
