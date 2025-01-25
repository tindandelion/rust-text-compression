use std::fs;
use std::time::Instant;
use text_compression::decode;
use text_compression::encode_with_policy;
use text_compression::policies::CaptureAll;

const INPUT_FILENAME: &str = "test-data/wap-6400.txt";

fn main() {
    println!("* Compressing {}...", INPUT_FILENAME);
    let source = fs::read_to_string(INPUT_FILENAME).unwrap();

    let start = Instant::now();
    let (encoded, substrings) = encode_with_policy(&source, CaptureAll);
    let decoded = decode(&encoded, &substrings);
    assert_eq!(decoded, source);

    println!("* Finished in {:?}", start.elapsed());
}
