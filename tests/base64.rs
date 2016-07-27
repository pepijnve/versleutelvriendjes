extern crate versleutelvriendjes;

#[test]
fn to_base64_padding_1() {
    let input = "any carnal pleasu";
    let base64 = versleutelvriendjes::base64::to_base64(input.as_bytes());
    let expected = "YW55IGNhcm5hbCBwbGVhc3U=";
    assert_eq!(expected, base64);
}

#[test]
fn to_base64_padding_2() {
    let input = "any carnal pleas";
    let base64 = versleutelvriendjes::base64::to_base64(input.as_bytes());
    let expected = "YW55IGNhcm5hbCBwbGVhcw==";
    assert_eq!(expected, base64);
}