use decoder::decode_string;
use encoder::encode;
use std::fs;
use std::time::Instant;

mod decoder;
mod encoder;

struct ExperimentResult {
    source_length_chars: usize,
    substrings: Vec<String>,
    compression_ratio: f32,
    time_elapsed: f32,
}

const INPUT_FILENAMES: &[&str] = &[
    "hamlet-100.txt",
    "hamlet-200.txt",
    "hamlet-400.txt",
    "hamlet-800.txt",
    "hamlet-1600.txt",
];

fn main() {
    println!("* Running experiments...");
    for filename in INPUT_FILENAMES {
        println!("File name: {}", filename);
        let result = run_experiment(filename);

        println!("Source length in chars: {}", result.source_length_chars);
        println!("Compression ratio: {:.2}%", result.compression_ratio);
        println!("Time elapsed: {:.2}s", result.time_elapsed);
        println!("Top 5 substrings: {:?}", result.substrings);
        println!("================================================");
    }
    println!("* Experiments finished.");
}

fn run_experiment(file_name: &str) -> ExperimentResult {
    let source = fs::read_to_string("test-data/".to_string() + file_name).unwrap();

    let start = Instant::now();
    let (encoded, substrings) = encode(&source);
    let end = Instant::now();
    let time_elapsed = end.duration_since(start).as_secs_f32();

    let decoded = decode_string(&encoded, &substrings);
    assert_eq!(decoded, source);

    let compression_ratio = (1.0 - (encoded.len() as f32 / source.len() as f32)) * 100.0;
    ExperimentResult {
        source_length_chars: source.len(),
        substrings: substrings[..5].to_vec(),
        compression_ratio,
        time_elapsed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_and_decode_ascii_string() {
        let source =
        "low low low low low lowest lowest newer newer newer newer newer newer wider wider wider new new"
            .to_string();

        let (encoded, substrings) = encode(&source);
        let decoded = decode_string(&encoded, &substrings);

        assert_eq!(decoded, source);
    }

    #[test]
    fn encode_and_decode_multibyte_string() {
        let source = "こんにちはこんにちは世界世界".to_string();

        let (encoded, substrings) = encode(&source);
        assert_eq!(10, substrings.len());

        let decoded = decode_string(&encoded, &substrings);
        assert_eq!(decoded, source);
    }
}
