mod build_ledger;
mod encode_string;
mod encoder_spec;
pub mod ledger_policies;
mod substring;
mod substring_counts;
mod substring_ledger;
pub mod substring_selector;

use build_ledger::build_ledger;
use encode_string::encode_string;
pub use encode_string::SPEC as ENCODER_SPEC;
pub use substring::Substring;
use substring_ledger::{LedgerPolicy, SubstringLedger};
use substring_selector::SubstringSelector;

use crate::encoding_table::EncodingTable;

pub fn encode_with_policy<P: LedgerPolicy>(
    string: &str,
    policy: P,
) -> (Vec<u8>, EncodingTable, usize) {
    encode(
        string,
        policy,
        &SubstringSelector::order_by_compression_gain(ENCODER_SPEC),
    )
}

pub fn encode<P: LedgerPolicy>(
    string: &str,
    ledger_policy: P,
    substring_selector: &SubstringSelector,
) -> (Vec<u8>, EncodingTable, usize) {
    let ledger = build_ledger(string, ledger_policy);
    let ledger_size = ledger.len();
    let substrings = ledger.build_encoding_table(&substring_selector);
    let encoded = encode_string(string, &substrings);
    (encoded, substrings, ledger_size)
}
