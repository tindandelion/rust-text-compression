use crate::core::EncodingTable;

const ENCODED_MARKER: u8 = 0xF5;

pub fn decode_string(encoded_bytes: &[u8], substrings: &EncodingTable) -> String {
    let mut result = String::new();

    let mut head = encoded_bytes;
    while head.len() > 0 {
        let byte = head[0];
        if byte >= ENCODED_MARKER {
            let hi_byte = ((byte - ENCODED_MARKER) as u16) << 8;
            let lo_byte = head[1] as u16;
            let index = hi_byte + lo_byte;
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
    use crate::encoder::Substring;

    use super::*;

    #[test]
    fn decode_simple_encoded_string() {
        let string = "abcdef";
        let substrings = EncodingTable::new(vec![]);

        let decoded = decode_string(string.as_bytes(), &substrings);
        assert_eq!(string, decoded);
    }

    #[test]
    fn decode_string_with_encoded_substring() {
        let encoded = vec![0xF5, 0x00];
        let substrings = make_encoding_table(vec!["abc".to_string()]);

        let decoded = decode_string(&encoded, &substrings);
        assert_eq!(decoded, "abc");
    }

    #[test]
    fn decode_string_with_encoded_substrings_and_single_characters() {
        let encoded = vec![0xF5, 0x00, 0x41, 0xF5, 0x01, 0x41, 0x42, 0x43];
        let substrings = make_encoding_table(vec!["abc".to_string(), "def".to_string()]);

        let decoded = decode_string(&encoded, &substrings);
        assert_eq!(decoded, "abcAdefABC");
    }

    #[test]
    fn decode_string_with_multi_byte_characters() {
        let sample_string = "犬猫魚鳥";
        let encoded = sample_string.as_bytes();

        let decoded = decode_string(&encoded, &make_encoding_table(vec![]));
        assert_eq!(decoded, sample_string);
    }

    #[test]
    fn decode_string_with_large_encoding_table() {
        let encoded = vec![0xF6, 0x00, 0xF6, 0x01, 0x61, 0x62, 0x63];
        let mut substrings: Vec<String> = (0..256).map(|i| format!("string_{}", i)).collect();
        substrings.push("bb".to_string());
        substrings.push("cc".to_string());

        let decoded = decode_string(&encoded, &make_encoding_table(substrings));
        assert_eq!(decoded, "bbccabc");
    }

    fn make_encoding_table(substrings: Vec<String>) -> EncodingTable {
        EncodingTable::new(substrings.into_iter().map(Substring::from).collect())
    }

    // TODO: Invalid encoded string:
    // 0xF5 at the end of the string
    // Missing entry in the substring dictionary

    // TODO: Multi-byte characters: invalid UTF-8
}
