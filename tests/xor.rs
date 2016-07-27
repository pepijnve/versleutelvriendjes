extern crate versleutelvriendjes;

use versleutelvriendjes::charfreq::*;
use versleutelvriendjes::hex::*;
use versleutelvriendjes::xor::*;

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

#[test]
fn challenge3() {
    let decoded = decode("f4f4ffbdb1e5e6f4f4bdb1f5e3f8f4f8f4f8f4f8f4f8f4f8f4f8f4f8f4f8f4f8f4f8f4f8f4f8f4f8f4");
    let expected = "een, twee, drieieieieieieieieieieieieieie";

    assert_eq!( decoded, expected );
}

#[test]
fn challenge4() {
    let decoded = decode("31161d53001217161016531c1f170618b0d412530609061d531c1f121d53111a015310b0cf1e1f165f53b0d41c185300b0cf001fb0cf53111a01532c160a5314160116181a01");
    let expected = "Ben sadece oldukça uzun olan bir cümle, çok süslü bir _ey gerekir";

    assert_eq!( decoded, expected );
}

#[test]
fn challenge5() {
    let decoded = decode("231c07b6ea16110619065319161d5305b7e807065f5318071601b0d253191653171c0007120716b7fe1db7e853171f1c061bb0d25f531d1a10531e1c105315121d0712091a16");
    let expected = "Potřebuju jen větu, která je dostatečně dlouhá, nic moc fantazie";

    assert_eq!( decoded, expected );
}

fn decode(encoded: &str) -> String {
    let input = from_hex(encoded).unwrap();
    let (text,_) = decrypt_xor_single_byte_text(&input, score_english_text).unwrap();
    text
}