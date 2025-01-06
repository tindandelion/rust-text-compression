mod build_ledger;
mod encode_string;
mod encoder_spec;
mod substring;
mod substring_ledger;

use build_ledger::build_ledger;
use encode_string::{encode_string, SPEC as ENCODER_SPEC};
use substring::Substring;
use substring_ledger::SubstringLedger;

use crate::substring_dictionary::SubstringDictionary;

pub fn encode(string: &str) -> (Vec<u8>, SubstringDictionary) {
    let ledger = build_ledger(string);
    let substrings = ledger.get_most_impactful_strings(&ENCODER_SPEC);
    let encoded = encode_string(string, &substrings);
    (encoded, substrings)
}
