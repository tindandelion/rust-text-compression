mod encode_string;
mod encoder_spec;
mod learn_substrings;
mod substring_ledger;

use encode_string::{encode_string, SPEC as ENCODER_SPEC};
use learn_substrings::learn_substrings;
use substring_ledger::SubstringLedger;

pub fn encode(string: &str) -> (Vec<u8>, Vec<String>) {
    let substrings = learn_substrings(string, &ENCODER_SPEC);
    let encoded = encode_string(string, &substrings);
    (encoded, substrings)
}
