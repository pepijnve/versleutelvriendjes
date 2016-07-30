extern crate versleutelvriendjes;

use versleutelvriendjes::hamming;

#[test]
fn hamming_distance() {
    let a = "this is a test";
    let b = "wokka wokka!!!";
    let distance = hamming::distance(a.as_bytes(), b.as_bytes());

    assert_eq!(37, distance);
}