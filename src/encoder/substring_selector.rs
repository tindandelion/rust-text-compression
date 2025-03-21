use std::cmp::Ordering;

use super::{encoder_spec::EncoderSpec, Substring};

type ImpactComparator = fn(&EncodingImpact, &EncodingImpact) -> Ordering;

pub struct SubstringSelector {
    spec: EncoderSpec,
    impact_comparator: ImpactComparator,
}

struct EncodingImpact {
    substring: Substring,
    compression_gain: usize,
    count: usize,
}

impl SubstringSelector {
    fn new(spec: EncoderSpec, impact_comparator: ImpactComparator) -> Self {
        Self {
            spec,
            impact_comparator,
        }
    }

    pub fn order_by_compression_gain(spec: EncoderSpec) -> Self {
        let comparator: ImpactComparator = |a, b| b.compression_gain.cmp(&a.compression_gain);
        Self::new(spec, comparator)
    }

    pub fn order_by_frequency(spec: EncoderSpec) -> Self {
        let comparator: ImpactComparator = |a, b| b.count.cmp(&a.count);
        Self::new(spec, comparator)
    }

    pub fn select_substrings<'a>(
        &self,
        substrings: impl Iterator<Item = (&'a Substring, usize)>,
    ) -> Vec<Substring> {
        let impacts = self.calculate_impacts(substrings);
        impacts
            .into_iter()
            .take(self.spec.num_strings)
            .map(|impact| impact.substring)
            .collect()
    }

    fn calculate_impacts<'a>(
        &self,
        iter: impl Iterator<Item = (&'a Substring, usize)>,
    ) -> Vec<EncodingImpact> {
        let mut impacts: Vec<EncodingImpact> = iter
            .filter(|(_, count)| *count > 1)
            .map(|(substring, count)| {
                let compression_gain = self.calc_compression_gain(substring.as_str(), count);
                EncodingImpact {
                    substring: substring.clone(),
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
        let encoded_total_size = self.spec.encoded_size * count;
        unencoded_total_size
            .checked_sub(encoded_total_size)
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SPEC: EncoderSpec = EncoderSpec {
        encoded_size: 0,
        num_strings: 10,
    };

    mod string_filtering {
        use super::*;

        const NO_SORTING: ImpactComparator = |_, _| Ordering::Equal;

        #[test]
        fn reject_strings_shorter_than_encoded_representation() {
            let spec = EncoderSpec {
                encoded_size: 3,
                num_strings: 10,
            };
            let selector = SubstringSelector::new(spec, NO_SORTING);
            let substrings = vec![
                substring_count("aaaa", 2),
                substring_count("aaa", 2),
                substring_count("a", 2),
            ];

            let selected = selector.select_substrings(make_iter(&substrings));
            assert_eq!(vec!["aaaa"], to_strings(selected));
        }

        #[test]
        fn reject_single_occurrences() {
            let spec = EncoderSpec {
                encoded_size: 2,
                num_strings: 10,
            };

            let selector = SubstringSelector::new(spec, NO_SORTING);
            let substrings = vec![
                substring_count("aaaa", 1),
                substring_count("aaa", 2),
                substring_count("a", 2),
            ];

            let selected = selector.select_substrings(make_iter(&substrings));
            assert_eq!(vec!["aaa"], to_strings(selected));
        }

        #[test]
        fn trim_substrings_to_number_of_strings_from_spec() {
            let spec = EncoderSpec {
                encoded_size: 2,
                num_strings: 2,
            };

            let selector = SubstringSelector::new(spec, NO_SORTING);
            let substrings = vec![
                substring_count("aaaa", 3),
                substring_count("aaa", 3),
                substring_count("bbbbb", 3),
                substring_count("cccc", 4),
            ];

            let selected = selector.select_substrings(make_iter(&substrings));
            assert_eq!(vec!["aaaa", "aaa"], to_strings(selected));
        }
    }

    mod order_by_compression_gain {

        use super::*;

        #[test]
        fn select_by_string_length() {
            /*
             * For equal counts, longer strings make bigger impact on compression.
             */
            let selector = SubstringSelector::order_by_compression_gain(SPEC);
            let substrings = vec![
                substring_count("bb", 2),
                substring_count("aa", 2),
                substring_count("aaaaa", 2),
            ];

            let selected = selector.select_substrings(make_iter(&substrings));
            assert_eq!(vec!["aaaaa", "bb", "aa"], to_strings(selected));
        }

        #[test]
        fn select_by_count() {
            /*
             * For equal string lengths, more frequent substrings make bigger impact on compression.
             */
            let selector = SubstringSelector::order_by_compression_gain(SPEC);
            let substrings = vec![substring_count("aa", 2), substring_count("bb", 3)];

            let selected = selector.select_substrings(make_iter(&substrings));
            assert_eq!(vec!["bb", "aa"], to_strings(selected));
        }

        #[test]
        fn select_by_count_and_string_length() {
            /*
             * The string has more impact, when the total length of all its occurrences is bigger.
             */
            let selector = SubstringSelector::order_by_compression_gain(SPEC);
            let substrings = vec![
                substring_count("a", 2),
                substring_count("aaa", 2),
                substring_count("b", 8),
            ];

            let selected = selector.select_substrings(make_iter(&substrings));
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
            let selector = SubstringSelector::order_by_compression_gain(SPEC);
            let substrings = vec![substring_count("aaaaa", 2), substring_count("aaa", 3)];

            let selected = selector.select_substrings(make_iter(&substrings));
            assert_eq!(vec!["aaaaa", "aaa"], to_strings(selected));
        }
    }

    mod order_by_frequency {

        use super::*;

        #[test]
        fn order_by_occurrence_frequency() {
            let selector = SubstringSelector::order_by_frequency(SPEC);
            let substrings = vec![
                substring_count("a", 3),
                substring_count("b", 5),
                substring_count("aaa", 2),
            ];

            let selected = selector.select_substrings(make_iter(&substrings));
            assert_eq!(vec!["b", "a", "aaa"], to_strings(selected));
        }
    }

    fn substring_count(substring: &str, count: usize) -> (Substring, usize) {
        (substring.into(), count)
    }

    fn to_strings(substrings: Vec<Substring>) -> Vec<String> {
        substrings.into_iter().map(|s| s.to_string()).collect()
    }

    fn make_iter(
        substrings: &Vec<(Substring, usize)>,
    ) -> impl Iterator<Item = (&Substring, usize)> {
        substrings.iter().map(|(s, c)| (s, *c))
    }
}
