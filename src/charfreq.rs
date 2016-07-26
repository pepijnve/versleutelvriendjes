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