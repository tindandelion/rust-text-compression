use super::{encoder_spec::EncoderSpec, SubstringLedger};

pub fn learn_substrings(s: &str, encoder_spec: &EncoderSpec) -> Vec<String> {
    let dict = build_substring_ledger(s);
    dict.get_most_impactful_strings(&encoder_spec)
        .iter()
        .map(|s| s.to_string())
        .collect()
}

fn build_substring_ledger(source: &str) -> SubstringLedger {
    let mut dict = SubstringLedger::new();
    let mut head: &str = source;

    while let Some(next_char) = head.chars().next() {
        if let Some(substr_match) = dict.find_longest_match(head) {
            let rest = &head[substr_match.len()..];
            if let Some(follow_up_match) = dict.find_longest_match(rest) {
                dict.increment_count(&follow_up_match);

                let new_substring = substr_match.clone() + &follow_up_match;
                head = &head[new_substring.len()..];
                dict.insert_new(&new_substring);
            } else {
                head = &head[substr_match.len()..];
            }

            dict.increment_count(&substr_match);
        } else {
            dict.insert_new(&next_char.to_string());
            head = &head[next_char.len_utf8()..];
        }
    }
    dict
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    const ENCODER_SPEC: EncoderSpec = EncoderSpec {
        num_strings: 256,
        encoded_size: 0,
    };

    #[test]
    fn learn_unique_chars() {
        let s = "abc";
        let expected = as_strings(vec!["a", "b", "c"]);
        let substrings = learn_substrings(s, &ENCODER_SPEC);
        assert_eq!(as_set(expected), as_set(substrings));
    }

    #[test]
    fn learn_substring() {
        let s = "ababab";
        let expected = as_strings(vec!["ab", "a", "b"]);
        let substrings = learn_substrings(s, &ENCODER_SPEC);
        assert_eq!(as_set(expected), as_set(substrings));
    }

    #[test]
    fn learn_several_substrings() {
        let s = "abcabcabc";
        let expected = as_strings(vec!["cab", "ab", "a", "b", "c"]);
        let substrings = learn_substrings(s, &ENCODER_SPEC);
        assert_eq!(as_set(expected), as_set(substrings));
    }

    #[test]
    fn learn_substrings_with_multi_byte_characters() {
        let s = "犬猫魚鳥";
        let expected = as_strings(vec!["犬", "猫", "魚", "鳥"]);
        let substrings = learn_substrings(s, &ENCODER_SPEC);
        assert_eq!(as_set(expected), as_set(substrings));
    }

    fn as_strings(v: Vec<&str>) -> Vec<String> {
        v.into_iter().map(|s| s.to_string()).collect()
    }

    fn as_set(v: Vec<String>) -> HashSet<String> {
        v.into_iter().collect()
    }
}
