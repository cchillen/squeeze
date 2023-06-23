//! Contains code for interacting with files.
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::Write;
use std::slice;

use crate::ESCAPE;

const BITS_PER_BYTE: u8 = 8;

const FIVE_BITS: u8 = 5; // Integer value for 5 bits
const EIGHT_BITS: u8 = 8; // Integer value for 8 bits (1 byte)
const HIGH_5_MASK: u8 = 0xF8u8; // Masks off the 5 higher order bits of char

const LOW_5_MASK: u8 =  0x0000001Fu8; // Lower 5 bits of int
const LOW_8_MASK: u8 = 0x000000FFu8; // Lower 8 bits of int
const HIGH_BIT: u8 = 0x80; // Mask for highest order bit of unsigned char

const ESCAPE_BITS: u8 = 3;

struct BitBuffer {
    bits: u8,
    bcount: u8,
}

impl BitBuffer {
    pub fn new() -> Self {
        BitBuffer{
            bits: 0,
            bcount: 0,
        }
    }

    pub fn is_full(&self) -> bool {
        self.bcount == BITS_PER_BYTE
    }

    pub fn clear(&mut self) {
        self.bits = 0;
        self.bcount = 0;
    }
}

pub struct BitWriter {
    buffer: BitBuffer,
    file: File,
}

pub struct BitReader {
    buffer: BitBuffer,
    file: File,
}

impl BitWriter {
    pub fn new(file: File) -> Self {
        BitWriter {
            buffer: BitBuffer::new(),
            file,
        }
    }

    pub fn write_five_bits(&mut self, code: u8) {
        // Empty Buffer if it is full (8 bytes is a full buffer) */
        if self.buffer.is_full() {
            self.file.write(slice::from_ref(&self.buffer.bits)).unwrap();
            self.buffer.clear();
        }

        // Extract code to unsigned char from int
        let mut new_data = code & LOW_5_MASK;

        // Shift new Data such that everything is in higher order bits
        new_data = new_data << (EIGHT_BITS - FIVE_BITS);

        let mut copy_bit; // Used to copy bits from new_data to buffer

        // Add bit from new_data to buffer and print buffer when its full
        for _ in 0..FIVE_BITS {
            self.buffer.bits = self.buffer.bits << 1; //Shift buffer over by one

            copy_bit = HIGH_BIT & new_data; // Get highest order bit from new_data

            // Shift bit from highest order to lowest order
            copy_bit = copy_bit >> (EIGHT_BITS - 1);

            new_data &= !HIGH_BIT; //Clear highest order bit from new_data
            new_data = new_data << 1; //Shift new_data over by one

            self.buffer.bits = self.buffer.bits | copy_bit; //Copy new data to buffer
            self.buffer.bcount += 1; //Increment bit count

            // If buffer is full, write to file and clear buffer.
            if self.buffer.is_full() {
                self.file.write(slice::from_ref(&self.buffer.bits)).unwrap();
                self.buffer.clear();
            }
        }
    }

    pub fn write_eight_bits(&mut self, code: u8) {
        if self.buffer.is_full() {
            self.file.write(slice::from_ref(&self.buffer.bits)).unwrap();
            self.buffer.clear();
        }

        let mut new_data = code & LOW_8_MASK; // Extract code to unsigned char from int
        let mut copy_bit;

        /* Add bit from new_data to buffer and print buffer when its full */
        for _ in 0..EIGHT_BITS {
            self.buffer.bits = self.buffer.bits << 1; // Shift buffer over by one.

            copy_bit = HIGH_BIT & new_data; // Get highest order bit from new_data.
            copy_bit = copy_bit >> (EIGHT_BITS - 1); // Shift bit from higest order to lowest
                                                     // order.

            new_data &= !HIGH_BIT; // Clear highest order bit from new_data.
            new_data = new_data << 1; // Shift new_data over by one.

            self.buffer.bits = self.buffer.bits | copy_bit; // Copy new data to buffer
            self.buffer.bcount += 1; // Increment bit count.

            /* If buffer is full, write to file and clear buffer. */
            if self.buffer.is_full() {
                self.file.write(slice::from_ref(&self.buffer.bits)).unwrap();
                self.buffer.clear();
            }
        }
    }

