use std::cmp::Ordering;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Substring(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubstringCount {
    pub value: Substring,
    pub count: usize,
}

impl SubstringCount {
    pub fn new(value: Substring, count: usize) -> Self {
        Self { value, count }
    }
}

impl Substring {
    pub fn from_char(c: char) -> Self {
        Self(c.to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn concat(&self, other: &Substring) -> Substring {
        Substring(self.0.clone() + &other.0)
    }

    pub fn matches_start(&self, text: &str) -> bool {
        text.starts_with(&self.0)
    }
}

impl ToString for Substring {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl Ord for Substring {
    fn cmp(&self, other: &Self) -> Ordering {
        let by_length = (other.0.len()).cmp(&self.0.len());
        if by_length.is_eq() {
            self.0.cmp(&other.0)
        } else {
            by_length
        }
    }
}

impl PartialOrd for Substring {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<String> for Substring {
    fn from(s: String) -> Self {
        assert!(!s.is_empty(), "Cannot create Substring from empty string");
        Substring(s)
    }
}

impl From<&str> for Substring {
    fn from(s: &str) -> Self {
        assert!(!s.is_empty(), "Cannot create Substring from empty string");
        Substring(s.to_string())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    #[should_panic]
    fn substring_from_empty_str() {
        let _ = Substring::from("");
    }

    #[test]
    #[should_panic]
    fn substring_from_empty_string() {
        let _ = Substring::from("".to_string());
    }

    #[test]
    fn order_substrings_by_length_descending() {
        let mut substrings = vec![
            Substring::from("abc"),
            Substring::from("bc"),
            Substring::from("a"),
        ];

        substrings.sort();
        assert_eq!(vec!["abc", "bc", "a"], to_strings(&substrings));
    }

    #[test]
    fn order_substrings_of_same_length_lexicographically() {
        let mut substrings = vec![
            Substring::from("bcd"),
            Substring::from("abc"),
            Substring::from("xyz"),
        ];

        substrings.sort();
        assert_eq!(vec!["abc", "bcd", "xyz"], to_strings(&substrings));
    }

    fn to_strings(substrings: &Vec<Substring>) -> Vec<String> {
        substrings.iter().map(|s| s.to_string()).collect()
    }
}
