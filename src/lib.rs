mod codes;
mod bits;

use codes::Encoder;
use codes::ESCAPE;
use bits::{BitReader, BitWriter};

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader};

const FORMAT_CODE: u8 = 1;
const SHORT_SIZE: u8 = 8;

pub fn run(input_file: String, output_file: String) -> Result<(), Box<dyn Error>> {
    // Attempt to open files and perform error checking.
    let input = File::open(input_file)
        .expect("Could not open input file.");

    let output = File::create(output_file)
        .expect("Could not open output file.");

    let writer = BitWriter::new(output);

    squeeze(input, writer);

    Ok(())
}

/// Implementation of the compression algorithm.
fn squeeze(input: File, mut writer: BitWriter) {
    let encoder = Encoder::new();

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

/// Implementation of decompression algorithm.
fn unsqueeze(output: File, mut reader:BitReader) {
    let encoder = Encoder::new();

    // Validate file format before decompression.
    let format_code = reader.read_five_bits().expect("Could not read input file.");

    if let Some(format_code) = format_code {
        if format_code != FORMAT_CODE {
            panic!("Invalid compressed format!\n");
        }
    }

    /*
    if (format_code != FORMAT_CODE) {
        fprintf(stderr, "Invalid compressed format\n");
        // Close file streams and exit
        fclose(input);
        fclose(output);
        exit(EXIT_FAILURE); //Exit with failure status
    }
     */

    //let mut short_code; // Stores 5 bits from input file.
    let mut byte: u8; // Stores 8 bit char ready to be output.

    // Read from input file until EOF is reached.
    while let Ok(Some(short_code)) = reader.read_five_bits() {
        // Attempt to decode character.
        let code = encoder.decode(short_code);

        match code {
            None => panic!("Unknown character encountered. Could not decompress."),
            Some(code) if code == ESCAPE as char => {
                // Read 8 bit char if escape char is detected.
                byte = reader.read_eight_bits().unwrap().expect("No bits found.");

                // If EOF is reached, flush buffer and leave.
                if byte < 0 {
                    //flushBits(buffer, input);
                    return;
                }
            },
            Some(code) => {
                byte = code as u8; // Char is ready to be written to output file.
            },
        }

        // Write to file.
        write!(&output, "{}", byte).expect("Error! Could not write to file.");
    }
}
