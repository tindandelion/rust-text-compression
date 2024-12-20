use encoder::learn_substrings;

mod encoder;
mod substring_dictionary;

fn main() {
    let s =
        "low low low low low lowest lowest newer newer newer newer newer newer wider wider wider new new"
            .to_string();
    let dict = learn_substrings(&s);
    println!("{:?}", dict);
}
