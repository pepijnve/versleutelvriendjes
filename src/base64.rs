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

    let remainder_bytes = byte_count % 3;
    let bulk_bytes = byte_count - remainder_bytes;

    let char_count = (byte_count + 2) / 3 * 4;

    let mut chars = Vec::<u8>::with_capacity(char_count);

    let mut i : usize = 0;

    while i < bulk_bytes {
        let b1 = data[i] as i32;
        let b2 = data[i + 1] as i32;
        let b3 = data[i + 2] as i32;

        let c1 = (b1 >> 2) & 0x3f;
        let c2 = ((b1 & 0x3) << 4) | ((b2 >> 4) & 0xf);
        let c3 = ((b2 & 0xf) << 2) | ((b3 >> 6) & 0x3);
        let c4 = b3 & 0x3f;

        chars.push(CHARS[c1 as usize]);
        chars.push(CHARS[c2 as usize]);
        chars.push(CHARS[c3 as usize]);
        chars.push(CHARS[c4 as usize]);

        i = i + 3;
    }

    match remainder_bytes {
        1 => {
            let b1 = data[i] as i32;

            let c1 = (b1 >> 2) & 0x3f;
            let c2 = (b1 & 0x3) << 4;

            chars.push(CHARS[c1 as usize]);
            chars.push(CHARS[c2 as usize]);
            chars.push(EQUALS);
            chars.push(EQUALS);
        }
        2 => {
            let b1 = data[i] as i32;
            let b2 = data[i + 1] as i32;

            let c1 = (b1 >> 2) & 0x3f;
            let c2 = ((b1 & 0x3) << 4) | ((b2 >> 4) & 0xf);
            let c3 = (b2 & 0xf) << 2;

            chars.push(CHARS[c1 as usize]);
            chars.push(CHARS[c2 as usize]);
            chars.push(CHARS[c3 as usize]);
            chars.push(EQUALS);
        }
        _ => {

        }
    }

    String::from_utf8(chars).unwrap()
}