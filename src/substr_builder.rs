use crate::substring_dictionary::{EncoderSpec, SubstringDictionary};

pub fn learn_substrings(s: &str, encoder_spec: &EncoderSpec) -> Vec<String> {
    let dict = build_substring_dictionary(s);
    dict.get_most_impactful_strings(&encoder_spec)
        .iter()
        .map(|s| s.to_string())
        .collect()
}

pub fn build_substring_dictionary(s: &str) -> SubstringDictionary {
    let mut dict = SubstringDictionary::new();
    let mut head: &str = s;

    while head.len() > 0 {
        if let Some(substr_match) = dict.find_longest_match(head) {
            let rest = &head[substr_match.len()..];
            if let Some(follow_up_match) = dict.find_longest_match(rest) {
                dict.increment_count(&follow_up_match);

                let new_string = substr_match.clone() + &follow_up_match;
                head = &head[new_string.len()..];
                dict.insert_new(&new_string);
            } else {
                head = &head[substr_match.len()..];
            }

            dict.increment_count(&substr_match);
        } else {
            // TODO: Using unwrap()
            let char = head.chars().next().unwrap();
            dict.insert_new(&char.to_string());
            head = &head[char.len_utf8()..];
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
