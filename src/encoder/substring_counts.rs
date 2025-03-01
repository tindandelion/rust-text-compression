use super::{substring_ledger::SubstringMap, Substring};

pub struct SubstringCounts<'a>(pub &'a SubstringMap);

impl<'a> SubstringCounts<'a> {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, substr: &Substring) -> Option<usize> {
        self.0.get(substr).map(|&count| count)
    }
}
