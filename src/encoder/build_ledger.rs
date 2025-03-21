use super::{
    substring::SubstringCount, substring_ledger::LedgerPolicy, Substring, SubstringLedger,
};

pub fn build_ledger<LP: LedgerPolicy>(source: &str, policy: LP) -> SubstringLedger<LP> {
    BuildState::new(source, policy).run_until_end().ledger
}

struct BuildState<'a, LP: LedgerPolicy> {
    head: &'a str,
    ledger: SubstringLedger<LP>,
    last_match: Option<SubstringCount>,
}

impl<'a, LP: LedgerPolicy> BuildState<'a, LP> {
    fn new(head: &'a str, policy: LP) -> Self {
        Self {
            head,
            ledger: SubstringLedger::with_policy(policy),
            last_match: None,
        }
    }

    fn run_until_end(mut self) -> BuildState<'a, LP> {
        while !self.at_end() {
            self = self.step();
        }
        self
    }

    fn at_end(&self) -> bool {
        self.head.len() == 0
    }

    fn step(mut self) -> BuildState<'a, LP> {
        if let Some(next_char) = self.head.chars().next() {
            if let Some(substr_match) = self.find_longest_match() {
                self.ledger.increment_count(&substr_match.value);
                self.merge_with_follow_up_match(&substr_match)
            } else {
                self.create_single_char_substring(next_char)
            }
        } else {
            self.make_end_state()
        }
    }

    fn find_longest_match(&self) -> Option<SubstringCount> {
        self.last_match
            .clone()
            .or_else(|| self.ledger.find_longest_match(self.head))
    }

    fn should_merge(&self, substr_match: &SubstringCount, follow_up: &SubstringCount) -> bool {
        self.ledger
            .should_merge(substr_match.count, follow_up.count)
    }

    fn merge_with_follow_up_match(mut self, substr_match: &SubstringCount) -> BuildState<'a, LP> {
        let rest = &self.head[substr_match.value.len()..];
        let follow_up_match = self.ledger.find_longest_match(rest);
        let mut last_match = follow_up_match.clone();

        if let Some(follow_up) = &follow_up_match {
            if self.should_merge(substr_match, follow_up) {
                let new_substring = substr_match.value.concat(&follow_up.value);
                self.ledger.increment_count(&new_substring);
            }
            if !self.ledger.contains(&follow_up.value) {
                last_match = None;
            }
        } else {
            last_match = None;
        }

        self.make_new_state(rest, last_match)
    }

    fn create_single_char_substring(mut self, next_char: char) -> BuildState<'a, LP> {
        let new_substring = Substring::from_char(next_char);
        let rest = &self.head[new_substring.len()..];
        self.ledger.increment_count(&new_substring);
        self.make_new_state(rest, None)
    }

    fn make_end_state(self) -> BuildState<'a, LP> {
        self.make_new_state("", None)
    }

    fn make_new_state(
        self,
        head: &'a str,
        last_match: Option<SubstringCount>,
    ) -> BuildState<'a, LP> {
        BuildState {
            head,
            ledger: self.ledger,
            last_match,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::encoder::{ledger_policies::CaptureAll, substring_counts::SubstringCounts};

    use super::*;

    #[test]
    fn learn_unique_characters() {
        let mut state = BuildState::new("abc", CaptureAll);
        state = state.run_until_end();
        assert_eq!(vec![("a", 1), ("b", 1), ("c", 1)], state.ledger.entries());
    }

    #[test]
    fn learn_substring() {
        let mut state = BuildState::new("abab", CaptureAll);
        state = state.run_until_end();
        assert_eq!(vec![("ab", 1), ("a", 2), ("b", 2)], state.ledger.entries());
    }

    #[test]
    fn learn_several_substrings_step_by_step() {
        let mut state = BuildState::new("abcabcabc", CaptureAll);

        state = state.run_until_end();
        assert_eq!(
            vec![
                ("abc", 1),
                ("cab", 1),
                ("ab", 2),
                ("bc", 1),
                ("a", 2),
                ("b", 2),
                ("c", 3)
            ],
            state.ledger.entries()
        );
    }

    #[test]
    fn learn_substrings_with_multi_byte_characters() {
        let mut state = BuildState::new("犬猫魚鳥", CaptureAll);
        state = state.run_until_end();
        assert_eq!(
            vec![("犬", 1), ("猫", 1), ("魚", 1), ("鳥", 1)],
            state.ledger.entries()
        );
    }

    #[test]
    fn merge_three_consecutive_substrings() {
        let mut state = BuildState::new("camelot", CaptureAll);
        state.ledger.increment_count(&substring("ca"));
        state.ledger.increment_count(&substring("me"));
        state.ledger.increment_count(&substring("lot"));

        // Processing "ca" + "me" = "came"
        state = state.step();
        assert_eq!(
            vec![("came", 1), ("lot", 1), ("ca", 2), ("me", 1)],
            state.ledger.entries()
        );

        // Processing "me" + "lot" = "melot"
        state = state.step();
        assert_eq!(
            vec![("melot", 1), ("came", 1), ("lot", 1), ("ca", 2), ("me", 2)],
            state.ledger.entries()
        );

        // Processing "lot"
        state = state.step();
        assert_eq!(
            vec![("melot", 1), ("came", 1), ("lot", 2), ("ca", 2), ("me", 2)],
            state.ledger.entries()
        );
    }

    #[test]
    fn learn_same_substring_at_next_step() {
        /*
           A boundary case when we learn the same substring at the next step
           Breakdown is as follows:
            1. We learn "xx" from "x" + "x" => "xx"
            2. We use last matched value "x" with rest of the string ("x") => learn the same "xx" again
           This is a side effect of reusing the last matched value
           What we want in this case is to increment the count of "xx" by 1
        */
        let mut state = BuildState::new("xxx", CaptureAll);
        state.ledger.increment_count(&substring("x"));

        state = state.step();
        assert_eq!(vec![("xx", 1), ("x", 2)], state.ledger.entries());
        state = state.step();
        assert_eq!(vec![("xx", 2), ("x", 3)], state.ledger.entries());
    }

    #[test]
    fn do_not_merge_substrings_if_not_allowed_by_policy() {
        let state = BuildState::new("ababab", DisallowMerging);
        let next_state = state.run_until_end();
        assert_eq!(vec![("a", 3), ("b", 3)], next_state.ledger.entries());
    }

    #[test]
    fn clear_last_match_if_removed_from_ledger() {
        /*
           The edge case when we matched with the follow-up substring,
           We produce a new substring and put it into the ledger, but
           the cleanup that occurs when we insert a new substring, also
           could have removed the original substring from the ledger.

           We need to set last_match to None in that case
        */
        let mut state = BuildState::new("ababab", RemoveAll);
        state.ledger.increment_count(&substring("ab"));

        let next_state = state.step();
        assert!(next_state.last_match.is_none());
    }

    fn substring(s: &str) -> Substring {
        Substring::from(s)
    }

    struct RemoveAll;
    struct DisallowMerging;

    impl LedgerPolicy for DisallowMerging {
        fn should_merge(
            &self,
            _x_count: usize,
            _y_count: usize,
            _substrings: &SubstringCounts,
        ) -> bool {
            false
        }

        fn cleanup(&self, _counts: &mut SubstringCounts) {}
    }

    impl LedgerPolicy for RemoveAll {
        fn cleanup(&self, counts: &mut SubstringCounts) {
            counts.retain(|_, _| false);
        }

        fn should_merge(
            &self,
            _x_count: usize,
            _y_count: usize,
            _substrings: &SubstringCounts,
        ) -> bool {
            true
        }
    }
}
