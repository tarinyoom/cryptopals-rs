use cryptopals_rs::set_1::*;

#[test]
fn test_challenge_1() {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let expected_output = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    assert_eq!(
        challenge_1::convert(input.to_string()).unwrap(),
        expected_output.to_string()
    );
}

#[test]
fn test_challenge_2() {
    let input1 = "1c0111001f010100061a024b53535009181c";
    let input2 = "686974207468652062756c6c277320657965";
    let expected_output = "746865206b696420646f6e277420706c6179";
    assert_eq!(
        challenge_2::combine(input1.to_string(), input2.to_string()).unwrap(),
        expected_output.to_string()
    );
}

#[test]
fn test_challenge_3() {
    let input1 = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let expected_output = "Cooking MC's like a pound of bacon";
    assert_eq!(
        challenge_3::decode(input1).unwrap(),
        expected_output.to_string()
    );
}

#[test]
fn test_challenge_4() {
    let input = include_str!("challenge_4.txt");
    let expected_output = "Now that the party is jumping\n";
    assert_eq!(
        challenge_4::decode(input).unwrap(),
        expected_output.to_string()
    );
}

#[test]
fn test_challenge_5() {
    let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let key = "ICE";
    let expected_output = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
    assert_eq!(
        challenge_5::encrypt(input, key).unwrap(),
        expected_output.to_string()
    );
}
