extern crate versleutelvriendjes;

use versleutelvriendjes::hex::*;
use versleutelvriendjes::xor_decrypt::*;

#[test]
fn exc3() {
    let decoded = decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    let expected = "Cooking MC's like a pound of bacon";

    assert_eq!( decoded, expected );
}

#[test]
fn challenge1() {
    let decoded = decode("261a071712141a1d145f5307121a1f1c011e1217165304121d0753000316101a12121f53051c1c0153191c06530316031a191d5d53381d1203531416051c1d17161d5305121d53191c065d5334011212145314161712121d5d");
    let expected = "Uitdaging, tailormade want speciaal voor jou pepijn. Knap gevonden van jou. Graag gedaan.";

    assert_eq!( decoded, expected );
}

#[test]
fn challenge2() {
    let decoded = decode("0412011d1a1d1449530512011a12111f1653131012032c101c061d0713531a00531200001a141d161753071c5f53110607531d1605160153060016175f5350280412011d5b061d060016172c0512011a12111f16005a2e531c1d53110a5317161512061f075d");
    let expected = "warning: variable `cap_count` is assigned to, but never used, #[warn(unused_variables)] on by default.";

    assert_eq!( decoded, expected );
}

fn decode(encoded: &str) -> String {
    let english_freq_map = "etaoinshrdlcumwfgypbvkjxqz";
    let input = from_hex(encoded).unwrap();
    decrypt_xor_single_byte_text(&english_freq_map, &input)
}