use std::str;
use std::string::*;

use charfreq::*;

pub fn decrypt_xor_single_byte_text(ref_freq_order: &str, encrypted: &[u8]) -> Option<String> {
    let decode_and_score = |data: &[u8]| -> Option<(u32,String)> {
        match str::from_utf8(data) {
            Ok(s) => {
                if is_printable(s) {
                    let score = calculate_score(ref_freq_order, s);
                    Option::Some((score, s.to_string()))
                } else {
                    Option::None
                }
            }
            _ => Option::None
        }
    };

    decrypt_xor_single_byte(encrypted, decode_and_score)
}

fn is_printable(text: &str) -> bool {
    for c in text.chars() {
        if c.is_control() && c != '\n' && c != '\r' && c != '\t' {
            return false;
        }
    }

    true
}

fn calculate_score(ref_freq_order: &str, text: &str) -> u32 {
    let freq_map = build_char_freq_map(text);
    let freq_order = build_freq_order_string(&freq_map);

    let mut score = 0;

    let range = 6;

    let most_frequent = &freq_order[0..range];
    for c in ref_freq_order[0..range].chars() {
        if most_frequent.contains(c) {
            score += 1;
        }
    }

    let least_frequent = &freq_order[freq_order.len() - range..freq_order.len() - 1];
    for c in ref_freq_order[ref_freq_order.len() - range..ref_freq_order.len() - 1].chars() {
        if least_frequent.contains(c) {
            score += 1;
        }
    }

    score
}

pub fn decrypt_xor_single_byte<F, R>(encrypted: &[u8], decode_and_score: F) -> Option<R>
    where F: Fn(&[u8]) -> Option<(u32, R)> {
    let mut best_value : Option<R> = Option::None;
    let mut best_score : Option<u32> =  Option::None;

    for b in 0..256 {
        let x = b as u8;
        let decrypted = xor(encrypted, x);

        match decode_and_score(&decrypted) {
            Some((score, value)) => {
                if best_score.is_none() || score > best_score.unwrap() {
                    best_value = Option::Some(value);
                    best_score = Option::Some(score);
                }
            }
            None => {}
        }
    }

    return best_value;
}

pub fn xor(encrypted: &[u8], xor: u8) -> Vec<u8> {
    let mut decrypted = Vec::<u8>::with_capacity(encrypted.len());

    for v in encrypted {
        decrypted.push(v ^ xor);
    }

    decrypted
}