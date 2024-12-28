use decoder::decode_string;
use encoder::encode;
use std::fs;

mod decoder;
mod encoder;

const INPUT_FILENAME: &str = "test-data/hamlet-100.txt";

fn main() {
    let s = fs::read_to_string(INPUT_FILENAME).unwrap();
    let (encoded, substrings) = encode(&s);

    println!("Substrings size: {}", substrings.len());
    println!("Some common substrings:");
    println!("{:?}", &substrings[0..20]);

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

        let (encoded, substrings) = encode(&source);
        let decoded = decode_string(&encoded, &substrings);

        assert_eq!(decoded, source);
    }

    #[test]
    fn encode_and_decode_multibyte_string() {
        let source = "こんにちはこんにちは世界世界".to_string();

        let (encoded, substrings) = encode(&source);
        assert_eq!(10, substrings.len());

        let decoded = decode_string(&encoded, &substrings);
        assert_eq!(decoded, source);
    }
}
