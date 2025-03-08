use std::fs;
use std::time::Instant;
use text_compression::decode;
use text_compression::encode_with_policy;
use text_compression::policies::LimitLedgerSize;

struct ExperimentResult {
    source_length_chars: usize,
    substrings: Vec<String>,
    compression_ratio: f32,
    time_elapsed: f32,
}

const LEDGER_SIZE: usize = 65_536;
const INPUT_FILENAMES: &[&str] = &[
    "wap-1600.txt",
    "wap-3200.txt",
    "wap-6400.txt",
    "wap-12800.txt",
    "wap-25600.txt",
    "war-and-peace.txt",
    "war-and-peace-dbl.txt",
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
    let (encoded, substrings, _) =
        encode_with_policy(&source, LimitLedgerSize::with_max_size(LEDGER_SIZE));
    let end = Instant::now();
    let time_elapsed = end.duration_since(start).as_secs_f32();

    let decoded = decode(&encoded, &substrings);
    assert_eq!(decoded, source);

    let compression_ratio = (1.0 - (encoded.len() as f32 / source.len() as f32)) * 100.0;
    ExperimentResult {
        source_length_chars: source.len(),
        substrings: substrings.top(5).to_vec(),
        compression_ratio,
        time_elapsed,
    }
}
