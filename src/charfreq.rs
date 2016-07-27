use std::collections::BTreeMap;
use std::usize;

pub fn build_char_freq_map(text: &str) -> BTreeMap<char, usize> {
    let mut map = BTreeMap::new();

    for c in 'a' as u8 .. ('z' as u8) + 1 {
        map.insert(c as char, 0);
    }

    for c in text.chars() {
        match c {
            'a'...'z' | 'A' ... 'Z'  => {
                let lookup_char = c.to_lowercase().next().unwrap();
                *map.entry(lookup_char).or_insert(0) += 1;
            },
            _ => {}
        }
    }

    map
}

pub fn build_freq_order_string(freq_map: &BTreeMap<char, usize>) -> String {
    let mut order = Vec::<char>::with_capacity(freq_map.len());
    for c in freq_map.keys() {
        order.push(*c);
    }

    let default_freq = 0;
    order.sort_by_key(|c| usize::MAX - freq_map.get(c).unwrap_or(&default_freq));

    let mut freq_order_string = String::with_capacity(order.len());

    for c in order.iter() {
        freq_order_string.push(*c);
    }

    freq_order_string
}

static ENGLISH_CHAR_FREQUENCY_ORDER: &'static str = "etaoinshrdlcumwfgypbvkjxqz";

pub fn score_english_text(text: &str) -> Option<u32> {
    score_text(text, ENGLISH_CHAR_FREQUENCY_ORDER)
}

pub fn score_text(text: &str, ref_freq_order: &str) -> Option<u32> {
    if !is_printable(text) {
        return Option::None;
    }

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

    Option::Some(score)
}

fn is_printable(text: &str) -> bool {
    for c in text.chars() {
        if c.is_control() && c != '\n' && c != '\r' && c != '\t' {
            return false;
        }
    }

    true
}