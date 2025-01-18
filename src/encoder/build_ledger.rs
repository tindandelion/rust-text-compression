use super::{Substring, SubstringLedger};

pub fn build_ledger(source: &str) -> SubstringLedger {
    let mut state = BuildState {
        head: source,
        ledger: SubstringLedger::new(),
    };

    while state.head.len() > 0 {
        state = build_ledger_step(state);
    }
    state.ledger
}

struct BuildState<'a> {
    head: &'a str,
    ledger: SubstringLedger,
}

fn build_ledger_step<'a, 'b>(mut state: BuildState<'a>) -> BuildState<'a> {
    if let Some(next_char) = state.head.chars().next() {
        if let Some(substr_match) = state.ledger.find_longest_match(state.head) {
            state.ledger.increment_count(&substr_match);
            merge_with_follow_up_match(state, &substr_match)
        } else {
            create_single_char_substring(state, next_char)
        }
    } else {
        BuildState {
            head: "",
            ledger: state.ledger,
        }
    }
}

fn merge_with_follow_up_match<'a, 'b>(
    mut state: BuildState<'a>,
    substr_match: &Substring,
) -> BuildState<'a> {
    let rest = &state.head[substr_match.len()..];
    if let Some(follow_up_match) = state.ledger.find_longest_match(rest) {
        let new_substring = substr_match.concat(&follow_up_match);
        state.ledger.insert_new(new_substring);
    }

    BuildState {
        head: rest,
        ledger: state.ledger,
    }
}

fn create_single_char_substring<'a, 'b>(
    mut state: BuildState<'a>,
    next_char: char,
) -> BuildState<'a> {
    let new_substring = Substring::from_char(next_char);
    let rest = &state.head[new_substring.len()..];
    state.ledger.insert_new(new_substring);
    BuildState {
        head: rest,
        ledger: state.ledger,
    }
}

#[cfg(test)]
mod build_ledger_step_tests {
    use super::*;

    #[test]
    fn merge_three_consecutive_substrings() {
        let mut ledger = SubstringLedger::new();
        ledger.insert_new(substring("ca"));
        ledger.insert_new(substring("me"));
        ledger.insert_new(substring("lot"));

        let mut state = BuildState {
            head: "camelot",
            ledger,
        };

        // Processing "ca" + "me" = "came"
        state = build_ledger_step(state);
        assert_eq!(
            vec![("came", 1), ("lot", 1), ("ca", 2), ("me", 1)],
            state.ledger.entries()
        );

        // Processing "me" + "lot" = "melot"
        state = build_ledger_step(state);
        assert_eq!(
            vec![("melot", 1), ("came", 1), ("lot", 1), ("ca", 2), ("me", 2)],
            state.ledger.entries()
        );

        // Processing "lot"
        state = build_ledger_step(state);
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
