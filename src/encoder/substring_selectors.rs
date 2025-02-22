use super::{
    encoder_spec::EncoderSpec,
    substring::Substring,
    substring_ledger::{SubstringMap, SubstringSelector},
};

pub struct SelectByCompressionGain<'a> {
    encoder_spec: &'a EncoderSpec,
}

struct EncodingImpact {
    substring: Substring,
    compression_gain: usize,
}

impl<'a> SubstringSelector for SelectByCompressionGain<'a> {
    fn select_substrings(&self, substrings: SubstringMap) -> Vec<Substring> {
        let impacts = self.calculate_impacts(substrings);
        impacts
            .into_iter()
            .map(|impact| impact.substring)
            .take(self.encoder_spec.num_strings)
            .collect()
    }
}

impl<'a> SelectByCompressionGain<'a> {
    pub fn new(encoder_spec: &'a EncoderSpec) -> Self {
        Self { encoder_spec }
    }

    fn calculate_impacts(&self, substrings: SubstringMap) -> Vec<EncodingImpact> {
        let mut impacts: Vec<EncodingImpact> = substrings
            .into_iter()
            .map(|(substring, count)| {
                let compression_gain = self
                    .encoder_spec
                    .compression_gain(&substring.0, count as usize);
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
}

#[cfg(test)]
mod tests {
    use super::*;

    mod by_compression_gain {
        use std::collections::BTreeMap;

        use super::*;

        const SAMPLE_SPEC: EncoderSpec = EncoderSpec {
            num_strings: 1,
            encoded_size: 0,
        };

        #[test]
        fn select_by_string_length() {
            /*
             * For equal counts, longer strings make bigger impact on compression.
             */
            let selector = SelectByCompressionGain::new(&SAMPLE_SPEC);
            let substrings = BTreeMap::from([
                (Substring("bb".to_string()), 2),
                (Substring("aa".to_string()), 2),
                (Substring("aaaaa".to_string()), 2),
            ]);

            let selected = selector.select_substrings(substrings);
            assert_eq!(vec!["aaaaa"], to_strings(selected));
        }

        #[test]
        fn select_by_count() {
            /*
             * For equal string lengths, more frequent substrings make bigger impact on compression.
             */
            let selector = SelectByCompressionGain::new(&SAMPLE_SPEC);
            let substrings = BTreeMap::from([
                (Substring("aa".to_string()), 2),
                (Substring("bb".to_string()), 3),
            ]);

            let selected = selector.select_substrings(substrings);
            assert_eq!(vec!["bb"], to_strings(selected));
        }

        #[test]
        fn select_by_count_and_string_length() {
            /*
             * The string has more impact, when the total length of all its occurrences is bigger.
             */
            let selector = SelectByCompressionGain::new(&SAMPLE_SPEC);
            let substrings = BTreeMap::from([
                (Substring("a".to_string()), 1),
                (Substring("aaa".to_string()), 1),
                (Substring("b".to_string()), 4),
            ]);

            let selected = selector.select_substrings(substrings);
            assert_eq!(vec!["b"], to_strings(selected));
        }

        #[test]
        fn reject_strings_shorter_than_encoded_representation() {
            /*
             * Short strings are not encoded, because their encoded size is bigger than or equal to the string's length itself.
             */
            let encoder_spec_with_encoded_size = EncoderSpec {
                num_strings: 10,
                encoded_size: 2,
            };
            let selector = SelectByCompressionGain::new(&encoder_spec_with_encoded_size);
            let substrings = BTreeMap::from([
                (Substring("aaa".to_string()), 2),
                (Substring("a".to_string()), 2),
                (Substring("aa".to_string()), 4),
            ]);

            let selected = selector.select_substrings(substrings);
            assert_eq!(vec!["aaa"], to_strings(selected));
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
            let selector = SelectByCompressionGain::new(&SAMPLE_SPEC);
            let substrings = BTreeMap::from([
                (Substring("aaaaa".to_string()), 2),
                (Substring("aaa".to_string()), 3),
            ]);

            let selected = selector.select_substrings(substrings);
            assert_eq!(vec!["aaaaa"], to_strings(selected));
        }

        fn to_strings(substrings: Vec<Substring>) -> Vec<String> {
            substrings.into_iter().map(|s| s.0).collect()
        }
    }
}
