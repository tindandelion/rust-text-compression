use encoder::encode_string;
use substr_builder::{clean_short_substrings, learn_substrings};

mod encoder;
mod substr_builder;
mod substring_dictionary;

fn main() {
    let s =
        "low low low low low lowest lowest newer newer newer newer newer newer wider wider wider new new"
            .to_string();
    let mut substrings = learn_substrings(&s);
    clean_short_substrings(&mut substrings);

    let encoded = encode_string(&s, &substrings);
    println!(
        "Original size: {}, encoded size: {}",
        s.bytes().len(),
        encoded.len()
    );
    println!("{:?}", encoded);
}
