const CHARS: [u8; 16] = [
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
    'a' as u8,
    'b' as u8,
    'c' as u8,
    'd' as u8,
    'e' as u8,
    'f' as u8
];

pub fn to_hex(data: &[u8]) -> String {
    let mut chars = Vec::<u8>::with_capacity(data.len() * 2);

    for b in data {
        let val = *b as i32;
        let high = (val >> 4) & 0xF;
        let low = val & 0xF;
        chars.push(CHARS[high as usize]);
        chars.push(CHARS[low as usize]);
    }

    String::from_utf8(chars).unwrap()
}

#[derive(Copy, Clone, Debug)]
pub enum FromHexError {
    /// The input contained a character not part of the hex format
    InvalidHexCharacter(char, usize),
    /// The input had an invalid length
    InvalidHexLength,
}

pub fn from_hex(text: &str) -> Result<Vec<u8>, FromHexError> {
    let mut b = Vec::with_capacity(text.len() / 2);
    let mut modulus = 0;
    let mut buf = 0;

    for (idx, byte) in text.bytes().enumerate() {
        buf <<= 4;

        match byte {
            b'A'...b'F' => buf |= byte - b'A' + 10,
            b'a'...b'f' => buf |= byte - b'a' + 10,
            b'0'...b'9' => buf |= byte - b'0',
            _ => {
                let ch = text[idx..].chars().next().unwrap();
                return Err(FromHexError::InvalidHexCharacter(ch, idx))
            }
        }

        modulus += 1;
        if modulus == 2 {
            modulus = 0;
            b.push(buf);
        }
    }

    match modulus {
        0 => Ok(b.into_iter().collect()),
        _ => Err(FromHexError::InvalidHexLength),
    }
}