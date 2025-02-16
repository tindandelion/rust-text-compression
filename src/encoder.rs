mod build_ledger;
mod encode_string;
mod encoder_spec;
pub mod ledger_policies;
mod substring;
mod substring_ledger;

use build_ledger::build_ledger;
use encode_string::encode_string;
pub use encode_string::SPEC as ENCODER_SPEC;
use substring::Substring;
use substring_ledger::{LedgerPolicy, SubstringLedger};

use crate::substring_dictionary::SubstringDictionary;

pub fn encode_with_policy<P: LedgerPolicy>(
    string: &str,
    policy: P,
) -> (Vec<u8>, SubstringDictionary, usize) {
    let ledger = build_ledger(string, policy);
    let ledger_size = ledger.len();
    let substrings = ledger.get_most_impactful_strings(&ENCODER_SPEC);
    let encoded = encode_string(string, &substrings);
    (encoded, substrings, ledger_size)
}
