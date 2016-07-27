use std::str;
use std::string::*;

pub fn decrypt_xor_single_byte_text<F>(encrypted: &[u8], score_text: F) -> Option<(String,u32)>
    where F: Fn(&str) -> Option<u32> {
    let decode_and_score = |data: &[u8]| -> Option<(String, u32)> {
        match str::from_utf8(data) {
            Ok(s) => {
                match score_text(s) {
                    Some(score) => {
                        Option::Some((s.to_string(), score))
                    }
                    _ => Option::None
                }
            }
            _ => Option::None
        }
    };

    decrypt_xor_single_byte(encrypted, decode_and_score)
}

pub fn decrypt_xor_single_byte<F, R>(encrypted: &[u8], decode_and_score: F) -> Option<(R,u32)>
    where F: Fn(&[u8]) -> Option<(R,u32)> {
    let mut best_result : Option<(R,u32)> = Option::None;

    for b in 0..256 {
        let x = [b as u8];
        let decrypted = xor(encrypted, &x);

        match decode_and_score(&decrypted) {
            Some(result) => {
                match best_result {
                    Some((_,best_score)) => {
                        let (_,score) = result;
                        if score > best_score {
                            best_result = Some(result);
                        }
                    }
                    None => {
                        best_result = Some(result);
                    }
                }
            }
            None => {}
        }
    }

    best_result
}

pub fn xor(encrypted: &[u8], xor: &[u8]) -> Vec<u8> {
    let mut decrypted = Vec::<u8>::with_capacity(encrypted.len());

    let mut xor_idx = 0;

    for v in encrypted {
        decrypted.push(v ^ xor[xor_idx]);
        xor_idx += 1;
        if xor_idx == xor.len() {
            xor_idx = 0;
        }
    }

    decrypted
}