use decoder::decode_string;
use encoder::encode_string;
use substr_builder::{clean_short_substrings, learn_substrings};

mod decoder;
mod encoder;
mod substr_builder;
mod substring_dictionary;

fn main() {
    let s =
        "low low low low low lowest lowest newer newer newer newer newer newer wider wider wider new new"
            .to_string();
    let mut substrings = learn_substrings(&s);
    clean_short_substrings(&mut substrings);

    let encoded = encode_string(&s, &substrings);
    println!(
        "Original size: {}, encoded size: {}",
        s.bytes().len(),
        encoded.len()
    );
    println!("{:?}", encoded);

    let decoded = decode_string(&encoded, &substrings);
    println!("{}", decoded);
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
}
