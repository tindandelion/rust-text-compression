pub struct EncoderSpec {
    pub num_strings: usize,
    pub encoded_size: usize,
}

impl EncoderSpec {
    pub fn compression_gain(&self, string: &str, count: usize) -> usize {
        let unencoded_total_size = string.len() * count;
        let encoded_total_size = self.encoded_size * count;
        unencoded_total_size
            .checked_sub(encoded_total_size)
            .unwrap_or(0)
    }
}
