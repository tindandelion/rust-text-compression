use std::fs;
use std::time::Instant;
use text_compression::decoder::decode_string;
use text_compression::encoder::encode;

const INPUT_FILENAME: &str = "test-data/hamlet-800.txt";

fn main() {
    println!("* Compressing {}...", INPUT_FILENAME);
    let source = fs::read_to_string(INPUT_FILENAME).unwrap();

    let start = Instant::now();
    let (encoded, substrings) = encode(&source);
    let decoded = decode_string(&encoded, &substrings);
    assert_eq!(decoded, source);

    println!("* Finished in {:?}", start.elapsed());
}
