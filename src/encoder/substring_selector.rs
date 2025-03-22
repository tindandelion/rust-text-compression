use crate::core::Substring;

use super::encoder_spec::EncoderSpec;

pub struct SubstringSelector {
    spec: EncoderSpec,
}

struct EncodingImpact {
    substring: Substring,
    compression_gain: usize,
    count: usize,
}

impl SubstringSelector {
    pub fn new(spec: EncoderSpec) -> Self {
        Self { spec }
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
                let compression_gain = self.calc_compression_gain(substring, count);
                EncodingImpact {
                    substring: substring.clone(),
                    compression_gain,
                    count,
                }
            })
            .filter(|impact| impact.compression_gain > 0)
            .collect();
        impacts.sort_by(|a, b| b.count.cmp(&a.count));
        impacts
    }

    fn calc_compression_gain(&self, string: &Substring, count: usize) -> usize {
        let unencoded_total_size = string.len() * count;
        let encoded_total_size = self.spec.encoded_size * count;
        unencoded_total_size.saturating_sub(encoded_total_size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reject_strings_shorter_than_encoded_representation() {
        let spec = EncoderSpec {
            encoded_size: 3,
            num_strings: 10,
        };
        let selector = SubstringSelector::new(spec);
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

        let selector = SubstringSelector::new(spec);
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

        let selector = SubstringSelector::new(spec);
        let substrings = vec![
            substring_count("aaaa", 3),
            substring_count("aaa", 3),
            substring_count("bbbbb", 3),
            substring_count("cccc", 4),
        ];

        let selected = selector.select_substrings(make_iter(&substrings));
        assert_eq!(vec!["cccc", "aaaa"], to_strings(selected));
    }

    #[test]
    fn order_by_occurrence_frequency() {
        let spec = EncoderSpec {
            encoded_size: 0,
            num_strings: 10,
        };

        let selector = SubstringSelector::new(spec);
        let substrings = vec![
            substring_count("a", 3),
            substring_count("b", 5),
            substring_count("aaa", 2),
        ];

        let selected = selector.select_substrings(make_iter(&substrings));
        assert_eq!(vec!["b", "a", "aaa"], to_strings(selected));
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
