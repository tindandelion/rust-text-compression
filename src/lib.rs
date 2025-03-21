mod core;
mod decoder;
mod encoder;

pub use decoder::decode_string as decode;
pub use encoder::encode;
pub use encoder::encode_with_policy;
pub use encoder::substring_selector::SubstringSelector;
pub use encoder::ENCODER_SPEC;

pub mod policies {
    pub use super::encoder::ledger_policies::{CaptureAll, LimitLedgerSize};
}

#[cfg(test)]
mod tests {
    use super::*;
    use policies::CaptureAll;

    #[test]
    fn encode_and_decode_ascii_string() {
        let source =
        "low low low low low lowest lowest newer newer newer newer newer newer wider wider wider new new";

        let (encoded, substrings, _) = encode_with_policy(source, CaptureAll);
        let decoded = decode(&encoded, &substrings);

        assert_eq!(decoded, source);
    }

    #[test]
    fn encode_and_decode_multi_byte_string() {
        let source = "こんにちはこんにちは世界世界";

        let (encoded, substrings, _) = encode_with_policy(source, CaptureAll);
        let decoded = decode(&encoded, &substrings);
        assert_eq!(decoded, source);
    }
}
