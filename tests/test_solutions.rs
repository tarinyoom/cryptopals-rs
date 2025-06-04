use cryptopals_rs::set_1::challenge_1;
use cryptopals_rs::set_1::challenge_2;

#[test]
fn test_challenge_1() {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let expected_output = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    assert_eq!(challenge_1::convert(input.to_string()).unwrap(), expected_output.to_string());
}

#[test]
fn test_challenge_2() {
    let input1 = "1c0111001f010100061a024b53535009181c";
    let input2 = "686974207468652062756c6c277320657965";
    let expected_output = "746865206b696420646f6e277420706c6179";
    assert_eq!(challenge_2::combine(input1.to_string(), input2.to_string()).unwrap(), expected_output.to_string());
}
