extern crate versleutelvriendjes;

#[test]
fn to_base64() {
    let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let bytes = versleutelvriendjes::hex::from_hex(&hex).unwrap();

    let base64 = versleutelvriendjes::base64::to_base64(&bytes);
    let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    assert_eq!(expected, base64);
}