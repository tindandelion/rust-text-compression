use std::fs;
use std::time::Instant;
use text_compression::decode;
use text_compression::encode_with_policy;
use text_compression::policies::LimitDictionarySize;
use text_compression::ENCODER_SPEC;

struct ExperimentResult {
    ledger_size: usize,
    substrings: Vec<String>,
    compression_ratio: f32,
    time_elapsed: f32,
}

const INPUT_FILENAME: &str = "wap-25600.txt";
const LEDGER_SIZE_FACTORS: &[usize] = &[1, 2, 4, 8, 16, 32, 64, 128, 256];

fn main() {
    println!("* Running experiments...");
    for factor in LEDGER_SIZE_FACTORS {
        println!("Factor: {}", factor);
        let result = run_experiment(*factor);

        println!("Substring ledger size: {}", result.ledger_size);
        println!("Compression ratio: {:.2}%", result.compression_ratio);
        println!("Time elapsed: {:.2}s", result.time_elapsed);
        println!("Top 5 substrings: {:?}", result.substrings);
        println!("================================================");
    }
    println!("* Experiments finished.");
}

fn run_experiment(ledger_factor: usize) -> ExperimentResult {
    let source = fs::read_to_string("test-data/".to_string() + INPUT_FILENAME).unwrap();
    let ledger_size = ENCODER_SPEC.num_strings * ledger_factor;
    let policy = LimitDictionarySize::with_max_size(ledger_size);

    let start = Instant::now();
    let (encoded, substrings) = encode_with_policy(&source, policy);
    let end = Instant::now();
    let time_elapsed = end.duration_since(start).as_secs_f32();

    let decoded = decode(&encoded, &substrings);
    assert_eq!(decoded, source);

    let compression_ratio = (1.0 - (encoded.len() as f32 / source.len() as f32)) * 100.0;
    ExperimentResult {
        ledger_size,
        substrings: substrings.top(5).to_vec(),
        compression_ratio,
        time_elapsed,
    }
}
