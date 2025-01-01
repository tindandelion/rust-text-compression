use crate::substring_dictionary::SubstringDictionary;

pub fn decode_string(encoded_bytes: &[u8], substrings: &SubstringDictionary) -> String {
    let mut result = String::new();

    let mut head = encoded_bytes;
    while head.len() > 0 {
        let byte = head[0];
        if byte == 0xF5 {
            let index = head[1];
            result.push_str(substrings.get(index as usize));
            head = &head[2..];
        } else {
            let (char, width) = decode_first_char(head);
            result.push(char);
            head = &head[width..];
        }
    }
    result
}

fn decode_first_char(bytes: &[u8]) -> (char, usize) {
    let width = utf8_char_width(bytes[0]);
    let char = std::str::from_utf8(&bytes[..width])
        .unwrap()
        .chars()
        .next()
        .unwrap();
    (char, width)
}

fn utf8_char_width(first_byte: u8) -> usize {
    match first_byte {
        0..=127 => 1,
        192..=223 => 2,
        224..=239 => 3,
        240..=247 => 4,
        // TODO: handle invalid UTF-8 leading byte
        _ => 0, // Invalid UTF-8 leading byte
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_simple_encoded_string() {
        let string = "abcdef";
        let substrings = SubstringDictionary::new(vec![]);

        let decoded = decode_string(string.as_bytes(), &substrings);
        assert_eq!(string, decoded);
    }

    #[test]
    fn decode_string_with_encoded_substring() {
        let encoded = vec![0xF5, 0x00];
        let substrings = SubstringDictionary::new(vec!["abc".to_string()]);

        let decoded = decode_string(&encoded, &substrings);
        assert_eq!(decoded, "abc");
    }

    #[test]
    fn decode_string_with_encoded_substrings_and_single_characters() {
        let encoded = vec![0xF5, 0x00, 0x41, 0xF5, 0x01, 0x41, 0x42, 0x43];
        let substrings = SubstringDictionary::new(vec!["abc".to_string(), "def".to_string()]);

        let decoded = decode_string(&encoded, &substrings);
        assert_eq!(decoded, "abcAdefABC");
    }

    #[test]
    fn decode_string_with_multi_byte_characters() {
        let sample_string = "犬猫魚鳥";
        let encoded = sample_string.as_bytes();

        let decoded = decode_string(&encoded, &SubstringDictionary::new(vec![]));
        assert_eq!(decoded, sample_string);
    }

    // TODO: Invalid encoded string:
    // 0xF5 at the end of the string
    // Missing entry in the substring dictionary

    // TODO: Multi-byte characters: invalid UTF-8
}
