mod decoder;
mod encoder;
mod substring_dictionary;

pub use decoder::decode_string as decode;
pub use encoder::encode_with_policy;
pub use encoder::ENCODER_SPEC;

pub mod policies {
    pub use super::encoder::ledger_policies::{CaptureAll, LimitDictionarySize};
}

#[cfg(test)]
mod tests {
    use super::*;
    use policies::CaptureAll;

    #[test]
    fn encode_and_decode_ascii_string() {
        let source =
        "low low low low low lowest lowest newer newer newer newer newer newer wider wider wider new new"
            .to_string();

        let (encoded, substrings) = encode_with_policy(&source, CaptureAll);
        let decoded = decode(&encoded, &substrings);

        assert_eq!(decoded, source);
    }

    #[test]
    fn encode_and_decode_multibyte_string() {
        let source = "こんにちはこんにちは世界世界".to_string();

        let (encoded, substrings) = encode_with_policy(&source, CaptureAll);
        let decoded = decode(&encoded, &substrings);
        assert_eq!(decoded, source);
    }
}
