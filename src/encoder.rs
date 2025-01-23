mod build_ledger;
mod encode_string;
mod encoder_spec;
mod ledger_policies;
mod substring;
mod substring_ledger;

use build_ledger::build_ledger;
use encode_string::{encode_string, SPEC as ENCODER_SPEC};
use ledger_policies::CaptureAll;
use substring::Substring;
use substring_ledger::SubstringLedger;

use crate::substring_dictionary::SubstringDictionary;

pub fn encode(string: &str) -> (Vec<u8>, SubstringDictionary) {
    let ledger = build_ledger(string, CaptureAll);
    let substrings = ledger.get_most_impactful_strings(&ENCODER_SPEC);
    let encoded = encode_string(string, &substrings);
    (encoded, substrings)
}
