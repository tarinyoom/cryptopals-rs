fn convert_chunk(chunk: &[u8], key: &[u8]) -> Vec<u8> {
    chunk.iter().zip(key.iter()).map(|(a, b)| a ^ b).collect()
}

fn fmt_hex_char(byte: u8) -> u8 {
    match byte {
        0..=9 => b'0' + byte,
        10..=15 => b'a' + byte - 10,
        _ => b'?',
    }
}

pub fn encrypt(msg: &str, key: &str) -> Result<String, String> {
    let msg_vec = msg.as_bytes();
    let key_vec = key.as_bytes();

    let key_len = key.len();

    // chunk against key length
    let chunks: Vec<&[u8]> = msg_vec.chunks(key_len).collect();

    // convert keys
    let converted: Vec<u8> = chunks
        .iter()
        .flat_map(|chunk| convert_chunk(&chunk, &key_vec))
        .collect();

    // split into hex chars
    let hex_converted: Vec<u8> = converted
        .iter()
        .flat_map(|x| vec![x >> 4, x & 0b1111])
        .map(fmt_hex_char)
        .collect();

    // UTF-8 encode result
    Ok(String::from_utf8(hex_converted).unwrap())
}
