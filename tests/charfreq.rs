extern crate versleutelvriendjes;

use std::collections::BTreeMap;

use versleutelvriendjes::charfreq::*;

#[test]
fn char_freq() {
    let text = "aababcabcdabcdeabcdef";
    let map = build_char_freq_map(&text);

    let mut expected = BTreeMap::new();
    for c in 'a' as u8 .. 'z' as u8 + 1 {
        expected.insert(c as char, 0);
    }
    expected.insert('a', 6);
    expected.insert('b', 5);
    expected.insert('c', 4);
    expected.insert('d', 3);
    expected.insert('e', 2);
    expected.insert('f', 1);

    assert_eq!(expected, map);
}

#[test]
fn char_freq2() {
    let input = "uu~<0dguu<0tbyuyuyuyuyuyuyuyuyuyuyuyuyuyu";
    let actual = build_freq_order_string(&build_char_freq_map(&input));
    let expected = "uybdgtacefhijklmnopqrsvwxz";

    assert_eq!(expected, actual)
}