    pub fn flush(&mut self) {
        // Do nothing if buffer is already empty.
        if self.buffer.bcount == 0 {
            return
        }

        // Write an extra escape character if there are 1-3 bits in buffer
        if self.buffer.bcount <= ESCAPE_BITS {
            self.write_five_bits(ESCAPE);
        }

        // Shift over the buffer by 8 - bitcount and dump into file. The 
        // left shift makes the unused bits zeros.
        self.buffer.bits = self.buffer.bits << (BITS_PER_BYTE - self.buffer.bcount);
        self.file.write(slice::from_ref(&self.buffer.bits)).unwrap();

        self.buffer.clear();
    }
}

impl BitReader {
    pub fn new(file: File) -> Self {
        BitReader {
            buffer: BitBuffer::new(),
            file,
        }
    }

    pub fn read_five_bits(&mut self) -> io::Result<Option<u8>> {
        let mut return_value: u8 = 0;

        // Return if buffer contains 5 or more bits.
        if self.buffer.bcount >= FIVE_BITS {
            // Shift buffer over all the way.
            self.buffer.bits = self.buffer.bits << (EIGHT_BITS - self.buffer.bcount);
            // Mask off the 5 higher order bits.
            return_value = self.buffer.bits & HIGH_5_MASK;
            // Shift return value back down.
            return_value = return_value >> (EIGHT_BITS - FIVE_BITS);
            // Clear the 5 higher order bits.
            self.buffer.bits = self.buffer.bits & !HIGH_5_MASK;
            // Shift buffer back to normal.
            self.buffer.bits = self.buffer.bits >> (EIGHT_BITS - self.buffer.bcount);
            // Decrement bit count by 5.
            self.buffer.bcount -= FIVE_BITS;

            return Ok(Some(return_value));
        }

        let mut new_data: [u8; 1] = [0; 1];

        // Read data from file.
        if self.file.read(&mut new_data)? == 0 {
            return Ok(None); // No more data to be read from file.
        }

        let mut output_found = false; // True if return value has been assigned.
        let mut copy_bit; // Used to copy bits from new_data to buffer.

        // Add byte from raw buffer to bit buffer and return value all data has been added.
        for _ in 0..EIGHT_BITS {
            self.buffer.bits = self.buffer.bits << 1; // Shift buffer over by one. 

            copy_bit = HIGH_BIT & new_data[0]; // Get highest order bit from new_data.
            copy_bit = copy_bit >> (EIGHT_BITS - 1); // Shift bit from highest to lowest order.

            new_data[0] &= !HIGH_BIT; // Clear highest order bit from new_data.
            new_data[0] = new_data[0] << 1; // Shift new_data over by one.

            self.buffer.bits = self.buffer.bits | copy_bit; // Copy new data to buffer.
            self.buffer.bcount += 1; // Increment bit count.

            // If buffer contains at least 5 bits, store return value and update buffer.
            if self.buffer.bcount >= FIVE_BITS && !output_found {
                output_found = true;
                return_value = self.buffer.bits;
                self.buffer.clear();
            }
        }

        Ok(Some(return_value)) // Return 5 bits.
    }

    pub fn read_eight_bits(&mut self) -> io::Result<Option<u8>> {
        Ok(None)
    }
}

#[cfg(test)]
mod test {
    use std::fs::File;
    use super::BitWriter;

    #[test]
    fn write5bits() {
        let file = File::create("test.txt").unwrap();
        let mut bit_writer = BitWriter::new(file);
        bit_writer.write_five_bits(0)
    }
}
