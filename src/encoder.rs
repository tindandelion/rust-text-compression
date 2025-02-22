mod build_ledger;
mod encode_string;
mod encoder_spec;
pub mod ledger_policies;
mod substring;
mod substring_ledger;
pub mod substring_selectors;

use build_ledger::build_ledger;
use encode_string::encode_string;
pub use encode_string::SPEC as ENCODER_SPEC;
use substring::Substring;
use substring_ledger::{LedgerPolicy, SubstringLedger, SubstringSelector};
use substring_selectors::SelectByCompressionGain;

use crate::encoding_table::EncodingTable;

pub fn encode_with_policy<P: LedgerPolicy>(
    string: &str,
    policy: P,
) -> (Vec<u8>, EncodingTable, usize) {
    encode(string, policy, SelectByCompressionGain::new(&ENCODER_SPEC))
}

fn encode<P: LedgerPolicy>(
    string: &str,
    ledger_policy: P,
    substring_selector: impl SubstringSelector,
) -> (Vec<u8>, EncodingTable, usize) {
    let ledger = build_ledger(string, ledger_policy);
    let ledger_size = ledger.len();
    let substrings = ledger.select_substrings(&substring_selector);
    let encoded = encode_string(string, &substrings);
    (encoded, substrings, ledger_size)
}
