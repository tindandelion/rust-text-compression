mod build_ledger;
mod encode_string;
mod encoder_spec;
pub mod ledger_policies;
mod substring_counts;
mod substring_ledger;
pub mod substring_selector;

use build_ledger::build_ledger;
use encode_string::encode_string;
pub use encode_string::SPEC as ENCODER_SPEC;
use substring_ledger::{LedgerPolicy, SubstringLedger};
use substring_selector::SubstringSelector;

use crate::core::EncodingTable;

const LEDGER_SIZE: usize = 65_536;

pub fn encode(string: &str) -> (Vec<u8>, EncodingTable, usize) {
    let policy = ledger_policies::LimitLedgerSize::with_max_size(LEDGER_SIZE);
    encode_with_policy(string, policy)
}

pub fn encode_with_policy<P: LedgerPolicy>(
    string: &str,
    policy: P,
) -> (Vec<u8>, EncodingTable, usize) {
    let ledger = build_ledger(string, policy);
    let ledger_size = ledger.len();
    let substring_selector = SubstringSelector::order_by_frequency(ENCODER_SPEC);
    let substrings = ledger.build_encoding_table(&substring_selector);
    let encoded = encode_string(string, &substrings);
    (encoded, substrings, ledger_size)
}
