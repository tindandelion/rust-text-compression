pub fn decode_string(encoded_bytes: &[u8], substrings: &[String]) -> String {
    let mut result = String::new();

    let mut head = encoded_bytes;
    while head.len() > 0 {
        let byte = head[0];
        if byte == 0xF5 {
            let index = head[1];
            result.push_str(&substrings[index as usize]);
            head = &head[2..];
        } else {
            result.push(byte as char);
            head = &head[1..];
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_simple_encoded_string() {
        let string = "abcdef";

        let decoded = decode_string(string.as_bytes(), &vec![]);
        assert_eq!(string, decoded);
    }

    #[test]
    fn decode_string_with_encoded_substring() {
        let encoded = vec![0xF5, 0x00];
        let substrings = vec!["abc".to_string()];

        let decoded = decode_string(&encoded, &substrings);
        assert_eq!(decoded, "abc");
    }

    #[test]
    fn decode_string_with_encoded_substrings_and_single_characters() {
        let encoded = vec![0xF5, 0x00, 0x41, 0xF5, 0x01, 0x41, 0x42, 0x43];
        let substrings = vec!["abc".to_string(), "def".to_string()];

        let decoded = decode_string(&encoded, &substrings);
        assert_eq!(decoded, "abcAdefABC");
    }

    // TODO: Invalid encoded string:
    // 0xF5 at the end of the string
    // Missing entry in the table

    // TODO: Multi-byte characters
}
