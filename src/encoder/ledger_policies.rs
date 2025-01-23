use super::{
    substring::Substring,
    substring_ledger::{LedgerPolicy, SubstringMap},
};

pub struct CaptureAll;

impl LedgerPolicy for CaptureAll {
    fn cleanup(&self, _substrings: &mut SubstringMap) {}

    fn should_merge(&self, _x: &Substring, _y: &Substring, _substrings: &SubstringMap) -> bool {
        true
    }
}
