use std::fs;
use std::time::Instant;
use text_compression::decode;
use text_compression::encode_with_policy;
use text_compression::policies::LimitLedgerSize;

const INPUT_FILENAME: &str = "test-data/wap-25600.txt";

fn main() {
    println!("* Compressing {}...", INPUT_FILENAME);
    let source = fs::read_to_string(INPUT_FILENAME).unwrap();
    let policy = LimitLedgerSize::with_max_size(32_768);

    let start = Instant::now();
    let (encoded, substrings, _) = encode_with_policy(&source, policy);
    let decoded = decode(&encoded, &substrings);
    assert_eq!(decoded, source);

    println!("* Finished in {:?}", start.elapsed());
}
