use crate::core::EncodingTable;

use super::error::DecodeError;

const ENCODED_MARKER: u8 = 0xF5;

pub fn decode(encoded_bytes: &[u8], substrings: &EncodingTable) -> Result<String, DecodeError> {
    let mut result = String::new();

    let mut head = encoded_bytes;
    while !head.is_empty() {
        let byte = head[0];
        if byte >= ENCODED_MARKER {
            let substring = decode_substring(head, substrings)?;
            result.push_str(substring);
            head = &head[2..];
        } else {
            let (char, width) = decode_first_char(head)?;
            result.push(char);
            head = &head[width..];
        }
    }
    Ok(result)
}

fn decode_substring<'a>(
    text_head: &'a [u8],
    substrings: &'a EncodingTable,
) -> Result<&'a str, DecodeError> {
    if text_head.len() < 2 {
        return Err(DecodeError::MissingEntryIndex);
    }
    let hi_byte = ((text_head[0] - ENCODED_MARKER) as u16) << 8;
    let lo_byte = text_head[1] as u16;
    let index = (hi_byte + lo_byte) as usize;
    substrings
        .get(index)
        .ok_or(DecodeError::InvalidEntryIndex(index))
}

fn decode_first_char(bytes: &[u8]) -> Result<(char, usize), DecodeError> {
    let width = utf8_char_width(bytes[0])?;
    if bytes.len() < width {
        return Err(DecodeError::InvalidUtf8Length {
            expected: width,
            actual: bytes.len(),
        });
    }

    let char = std::str::from_utf8(&bytes[..width])?
        .chars()
        .next()
        .ok_or(DecodeError::InvalidUtf8Character)?;
    Ok((char, width))
}

fn utf8_char_width(first_byte: u8) -> Result<usize, DecodeError> {
    match first_byte {
        0..=127 => Ok(1),
        192..=223 => Ok(2),
        224..=239 => Ok(3),
        240..=247 => Ok(4),
        _ => Err(DecodeError::InvalidUtf8LeadingByte(first_byte)),
    }
}

#[cfg(test)]
mod tests {

    use crate::core::Substring;

    use super::*;

    #[test]
    fn decode_simple_encoded_string() {
        let string = "abcdef";
        let substrings = EncodingTable::new(vec![]);

        let decoded = decode(string.as_bytes(), &substrings).unwrap();
        assert_eq!(string, decoded);
    }

    #[test]
    fn decode_string_with_encoded_substring() {
        let encoded = vec![0xF5, 0x00];
        let substrings = make_encoding_table(vec!["abc".to_string()]);

        let decoded = decode(&encoded, &substrings).unwrap();
        assert_eq!(decoded, "abc");
    }

    #[test]
    fn decode_string_with_encoded_substrings_and_single_characters() {
        let encoded = vec![0xF5, 0x00, 0x41, 0xF5, 0x01, 0x41, 0x42, 0x43];
        let substrings = make_encoding_table(vec!["abc".to_string(), "def".to_string()]);

        let decoded = decode(&encoded, &substrings).unwrap();
        assert_eq!(decoded, "abcAdefABC");
    }

    #[test]
    fn decode_string_with_multi_byte_characters() {
        let sample_string = "犬猫魚鳥";
        let encoded = sample_string.as_bytes();

        let decoded = decode(encoded, &make_encoding_table(vec![])).unwrap();
        assert_eq!(decoded, sample_string);
    }

    #[test]
    fn decode_string_with_large_encoding_table() {
        let encoded = vec![0xF6, 0x00, 0xF6, 0x01, 0x61, 0x62, 0x63];
        let mut substrings: Vec<String> = (0..256).map(|i| format!("string_{}", i)).collect();
        substrings.push("bb".to_string());
        substrings.push("cc".to_string());

        let decoded = decode(&encoded, &make_encoding_table(substrings)).unwrap();
        assert_eq!(decoded, "bbccabc");
    }

    mod error_handling {
        use super::*;

        #[test]
        fn missing_entry_in_encoding_table() {
            let encoded = vec![0xF5, 0x01];

            let result = decode(&encoded, &make_encoding_table(vec!["a".to_string()]));
            assert_eq!(Err(DecodeError::InvalidEntryIndex(1)), result);
        }

        #[test]
        fn missing_entry_index_byte_in_encoded_string() {
            let encoded = vec![0x61, 0xF5];

            let result = decode(&encoded, &make_encoding_table(vec!["a".to_string()]));
            assert_eq!(Err(DecodeError::MissingEntryIndex), result);
        }

        #[test]
        fn invalid_utf8_character_leading_byte() {
            let encoded = vec![0x80, 0x61];

            let result = decode(&encoded, &make_encoding_table(vec![]));
            assert_eq!(Err(DecodeError::InvalidUtf8LeadingByte(0x80)), result);
        }

        #[test]
        fn missing_utf8_continuation_byte() {
            let encoded = vec![0xC2];

            let result = decode(&encoded, &make_encoding_table(vec![]));
            assert_eq!(
                Err(DecodeError::InvalidUtf8Length {
                    expected: 2,
                    actual: 1
                }),
                result
            );
        }

        #[test]
        fn invalid_utf8_continuation_byte() {
            let encoded = vec![0xC2, 0x00];

            let result = decode(&encoded, &make_encoding_table(vec![]));
            assert_eq!(Err(DecodeError::InvalidUtf8Character), result);
        }
    }

    fn make_encoding_table(substrings: Vec<String>) -> EncodingTable {
        EncodingTable::new(substrings.into_iter().map(Substring::from).collect())
    }
}
