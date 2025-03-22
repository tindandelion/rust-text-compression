use std::fs;
use std::time::Instant;
use text_compression::decode;
use text_compression::encode;

const INPUT_FILENAME: &str = "test-data/war-and-peace-quad.txt";

fn main() {
    println!("* Compressing {}...", INPUT_FILENAME);
    let source = fs::read_to_string(INPUT_FILENAME).unwrap();

    let start = Instant::now();
    let (encoded, substrings) = encode(&source);
    let decoded = decode(&encoded, &substrings);
    assert_eq!(decoded, source);

    println!("* Finished in {:?}", start.elapsed());
}
