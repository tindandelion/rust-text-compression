use decoder::decode_string;
use encoder::encode_string;
use std::fs;
use substr_builder::learn_substrings;
use substring_dictionary::EncoderSpec;

mod decoder;
mod encoder;
mod substr_builder;
mod substring_dictionary;

const INPUT_FILENAME: &str = "hamlet_trunc.txt";
const ENCODER_SPEC: EncoderSpec = EncoderSpec {
    num_strings: 256,
    encoded_size: 2,
};

fn main() {
    let s = fs::read_to_string(INPUT_FILENAME).unwrap();
    let substrings = learn_substrings(&s, &ENCODER_SPEC);

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

        let substrings = learn_substrings(&source, &ENCODER_SPEC);

        let encoded = encode_string(&source, &substrings);
        let decoded = decode_string(&encoded, &substrings);
        assert_eq!(decoded, source);
    }

    #[test]
    fn encode_and_decode_multibyte_string() {
        let source = "こんにちはこんにちは世界世界".to_string();

        let substrings = learn_substrings(&source, &ENCODER_SPEC);
        assert_eq!(10, substrings.len());

        let encoded = encode_string(&source, &substrings);
        let decoded = decode_string(&encoded, &substrings);
        assert_eq!(decoded, source);
    }

    #[test]

    fn learning_stability() {
        let encoder_spec = EncoderSpec {
            num_strings: 10,
            encoded_size: 2,
        };
        let source = fs::read_to_string(INPUT_FILENAME).unwrap();
        let learned_1 = learn_substrings(&source, &encoder_spec);
        let learned_2 = learn_substrings(&source, &encoder_spec);

        assert_eq!(learned_1, learned_2);
    }
}
