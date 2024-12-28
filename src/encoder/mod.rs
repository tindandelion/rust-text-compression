mod build_ledger;
mod encode_string;
mod encoder_spec;
mod substring_ledger;

use build_ledger::build_ledger;
use encode_string::{encode_string, SPEC as ENCODER_SPEC};
use substring_ledger::SubstringLedger;

pub fn encode(string: &str) -> (Vec<u8>, Vec<String>) {
    let ledger = build_ledger(string);
    let substrings = ledger.get_most_impactful_strings(&ENCODER_SPEC);
    let encoded = encode_string(string, &substrings);
    (encoded, substrings)
}
