fn char_to_hex(c: &u8) -> Result<u8, String> {
    match c {
        b'0'..=b'9' => Ok(c - b'0'),
        b'a'..=b'f' => Ok(c - b'a' + 10),
        b'A'..=b'F' => Ok(c - b'A' + 10),
        _ => Err(format!("Invalid hex char: {}", *c as char)),
    }
}

fn fmt_hex_char(byte: u8) -> u8 {
    match byte {
        0..=9 => b'0' + byte,
        10..=15 => b'a' + byte - 10,
        _ => b'?',
    }
}

fn fmt_hex(bytes: Vec<u8>) -> String {
    String::from_utf8(bytes.into_iter().map(fmt_hex_char).collect()).unwrap()
}

pub fn combine(l: String, r: String) -> Result<String, String> {
    let bytes_l: Vec<u8> = l
        .as_bytes()
        .iter()
        .map(char_to_hex)
        .collect::<Result<Vec<u8>, String>>()?;

    let bytes_r: Vec<u8> = r
        .as_bytes()
        .iter()
        .map(char_to_hex)
        .collect::<Result<Vec<u8>, String>>()?;

    let bytes_xor = bytes_l
        .iter()
        .zip(bytes_r.iter())
        .map(|(a, b)| a ^ b)
        .collect();

    Ok(fmt_hex(bytes_xor))
}
