//! Holds functions to help map between symbols and the codes used to represent them.
// Char in rust is 4 bytes long. char == u32.
const TABLE_SIZE: u8 = 31;
const ASCII_SIZE: u16 = 256;

pub struct Encoder {
    letters: [u8; ASCII_SIZE as usize], // Array that converts codes back into letters
    codes: [char; TABLE_SIZE as usize], // 5-bit short code 
}

impl Encoder {
    pub fn new() -> Self {
        let codes = [ //5-bit short code is its index within the codes array
            'e',
            't',
            'a',
            'i',
            '"',
            'n',
            ' ', //Space
            's',
            'o',
            'l',
            'r',
            'd',
            'c',
            '>',
            '<',
            '/',
            'p',
            'm',
            '-', //Dash
            'u',
            '.',
            'h',
            'f',
            '_', //Underscore
            '=',
            'g',
            ':', //Colon
            'b',
            '0', //Zero
            'y',
            '\n']; //New Line

        let mut letters = [0; ASCII_SIZE as usize];

        for i in 0..ASCII_SIZE {
            let mut pos: u8 = 0;
            let mut found: bool = false;

            while pos < TABLE_SIZE && !found {
                if char::from(i as u8) == codes[pos as usize] {
                    letters[i as usize] = pos;
                    found = true;
                }
                pos += 1;
            }

            if !found {
                letters[i as usize] = TABLE_SIZE;
            }
        }

        Encoder {
            codes: codes,
            letters: letters,
        }
    }

    /// Encode ASCII character as squeezed 5-bit code. If there is no 5-bit
    /// code to represent the input character, this function returns the escape
    /// code, 31.
    /// Returns a 5-bit code stored in `u8`.
    pub fn encode(&self, ch: char) -> u8 {
        if !char::is_ascii(&ch) {
            //TODO fix panic and support non-ascii characters.
            panic!("Non-ASCII Character Encoding Not Supported.")
        }

        let index = u32::from(ch);

        self.letters[index as usize]
    }

    /// Decode 5-bit code to corresponding ASCII character. This function is
    /// only defined for codes from [0, 30]. This function returns the escape
    /// code, 31 for characters outside the encoding range.
    /// Retuns character for the 5-bit code, or nothing if character is not 
    /// encoded.
    pub fn decode(&self, code: u8) -> Option<char> {
        if code >= TABLE_SIZE {
            None
        } else {
            Some(self.codes[code as usize])
        }
    }
}

#[cfg(test)]
mod test {
    use super::Encoder;

    #[test]
    fn encode_standard() {
        let encoder = Encoder::new();

        assert_eq!(encoder.encode('a'), 2);
    }

    #[test]
    fn decode_standard() {
        let encoder = Encoder::new();

        assert_eq!(encoder.decode(5), Some('n'));
    }
}
