use decoder::decode_string;
use encoder::encode_string;
use std::fs;
use substr_builder::{clean_short_substrings, learn_substrings};

mod decoder;
mod encoder;
mod substr_builder;
mod substring_dictionary;

const INPUT_FILENAME: &str = "hamlet_trunc.txt";

fn main() {
    let s = fs::read_to_string(INPUT_FILENAME).unwrap();
    let mut substrings = learn_substrings(&s);
    clean_short_substrings(&mut substrings);

    println!("Some common substrings:");
    println!("{:?}", &substrings[0..10]);

    let encoded = encode_string(&s, &substrings);
    let encoded_len = encoded.len();
    let original_len = s.bytes().len();
    let compression_ratio = (1.0 - (encoded_len as f32 / original_len as f32)) * 100.0;
    println!(
        "Original size: {} bytes, encoded size: {} bytes, compression ratio: {:.2}%",
        original_len, encoded_len, compression_ratio
    );

    let decoded = decode_string(&encoded, &substrings);
    println!("Decoded matches original: {}", decoded == s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_and_decode_ascii_string() {
        let source =
        "low low low low low lowest lowest newer newer newer newer newer newer wider wider wider new new"
            .to_string();

        let mut substrings = learn_substrings(&source);
        clean_short_substrings(&mut substrings);

        let encoded = encode_string(&source, &substrings);
        let decoded = decode_string(&encoded, &substrings);
        assert_eq!(decoded, source);
    }

    #[test]
    fn encode_and_decode_multibyte_string() {
        let source = "こんにちはこんにちは世界世界".to_string();

        let mut substrings = learn_substrings(&source);
        clean_short_substrings(&mut substrings);

        assert_eq!(10, substrings.len());

        let encoded = encode_string(&source, &substrings);
        let decoded = decode_string(&encoded, &substrings);
        assert_eq!(decoded, source);
    }

    #[test]
    fn learning_stability() {
        let source = fs::read_to_string(INPUT_FILENAME).unwrap();
        let learned_1 = learn_substrings(&source);
        let learned_2 = learn_substrings(&source);

        assert_eq!(learned_1, learned_2);
    }
}
