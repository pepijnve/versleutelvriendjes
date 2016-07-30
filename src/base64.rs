const CHARS: [u8; 64] = [
    'A' as u8,
    'B' as u8,
    'C' as u8,
    'D' as u8,
    'E' as u8,
    'F' as u8,
    'G' as u8,
    'H' as u8,
    'I' as u8,
    'J' as u8,
    'K' as u8,
    'L' as u8,
    'M' as u8,
    'N' as u8,
    'O' as u8,
    'P' as u8,
    'Q' as u8,
    'R' as u8,
    'S' as u8,
    'T' as u8,
    'U' as u8,
    'V' as u8,
    'W' as u8,
    'X' as u8,
    'Y' as u8,
    'Z' as u8,
    'a' as u8,
    'b' as u8,
    'c' as u8,
    'd' as u8,
    'e' as u8,
    'f' as u8,
    'g' as u8,
    'h' as u8,
    'i' as u8,
    'j' as u8,
    'k' as u8,
    'l' as u8,
    'm' as u8,
    'n' as u8,
    'o' as u8,
    'p' as u8,
    'q' as u8,
    'r' as u8,
    's' as u8,
    't' as u8,
    'u' as u8,
    'v' as u8,
    'w' as u8,
    'x' as u8,
    'y' as u8,
    'z' as u8,
    '0' as u8,
    '1' as u8,
    '2' as u8,
    '3' as u8,
    '4' as u8,
    '5' as u8,
    '6' as u8,
    '7' as u8,
    '8' as u8,
    '9' as u8,
    '+' as u8,
    '/' as u8,
];

const EQUALS: u8 = '=' as u8;

pub fn to_base64(data: &[u8]) -> String {
    let byte_count = data.len();
    let char_count = (byte_count + 2) / 3 * 4;

    let mut byte_index = 0;
    let mut buffer: u8 = 0;
    let mut chars = Vec::<u8>::with_capacity(char_count);

    for element in data {
        match byte_index {
            0 => {
                chars.push(CHARS[(element >> 2) as usize]);
                buffer = (element & 0x3) << 4;
                byte_index = 1;
            }
            1 => {
                chars.push(CHARS[(buffer | (element >> 4)) as usize]);
                buffer = (element & 0xf) << 2;
                byte_index = 2;
            }
            2 => {
                chars.push(CHARS[(buffer | (element >> 6)) as usize]);
                chars.push(CHARS[(element & 0x3f) as usize]);
                buffer = 0;
                byte_index = 0;
            }
            _ => {
                panic!("Internal error");
            }
        }
    }

    match byte_index {
        0 => {}
        1 => {
            chars.push(CHARS[buffer as usize]);
            chars.push(EQUALS);
            chars.push(EQUALS);
        }
        2 => {
            chars.push(CHARS[buffer as usize]);
            chars.push(EQUALS);
        }
        _ => {
            panic!("Internal error");
        }
    }

    unsafe {
        String::from_utf8_unchecked(chars)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum FromBase64Error {
    /// The input contained a character not part of the hex format
    InvalidBase64Character(char, usize),
}

pub fn from_base64(enc: &str) -> Result<Vec<u8>,FromBase64Error> {
    let char_count = enc.len();
    let byte_count = (char_count + 3) / 4 * 3;

    let mut char_index = 0;
    let mut buffer: u8 = 0;
    let mut bytes = Vec::<u8>::with_capacity(byte_count);

    for (idx,c) in enc.chars().enumerate() {
        let c_val = c as u8;
        let b = match c {
            'A'...'Z' => (c_val - 'A' as u8),
            'a'...'z' => (c_val - 'a' as u8) + 26,
            '0'...'9' => (c_val - '0' as u8) + 52,
            '+' => 62,
            '/' => 63,
            '=' => {
                break;
            },
            _ => {
                return Err(FromBase64Error::InvalidBase64Character(c,idx));
            }
        };

        match char_index {
            0 => {
                buffer = b << 2;
                char_index = 1;
            }
            1 => {
                bytes.push(buffer | (b >> 4));
                buffer = (b & 0xf) << 4;
                char_index = 2;
            }
            2 => {
                bytes.push(buffer | (b >> 2));
                buffer = (b & 0x3) << 6;
                char_index = 3;
            }
            3 => {
                bytes.push(buffer | b);
                buffer = 0;
                char_index = 0;
            }
            _ => {
                panic!("Internal error");
            }
        }
    }

    Ok(bytes.into_iter().collect())
}