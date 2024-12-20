use std::collections::HashMap;

pub fn build_dictionary(s: &str) -> HashMap<String, u32> {
    let mut dictionary = HashMap::new();
    let mut head: &str = s;
    while head.len() > 0 {
        if let Some(substr_match) = find_longest_match(head, &dictionary) {
            let count = dictionary.get_mut(&substr_match).unwrap();
            *count += 1;

            if let Some(following_string) =
                find_longest_match(&head[substr_match.len()..], &dictionary)
            {
                let new_string = substr_match.clone() + &following_string;
                head = &head[new_string.len()..];
                dictionary.insert(new_string, 1);
            } else {
                head = &head[substr_match.len()..];
            }
        } else {
            dictionary.insert(head[0..1].to_string(), 1);
            head = &head[1..];
        }
    }
    dictionary
}

fn find_longest_match(s: &str, dictionary: &HashMap<String, u32>) -> Option<String> {
    let mut keys: Vec<_> = dictionary.keys().collect();
    keys.sort_by(|a, b| (b.len()).cmp(&a.len()));
    let x = keys.iter().find(|&&k| s.starts_with(k));
    x.map(|s| s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_single_char() {
        let s = "a";
        let expected = HashMap::from([("a".to_string(), 1)]);
        let dictionary = build_dictionary(s);
        assert_eq!(expected, dictionary);
    }

    #[test]
    fn encode_two_consecutive_chars() {
        let s = "aab";
        let expected = HashMap::from([("a".to_string(), 2), ("b".to_string(), 1)]);
        let dictionary = build_dictionary(s);
        assert_eq!(expected, dictionary);
    }

    #[test]

    fn encode_three_consecutive_chars() {
        let s = "aaab";
        let expected = HashMap::from([
            ("a".to_string(), 2),
            ("aa".to_string(), 1),
            ("b".to_string(), 1),
        ]);
        let dictionary = build_dictionary(s);
        assert_eq!(expected, dictionary);
    }

    #[test]
    fn find_longest_match_when_found() {
        let map = HashMap::from([
            ("a".to_string(), 3),
            ("aa".to_string(), 2),
            ("aaa".to_string(), 1),
            ("b".to_string(), 1),
        ]);

        let found = find_longest_match("aaa", &map);
        assert_eq!(Some("aaa".to_string()), found);

        let found = find_longest_match("bba", &map);
        assert_eq!(Some("b".to_string()), found);
    }

    #[test]
    fn find_longest_match_when_not_found() {
        let map = HashMap::from([
            ("a".to_string(), 3),
            ("aa".to_string(), 2),
            ("aaa".to_string(), 1),
            ("b".to_string(), 1),
        ]);

        let found = find_longest_match("ccc", &map);
        assert_eq!(None, found);
    }
}
