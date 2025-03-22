mod core;
mod decoder;
mod encoder;

pub use decoder::decode;
pub use encoder::encode;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn encode_and_decode_ascii_string() {
        let source =
        "low low low low low lowest lowest newer newer newer newer newer newer wider wider wider new new";

        let (encoded, substrings) = encode(source);
        let decoded = decode(&encoded, &substrings);

        assert_eq!(decoded, source);
    }

    #[test]
    fn encode_and_decode_multi_byte_string() {
        let source = "こんにちはこんにちは世界世界";

        let (encoded, substrings) = encode(source);
        let decoded = decode(&encoded, &substrings);
        assert_eq!(decoded, source);
    }
}
