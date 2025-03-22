use std::fs;

use text_compression::{decode, encode};

#[test]
fn compression_ratio_test() {
    let test_files = [
        ("wap-100.txt", 35.9),
        ("wap-200.txt", 55.5),
        ("wap-400.txt", 72.9),
        ("wap-800.txt", 83.2),
        ("wap-1600.txt", 56.5),
        ("wap-3200.txt", 54.0),
        ("wap-6400.txt", 52.3),
        ("wap-12800.txt", 51.6),
    ];

    for (file_name, expected_ratio) in test_files {
        let source = read_test_file(file_name);
        let (encoded, substrings) = encode(&source);
        let decoded = decode(&encoded, &substrings);

        assert_eq!(
            decoded, source,
            "Decoded content does not match encoded on file {}",
            file_name
        );

        let compression_ratio = (1.0 - (encoded.len() as f32 / source.len() as f32)) * 100.0;
        assert!(
            compression_ratio >= expected_ratio,
            "Compression ratio is less than expected on file {}, expected {}, got {}",
            file_name,
            expected_ratio,
            compression_ratio
        );
    }
}

fn read_test_file(filename: &str) -> String {
    let file_path = format!("tests/data/{}", filename);
    fs::read_to_string(file_path).unwrap()
}
