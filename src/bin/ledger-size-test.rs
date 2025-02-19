use std::fs;
use std::time::Instant;
use text_compression::decode;
use text_compression::encode_with_policy;
use text_compression::policies::CaptureAll;
use text_compression::policies::LimitLedgerSize;
use text_compression::ENCODER_SPEC;

struct ExperimentResult {
    ledger_size: usize,
    top_10: Vec<String>,
    bottom_10: Vec<String>,
    compression_ratio: f32,
    time_elapsed: f32,
}

const INPUT_FILENAME: &str = "wap-25600.txt";
const LEDGER_SIZE_FACTORS: &[usize] = &[1, 2, 4, 8, 16, 32, 64, 128, 256];

fn main() {
    println!("* Running baseline experiment (CaptureAll)...");
    let baseline_result = run_baseline();
    println!("Substring ledger size: {}", baseline_result.ledger_size);
    println!(
        "Compression ratio: {:.2}%",
        baseline_result.compression_ratio
    );
    println!("Time elapsed: {:.2}s", baseline_result.time_elapsed);
    println!("Top 10 substrings: {:?}", baseline_result.top_10);
    println!("Bottom 10 substrings: {:?}", baseline_result.bottom_10);
    println!("================================================\n\n\n");

    println!("* Running experiments ledger size experiments...");
    for factor in LEDGER_SIZE_FACTORS {
        let ledger_size = ENCODER_SPEC.num_strings * factor;
        println!("Max ledger size: {}", ledger_size);
        let result = run_experiment(ledger_size);

        println!("Learned substring ledger size: {}", result.ledger_size);
        println!("Compression ratio: {:.2}%", result.compression_ratio);
        println!("Time elapsed: {:.2}s", result.time_elapsed);
        println!("Top 10 substrings: {:?}", result.top_10);
        println!("Bottom 10 substrings: {:?}", result.bottom_10);
        println!("================================================");
    }
    println!("* Experiments finished.");
}

fn run_baseline() -> ExperimentResult {
    let source = read_source_file();

    let start = Instant::now();
    let (encoded, substrings, ledger_size) = encode_with_policy(&source, CaptureAll);
    let time_elapsed = start.elapsed().as_secs_f32();

    let decoded = decode(&encoded, &substrings);
    assert_eq!(decoded, source);

    let compression_ratio = (1.0 - (encoded.len() as f32 / source.len() as f32)) * 100.0;
    ExperimentResult {
        ledger_size,
        top_10: substrings.top(10).to_vec(),
        bottom_10: substrings.bottom(10).to_vec(),
        compression_ratio,
        time_elapsed,
    }
}

fn run_experiment(ledger_size: usize) -> ExperimentResult {
    let source = read_source_file();
    let policy = LimitLedgerSize::with_max_size(ledger_size);

    let start = Instant::now();
    let (encoded, substrings, ledger_size) = encode_with_policy(&source, policy);
    let time_elapsed = start.elapsed().as_secs_f32();

    let decoded = decode(&encoded, &substrings);
    assert_eq!(decoded, source);

    let compression_ratio = (1.0 - (encoded.len() as f32 / source.len() as f32)) * 100.0;
    ExperimentResult {
        ledger_size,
        top_10: substrings.top(10).to_vec(),
        bottom_10: substrings.bottom(10).to_vec(),
        compression_ratio,
        time_elapsed,
    }
}

fn read_source_file() -> String {
    fs::read_to_string("test-data/".to_string() + INPUT_FILENAME).unwrap()
}
