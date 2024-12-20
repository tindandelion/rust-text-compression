use crate::substring_dictionary::SubstringDictionary;

pub fn learn_substrings(s: &str) -> Vec<String> {
    let mut dict = SubstringDictionary::new();
    let mut head: &str = s;
    while head.len() > 0 {
        if let Some(substr_match) = dict.find_longest_match(head) {
            if let Some(following_string) = dict.find_longest_match(&head[substr_match.len()..]) {
                let new_string = substr_match.clone() + &following_string;
                head = &head[new_string.len()..];
                dict.insert_new(&new_string);
            } else {
                head = &head[substr_match.len()..];
            }

            dict.increment_count(&substr_match);
        } else {
            dict.insert_new(&head[0..1]);
            head = &head[1..];
        }
    }
    dict.values().iter().map(|s| s.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn learn_unique_chars() {
        let s = "abc";
        let expected = vec!["a", "b", "c"];
        let substrings = learn_substrings(s);
        assert_eq!(expected, substrings);
    }

    #[test]
    fn learn_substring() {
        let s = "ababab";
        let expected = vec!["ab", "a", "b"];
        let substrings = learn_substrings(s);
        assert_eq!(expected, substrings);
    }

    #[test]
    fn learn_several_substrings() {
        let s = "abcabcabc";
        let expected = vec!["cab", "ab", "a", "b", "c"];
        let substrings = learn_substrings(s);
        assert_eq!(expected, substrings);
    }
}
