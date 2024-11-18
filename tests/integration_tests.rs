use squeeze::run;

use std::fs::read;
use std::fs::File;
use std::io::{self, Write};

#[test]
fn sample_file_1() {
    let input = include_bytes!("./data/input-1.txt");
    let expected = include_bytes!("./data/compressed-1.bin");
    
    let input_path = "input-1.txt";
    let expected_path = "compressed-1.bin";

    create_file(input_path, input)
        .expect("Could not write input file.");
    create_file(expected_path, expected)
        .expect("Could not expected output file.");

    check_squeeze(input_path.to_string(), expected_path.to_string());
}

#[test]
fn sample_file_2() {
    let input = include_bytes!("./data/input-2.txt");
    let expected = include_bytes!("./data/compressed-2.bin");
    
    let input_path = "input-2.txt";
    let expected_path = "compressed-2.bin";

    create_file(input_path, input)
        .expect("Could not write input file.");
    create_file(expected_path, expected)
        .expect("Could not expected output file.");

    check_squeeze(input_path.to_string(), expected_path.to_string());
}

fn create_file(path: &str, contents: &[u8]) -> io::Result<()> {
    let mut file = File::create(path)?;

    file.write_all(contents)?;
    file.flush()
}

fn check_squeeze(input: String, expected: String) {
    // TODO pass in expected compressed output.
    let output_file = "output.bin";

    _ = run(input.clone(), output_file.to_string()).unwrap();

    let expected = read(expected)
        .expect("Could not open input file.");
    let actual = read(output_file)
        .expect("Could not open output file.");

    assert_eq!(expected.len(), actual.len());
    assert_eq!(expected, actual);
}
