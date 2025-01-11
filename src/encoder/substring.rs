use std::cmp::Ordering;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Substring(pub(crate) String);

impl Substring {
    pub fn from_char(c: char) -> Self {
        Self(c.to_string())
    }

    #[cfg(test)]
    pub fn from_str(s: &str) -> Self {
        Self(s.to_string())
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn order_substrings_by_length_descending() {
        let mut substrings = vec![
            Substring("abc".to_string()),
            Substring("bc".to_string()),
            Substring("a".to_string()),
        ];

        substrings.sort();
        assert_eq!(vec!["abc", "bc", "a"], to_strings(&substrings));
    }

    #[test]
    fn order_substrings_of_same_length_lexicographically() {
        let mut substrings = vec![
            Substring("bcd".to_string()),
            Substring("abc".to_string()),
            Substring("xyz".to_string()),
        ];

        substrings.sort();
        assert_eq!(vec!["abc", "bcd", "xyz"], to_strings(&substrings));
    }

    fn to_strings(substrings: &Vec<Substring>) -> Vec<String> {
        substrings.iter().map(|s| s.to_string()).collect()
    }
}
