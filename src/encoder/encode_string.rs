use super::encoder_spec::EncoderSpec;

pub const SPEC: EncoderSpec = EncoderSpec {
    num_strings: 256,
    encoded_size: 2,
};

pub fn encode_string(source: &str, substrings: &[String]) -> Vec<u8> {
    assert!(substrings.len() <= 256);
    let mut result = vec![];

    let mut head = source;
    while head.len() > 0 {
        let found = substrings
            .iter()
            .enumerate()
            .find(|(_, substr)| head.starts_with(*substr));

        match found {
            Some((index, substr)) => {
                result.extend([0xF5, index as u8]);
                head = &head[substr.len()..];
            }
            None => {
                result.push(head.as_bytes()[0]);
                head = &head[1..];
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_string_with_empty_substrings() {
        let source = "abc";

        let encoded = encode_string(&source, &vec![]);
        assert_eq!(source.as_bytes(), encoded);
    }

    #[test]
    fn encode_substring_with_index_in_substring_list() {
        let source = "abc";
        let substrings = vec!["abc".to_string()];

        let encoded = encode_string(&source, &substrings);
        assert_eq!(vec![0xF5, 0x00], encoded);
    }

    #[test]
    fn encode_two_consecutive_substrings() {
        let source = "abcabc";
        let substrings = vec!["abc".to_string()];

        let encoded = encode_string(&source, &substrings);
        assert_eq!(vec![0xF5, 0x00, 0xF5, 0x00], encoded);
    }

    #[test]
    fn encode_two_consecutive_substrings_with_different_substrings() {
        let source = "abcdef";
        let substrings = vec!["abc".to_string(), "def".to_string()];

        let encoded = encode_string(&source, &substrings);
        assert_eq!(vec![0xF5, 0x00, 0xF5, 0x01], encoded);
    }

    #[test]
    fn encode_mix_of_substrings_and_single_characters() {
        let source = "abcxyzdef";
        let substrings = vec!["abc".to_string(), "def".to_string()];

        let encoded = encode_string(&source, &substrings);
        assert_eq!(
            vec![0xF5, 0x00, 'x' as u8, 'y' as u8, 'z' as u8, 0xF5, 0x01],
            encoded
        );
    }
}
