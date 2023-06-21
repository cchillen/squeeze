mod codes;
mod bits;

use codes::Encoder;
use codes::ESCAPE;
use bits::BitWriter;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const FORMAT_CODE: u8 = 1;
const SHORT_SIZE: u8 = 8;

pub fn run(input_file: String, output_file: String) -> Result<(), Box<dyn Error>> {
    // Attempt to open files and perform error checking.
    let input = File::open(input_file)
        .expect("Could not open input file.");

    let output = File::create(output_file)
        .expect("Could not open output file.");

    let writer = BitWriter::new(output);

    let encoder = Encoder::new();

    squeeze(input, writer, encoder);

    Ok(())
}

/// Implementation of the compression algorithm.
fn squeeze(input: File, mut writer: BitWriter, encoder: Encoder) {
    // Write the format code before compression.
    writer.write_five_bits(FORMAT_CODE);

    // Read entire input file and encode.
    let reader = BufReader::new(input);
    for byte in reader.bytes() {
        let unwrapped_byte = byte.unwrap();
        let char = char::from(unwrapped_byte);

        let code = encoder.encode(char);
        writer.write_five_bits(code);

        if code == ESCAPE {
            writer.write_eight_bits(unwrapped_byte);
        }
    }

    // Flush buffer after compression is finished.
    writer.flush();
}
