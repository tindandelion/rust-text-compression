use super::{Substring, SubstringLedger};

pub fn build_ledger(source: &str) -> SubstringLedger {
    let mut ledger = SubstringLedger::new();
    let mut head: &str = source;

    while head.len() > 0 {
        head = build_ledger_step(head, &mut ledger);
    }
    ledger
}

fn build_ledger_step<'a>(head: &'a str, ledger: &mut SubstringLedger) -> &'a str {
    if let Some(next_char) = head.chars().next() {
        let next_head: &'a str;

        if let Some(substr_match) = ledger.find_longest_match(head) {
            let rest = &head[substr_match.len()..];

            if let Some(follow_up_match) = ledger.find_longest_match(rest) {
                ledger.increment_count(&follow_up_match);

                let new_substring = substr_match.concat(&follow_up_match);
                next_head = &head[substr_match.len()..];
                ledger.insert_new(new_substring);
            } else {
                next_head = rest;
            }

            ledger.increment_count(&substr_match);
        } else {
            let new_substring = Substring::from_char(next_char);
            next_head = &head[new_substring.len()..];
            ledger.insert_new(new_substring);
        }
        next_head
    } else {
        ""
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn merge_three_consequent_substrings() {
        let mut ledger = SubstringLedger::new();
        ledger.insert_new(substring("ca"));
        ledger.insert_new(substring("me"));
        ledger.insert_new(substring("lot"));

        let source = "camelot";

        // Processing "ca" + "me" = "came"
        let next_head = build_ledger_step(source, &mut ledger);
        // Processing "me" + "lot" = "melot"
        build_ledger_step(next_head, &mut ledger);

        let expected = as_strings(vec!["ca", "me", "lot", "came", "melot"]);
        assert_eq!(as_set(expected), as_set(ledger.substrings()));
    }

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

    fn substring(s: &str) -> Substring {
        Substring::from_str(s)
    }
}
