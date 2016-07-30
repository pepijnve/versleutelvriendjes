extern crate versleutelvriendjes;

use versleutelvriendjes::base64;

#[test]
fn to_base64_padding_1() {
    let input = "any carnal pleasu";
    let base64 = base64::to_base64(input.as_bytes());
    let expected = "YW55IGNhcm5hbCBwbGVhc3U=";
    assert_eq!(expected, base64);
}

#[test]
fn to_base64_padding_2() {
    let input = "any carnal pleas";
    let base64 = base64::to_base64(input.as_bytes());
    let expected = "YW55IGNhcm5hbCBwbGVhcw==";
    assert_eq!(expected, base64);
}

#[test]
fn from_base64_padding_1() {
    let input = "YW55IGNhcm5hbCBwbGVhc3U=";
    let bytes = base64::from_base64(input).unwrap();
    let decoded = String::from_utf8(bytes).unwrap();
    let expected = "any carnal pleasu";
    assert_eq!(expected, decoded);
}

#[test]
fn from_base64_padding_2() {
    let input = "YW55IGNhcm5hbCBwbGVhcw==";
    let bytes = base64::from_base64(input).unwrap();
    let decoded = String::from_utf8(bytes).unwrap();
    let expected = "any carnal pleas";
    assert_eq!(expected, decoded);
}