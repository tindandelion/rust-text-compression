use std::time::Instant;
use std::{error::Error, fs};
use text_compression::{decode, encode};

const INPUT_FILENAME: &str = "test-data/war-and-peace-quad.txt";

fn main() -> Result<(), Box<dyn Error>> {
    println!("* Compressing {}...", INPUT_FILENAME);
    let source = fs::read_to_string(INPUT_FILENAME)?;

    let start = Instant::now();
    let (encoded, substrings) = encode(&source);
    let decoded = decode(&encoded, &substrings)?;
    assert_eq!(decoded, source);

    println!("* Finished in {:?}", start.elapsed());
    Ok(())
}
