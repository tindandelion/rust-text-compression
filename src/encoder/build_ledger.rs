use super::{Substring, SubstringLedger};

pub fn build_ledger(source: &str) -> SubstringLedger {
    BuildState::new(source).run_until_end().ledger
}

struct BuildState<'a> {
    head: &'a str,
    ledger: SubstringLedger,
    last_match: Option<Substring>,
}

impl<'a> BuildState<'a> {
    fn new(head: &'a str) -> Self {
        Self {
            head,
            ledger: SubstringLedger::new(),
            last_match: None,
        }
    }

    fn run_until_end(mut self) -> BuildState<'a> {
        while !self.at_end() {
            self = self.step();
        }
        self
    }

    fn at_end(&self) -> bool {
        self.head.len() == 0
    }

    fn step(mut self) -> BuildState<'a> {
        if let Some(next_char) = self.head.chars().next() {
            if let Some(substr_match) = self.find_longest_match() {
                self.ledger.increment_count(&substr_match);
                self.merge_with_follow_up_match(&substr_match)
            } else {
                self.create_single_char_substring(next_char)
            }
        } else {
            self.make_end_state()
        }
    }

    fn find_longest_match(&self) -> Option<Substring> {
        self.last_match
            .clone()
            .or_else(|| self.ledger.find_longest_match(self.head))
    }

    fn merge_with_follow_up_match(mut self, substr_match: &Substring) -> BuildState<'a> {
        let rest = &self.head[substr_match.len()..];
        let follow_up_match = self.ledger.find_longest_match(rest);

        if let Some(follow_up_match) = &follow_up_match {
            let new_substring = substr_match.concat(follow_up_match);
            self.ledger.insert_new(new_substring);
        }

        BuildState {
            head: rest,
            ledger: self.ledger,
            last_match: follow_up_match,
        }
    }

    fn create_single_char_substring(mut self, next_char: char) -> BuildState<'a> {
        let new_substring = Substring::from_char(next_char);
        let rest = &self.head[new_substring.len()..];
        self.ledger.insert_new(new_substring);
        BuildState {
            head: rest,
            ledger: self.ledger,
            last_match: None,
        }
    }

    fn make_end_state(self) -> BuildState<'a> {
        BuildState {
            head: "",
            ledger: self.ledger,
            last_match: None,
        }
    }
}

#[cfg(test)]
mod build_ledger_step_tests {
    use super::*;

    #[test]
    fn learn_unique_characters() {
        let mut state = BuildState::new("abc");
        state = state.run_until_end();
        assert_eq!(vec![("a", 1), ("b", 1), ("c", 1)], state.ledger.entries());
    }

    #[test]
    fn learn_substring() {
        let mut state = BuildState::new("abab");
        state = state.run_until_end();
        assert_eq!(vec![("ab", 1), ("a", 2), ("b", 2)], state.ledger.entries());
    }

    #[test]
    fn learn_several_substrings_step_by_step() {
        let mut state = BuildState::new("abcabcabc");

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
        let mut state = BuildState::new("犬猫魚鳥");
        state = state.run_until_end();
        assert_eq!(
            vec![("犬", 1), ("猫", 1), ("魚", 1), ("鳥", 1)],
            state.ledger.entries()
        );
    }

    #[test]
    fn merge_three_consecutive_substrings() {
        let mut state = BuildState::new("camelot");
        state.ledger.insert_new(substring("ca"));
        state.ledger.insert_new(substring("me"));
        state.ledger.insert_new(substring("lot"));

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

    fn substring(s: &str) -> Substring {
        Substring::from_str(s)
    }
}
