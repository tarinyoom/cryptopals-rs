use cryptopals_rs::hello_world;

#[test]
fn test_challenge_1() {
    assert_eq!(hello_world(), "Hello, world!");
}
