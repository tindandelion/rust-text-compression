use crate::encoding_table::EncodingTable;

use super::encoder_spec::EncoderSpec;

const INDEX_START: u16 = 0xF500;
const INDEX_END: u16 = 0xFFFF;

pub const SPEC: EncoderSpec = EncoderSpec {
    num_strings: (INDEX_END - INDEX_START) as usize,
    encoded_size: 2,
};

pub fn encode_string(source: &str, substrings: &EncodingTable) -> Vec<u8> {
    assert!(substrings.len() <= SPEC.num_strings);

    let mut encoding_buffer = [0; 4];
    let mut result = vec![];

    let mut head = source;
    while head.len() > 0 {
        match substrings.find_match(&head) {
            Some((index, substr)) => {
                let encoded_index = INDEX_START + index as u16;
                result.extend(encoded_index.to_be_bytes());
                head = &head[substr.len()..];
            }
            None => {
                // TODO: Error handling here
                let next_char = head.chars().next().unwrap();
                result.extend(next_char.encode_utf8(&mut encoding_buffer).as_bytes());
                head = &head[next_char.len_utf8()..];
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::encoder::Substring;

    use super::*;

    #[test]
    fn encode_string_with_empty_substrings() {
        let source = "abc";

        let encoded = encode_string(&source, &make_dictionary(vec![]));
        assert_eq!(source.as_bytes(), encoded);
    }

    #[test]
    fn encode_substring_with_index_in_substring_list() {
        let source = "abc";
        let substrings = vec!["abc".to_string()];

        let encoded = encode_string(&source, &make_dictionary(substrings));
        assert_eq!(vec![0xF5, 0x00], encoded);
    }

    #[test]
    fn encode_two_consecutive_substrings() {
        let source = "abcabc";
        let substrings = vec!["abc".to_string()];

        let encoded = encode_string(&source, &make_dictionary(substrings));
        assert_eq!(vec![0xF5, 0x00, 0xF5, 0x00], encoded);
    }

    #[test]
    fn encode_two_consecutive_substrings_with_different_substrings() {
        let source = "abcdef";
        let substrings = vec!["abc".to_string(), "def".to_string()];

        let encoded = encode_string(&source, &make_dictionary(substrings));
        assert_eq!(vec![0xF5, 0x00, 0xF5, 0x01], encoded);
    }

    #[test]
    fn encode_mix_of_substrings_and_single_characters() {
        let source = "abcxyzdef";
        let substrings = vec!["abc".to_string(), "def".to_string()];

        let encoded = encode_string(&source, &make_dictionary(substrings));
        assert_eq!(
            vec![0xF5, 0x00, 'x' as u8, 'y' as u8, 'z' as u8, 0xF5, 0x01],
            encoded
        );
    }

    #[test]
    fn encode_string_with_large_dictionary() {
        let source = "bbccabc";
        let mut substrings: Vec<String> = (0..256).map(|i| format!("string_{}", i)).collect();
        substrings.push("bb".to_string());
        substrings.push("cc".to_string());

        let encoded = encode_string(&source, &make_dictionary(substrings));
        assert_eq!(
            vec![0xF6, 0x00, 0xF6, 0x01, 'a' as u8, 'b' as u8, 'c' as u8],
            encoded
        );
    }

    #[test]
    fn encode_multibyte_string() {
        let source = "こんにちはこんにちは世界世界";

        let encoded = encode_string(&source, &make_dictionary(vec![]));
        assert_eq!(source.as_bytes(), encoded);
    }

    fn make_dictionary(substrings: Vec<String>) -> EncodingTable {
        EncodingTable::new(substrings.into_iter().map(Substring::from).collect())
    }
}
