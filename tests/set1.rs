extern crate versleutelvriendjes;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::u32;

use versleutelvriendjes::base64;
use versleutelvriendjes::charfreq;
use versleutelvriendjes::hex;
use versleutelvriendjes::xor;

#[test]
fn challenge1() {
    let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let bytes = hex::from_hex(&hex).unwrap();

    let base64 = base64::to_base64(&bytes);
    let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    assert_eq!(expected, base64);
}

#[test]
fn challenge2() {
    let input = hex::from_hex("1c0111001f010100061a024b53535009181c").unwrap();
    let key = hex::from_hex("686974207468652062756c6c277320657965").unwrap();
    let xor = xor::xor(&input, &key);

    let output = hex::to_hex(&xor);

    let expected = "746865206b696420646f6e277420706c6179";
    assert_eq!(expected, output);
}

#[test]
fn challenge3() {
    let input = hex::from_hex("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736").unwrap();
    let (decoded,_) = xor::decrypt_xor_single_byte_text(&input, charfreq::score_english_text).unwrap();

    let expected = "Cooking MC's like a pound of bacon";

    assert_eq!( decoded, expected );
}

#[test]
fn challenge4() {
    let f = File::open("tests/set1challenge4.txt").unwrap();
    let reader = BufReader::new(f);

    let mut results = Vec::new();

    for line in reader.lines() {
        let hex = line.unwrap();
        let input = hex::from_hex(&hex).unwrap();
        match xor::decrypt_xor_single_byte_text(&input, charfreq::score_english_text) {
            Some((text,score)) => {
                results.push((text, score));
            },
            None => {}
        }
    }

    results.sort_by_key(|text_score| {
        let (_,score) = *text_score;
        u32::MAX - score
    });
    let (ref best_result,_) = results[0];

    assert_eq!("Now that the party is jumping\n", best_result);
}

#[test]
fn challenge5() {
    let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let key = "ICE";

    let encoded = xor::xor(input.as_bytes(), key.as_bytes());
    let hex = hex::to_hex(&encoded);

    assert_eq!(hex, "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");
}