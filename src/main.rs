mod codes;
mod bits;

use codes::Encoder;
use codes::ESCAPE;
use bits::BitWriter;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::process;

const EXPECTED_ARG_NUM: usize = 3;
const FORMAT_CODE: u8 = 1;
const SHORT_SIZE: u8 = 8;

fn main() {
    /* Check for valid number of command line arguments */
    let args: Vec<String> = env::args().collect();
    if args.len() != EXPECTED_ARG_NUM {
        // Print error message for invalid number of command-line arguments.
        eprintln!("usage: squeeze <infile> <outfile>");
        process::exit(2);
    }

    /* Attempt to open files and perform error checking. */
    let input = File::open(&args[1])
        .expect("Could not open input file.");

    let output = File::create(&args[2])
        .expect("Could not open output file.");
    let writer = BitWriter::new(output);

    let encoder = Encoder::new();

    squeeze(input, writer, encoder);
}

/// Implementation of the compression algorithm.
fn squeeze(input: File, mut writer: BitWriter, encoder: Encoder) {
    /* Write the format code before compression. */
    writer.write_five_bits(FORMAT_CODE);

    /* Read from input file until EOF is reached. */
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

    /* Flush buffer after compression is finished. */
    writer.flush();
}
