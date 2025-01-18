use super::{Substring, SubstringLedger};

pub fn build_ledger(source: &str) -> SubstringLedger {
    let mut state = BuildState::new(source);

    while !state.at_end() {
        state = state.step();
    }
    state.ledger
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

#[cfg(test)]
mod learning_substrings_tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn learn_unique_chars() {
        let s = "abc";
        let expected = as_strings(vec!["a", "b", "c"]);

        let substrings = learn_substrings(s);
        assert_eq!(as_set(expected), as_set(substrings));
    }

    #[test]
    fn learn_substring() {
        let s = "ababab";
        let expected = as_strings(vec!["a", "b", "ab", "bab"]);
        let substrings = learn_substrings(s);
        assert_eq!(as_set(expected), as_set(substrings.to_vec()));
    }

    #[test]
    fn learn_several_substrings() {
        let s = "abcabcabc";
        let expected = as_strings(vec!["a", "b", "c", "ab", "bc", "cab", "abc"]);
        let substrings = learn_substrings(s);
        assert_eq!(as_set(expected), as_set(substrings.to_vec()));
    }

    #[test]
    fn learn_substrings_with_multi_byte_characters() {
        let s = "犬猫魚鳥";
        let expected = as_strings(vec!["犬", "猫", "魚", "鳥"]);
        let substrings = learn_substrings(s);
        assert_eq!(as_set(expected), as_set(substrings.to_vec()));
    }

    fn learn_substrings(s: &str) -> Vec<String> {
        build_ledger(s).substrings()
    }

    fn as_strings(v: Vec<&str>) -> Vec<String> {
        v.into_iter().map(|s| s.to_string()).collect()
    }

    fn as_set(v: Vec<String>) -> HashSet<String> {
        v.into_iter().collect()
    }
}
