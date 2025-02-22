use super::{
    encoder_spec::EncoderSpec,
    substring::Substring,
    substring_ledger::{SubstringMap, SubstringSelector},
};

pub struct SelectByCompressionGain<'a> {
    encoder_spec: &'a EncoderSpec,
}

struct EncodingImpact {
    substring: Substring,
    compression_gain: usize,
}

impl<'a> SubstringSelector for SelectByCompressionGain<'a> {
    fn select_substrings(&self, substrings: SubstringMap) -> Vec<Substring> {
        let impacts = self.calculate_impacts(substrings);
        impacts
            .into_iter()
            .map(|impact| impact.substring)
            .take(self.encoder_spec.num_strings)
            .collect()
    }
}

impl<'a> SelectByCompressionGain<'a> {
    pub fn new(encoder_spec: &'a EncoderSpec) -> Self {
        Self { encoder_spec }
    }

    fn calculate_impacts(&self, substrings: SubstringMap) -> Vec<EncodingImpact> {
        let mut impacts: Vec<EncodingImpact> = substrings
            .into_iter()
            .map(|(substring, count)| {
                let compression_gain = self
                    .encoder_spec
                    .compression_gain(&substring.0, count as usize);
                EncodingImpact {
                    substring,
                    compression_gain,
                }
            })
            .filter(|impact| impact.compression_gain > 0)
            .collect();
        impacts.sort_by(|a, b| b.compression_gain.cmp(&a.compression_gain));
        impacts
    }
}
