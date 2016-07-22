extern crate versleutelvriendjes;

#[test]
fn to_hex() {
    let input: [u8; 5] = [1, 2, 3, 4, 234];
    let output = versleutelvriendjes::hex::to_hex(&input);
    let expected = "01020304ea";
    assert_eq!(expected, output);
}

#[test]
fn from_hex() {
    let input = "01020304ea";
    let output: &[u8] = &versleutelvriendjes::hex::from_hex(&input).unwrap();
    let expected: [u8; 5] = [1, 2, 3, 4, 234];
    assert_eq!(expected, output);
}