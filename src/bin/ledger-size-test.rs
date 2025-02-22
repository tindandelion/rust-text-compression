use std::fs;
use std::time::Instant;
use text_compression::decode;
use text_compression::encode;
use text_compression::policies::CaptureAll;
use text_compression::policies::LimitLedgerSize;
use text_compression::SubstringSelector;
use text_compression::ENCODER_SPEC;

struct UsageStats {
    unused_entries: Vec<String>,
    table_size: usize,
}

impl ToString for UsageStats {
    fn to_string(&self) -> String {
        let used_count = self.table_size - self.unused_entries.len();
        format!("{}/{}", used_count, self.table_size)
    }
}
struct ExperimentResult {
    ledger_size: usize,
    top_10: Vec<String>,
    bottom_10: Vec<String>,
    usage_stats: UsageStats,
    compression_ratio: f32,
    time_elapsed: f32,
}

impl ExperimentResult {
    fn print(&self) {
        println!("Time elapsed: {:.2}s", self.time_elapsed);
        println!("Learned substring ledger size: {}", self.ledger_size);
        println!("Compression ratio: {:.2}%", self.compression_ratio);
        println!("Top 10 substrings: {:?}", self.top_10);
        println!("Bottom 10 substrings: {:?}", self.bottom_10);
        println!("Used entries: {}", self.usage_stats.to_string());
        println!("Unused substrings: {:?}", self.usage_stats.unused_entries);
    }
}
const INPUT_FILENAME: &str = "wap-25600.txt";
const LEDGER_SIZE_FACTORS: &[usize] = &[1, 2, 4, 8, 16, 32, 64, 128, 256];

fn main() {
    let selector = SubstringSelector::order_by_frequency(ENCODER_SPEC.encoded_size);
    println!("* Running baseline experiment (CaptureAll)...");
    let baseline_result = run_baseline(&selector);
    baseline_result.print();
    println!("================================================\n\n\n");

    println!("* Running ledger size experiments...");
    for factor in LEDGER_SIZE_FACTORS {
        let ledger_size = ENCODER_SPEC.num_strings * factor;
        println!("Max ledger size: {}", ledger_size);
        let result = run_experiment(ledger_size, &selector);
        result.print();
        println!("================================================");
    }
    println!("* Experiments finished.");
}

fn run_baseline(selector: &SubstringSelector) -> ExperimentResult {
    let source = read_source_file();

    let start = Instant::now();
    let (encoded, substrings, ledger_size) = encode(&source, CaptureAll, &selector);
    let time_elapsed = start.elapsed().as_secs_f32();

    let decoded = decode(&encoded, &substrings);
    assert_eq!(decoded, source);

    let compression_ratio = calc_compression_ratio(&source, &encoded);
    ExperimentResult {
        ledger_size,
        top_10: substrings.top(10),
        bottom_10: substrings.bottom(10),
        usage_stats: UsageStats {
            table_size: substrings.len(),
            unused_entries: substrings.unused_entries(),
        },
        compression_ratio,
        time_elapsed,
    }
}

fn run_experiment(ledger_size: usize, selector: &SubstringSelector) -> ExperimentResult {
    let source = read_source_file();
    let policy = LimitLedgerSize::with_max_size(ledger_size);

    let start = Instant::now();
    let (encoded, substrings, ledger_size) = encode(&source, policy, selector);
    let time_elapsed = start.elapsed().as_secs_f32();

    let decoded = decode(&encoded, &substrings);
    assert_eq!(decoded, source);

    let compression_ratio = calc_compression_ratio(&source, &encoded);
    ExperimentResult {
        ledger_size,
        top_10: substrings.top(10),
        bottom_10: substrings.bottom(10),
        usage_stats: UsageStats {
            table_size: substrings.len(),
            unused_entries: substrings.unused_entries(),
        },
        compression_ratio,
        time_elapsed,
    }
}

fn read_source_file() -> String {
    fs::read_to_string("test-data/".to_string() + INPUT_FILENAME).unwrap()
}

fn calc_compression_ratio(source: &str, encoded: &[u8]) -> f32 {
    (1.0 - (encoded.len() as f32 / source.len() as f32)) * 100.0
}
