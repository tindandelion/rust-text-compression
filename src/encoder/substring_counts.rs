mod tries;

pub use tries::TrieSubstringCounts;

#[cfg(test)]
pub mod util {
    use crate::encoder::Substring;

    use super::*;

    pub fn get_sorted_counts(counts: &TrieSubstringCounts) -> Vec<(&Substring, usize)> {
        let mut counts = counts.iter().collect::<Vec<_>>();
        counts.sort_by_key(|(substr, _)| *substr);
        counts
    }
}
