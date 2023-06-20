//! Contains code for interacting with files.
use std::fs::File;
use std::io::Write;
use std::slice;

const BITS_PER_BYTE: u8 = 8;

const FIVE_BITS: u8 = 5; // Integer value for 5 bits
const EIGHT_BITS: u8 = 8; // Integer value for 8 bits (1 byte)
const HIGH_5_MASK: u8 = 0xF8u8; // Masks off the 5 higher order bits of char

const LOW_5_MASK: u8 =  0x0000001Fu8; // Lower 5 bits of int
const LOW_8_MASK: u8 = 0x000000FFu8; // Lower 8 bits of int
const HIGH_BIT: u8 = 0x80; // Mask for highest order bit of unsigned char

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

impl BitWriter {
    pub fn new(file: File) -> Self {
        BitWriter {
            buffer: BitBuffer::new(),
            file: file,
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
        // Only flush for non-empty buffer.
        if self.buffer.bcount > 0 {
            // Shift over the buffer by 8 - bitcount and dump into file. The 
            // left shift makes the unused bits zeros.
            self.buffer.bits = self.buffer.bits << (BITS_PER_BYTE - self.buffer.bcount);
            self.file.write(slice::from_ref(&self.buffer.bits)).unwrap();

            self.buffer.clear();
        }
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