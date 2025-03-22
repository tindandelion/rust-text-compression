mod build_ledger;
mod encode_string;
mod encoder_spec;
mod ledger_size_policy;
mod substring_counts;
mod substring_ledger;
mod substring_selector;

use build_ledger::build_ledger;
use encode_string::encode_string;
use encode_string::SPEC as ENCODER_SPEC;
use ledger_size_policy::LimitLedgerSize;
use substring_ledger::SubstringLedger;
use substring_selector::SubstringSelector;

pub use crate::core::EncodingTable;

const LEDGER_SIZE: usize = 65_536;

pub fn encode(string: &str) -> (Vec<u8>, EncodingTable) {
    let policy = LimitLedgerSize::with_max_size(LEDGER_SIZE);
    let ledger = build_ledger(string, policy);

    let substring_selector = SubstringSelector::new(ENCODER_SPEC);
    let substrings = ledger.build_encoding_table(&substring_selector);
    let encoded = encode_string(string, &substrings);
    (encoded, substrings)
}
