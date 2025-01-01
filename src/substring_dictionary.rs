pub struct SubstringDictionary {
    substrings: Vec<String>,
}

impl SubstringDictionary {
    pub fn new(substrings: Vec<String>) -> Self {
        Self { substrings }
    }

    pub fn find_match(&self, text: &str) -> Option<(usize, &String)> {
        self.substrings
            .iter()
            .enumerate()
            .find(|(_, substr)| text.starts_with(*substr))
    }

    pub fn len(&self) -> usize {
        self.substrings.len()
    }

    pub fn top(&self, n: usize) -> &[String] {
        &self.substrings[..n]
    }

    pub fn get(&self, index: usize) -> &String {
        &self.substrings[index]
    }

    #[cfg(test)]
    pub fn to_vec(&self) -> Vec<String> {
        self.substrings.clone()
    }
}
