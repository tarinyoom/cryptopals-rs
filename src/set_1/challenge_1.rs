fn char_to_hex(c: &u8) -> Result<u8, String> {
    match c {
        b'0'..=b'9' => Ok(c - b'0'),
        b'a'..=b'f' => Ok(c - b'a' + 10),
        b'A'..=b'F' => Ok(c - b'A' + 10),
        _ => Err(format!("Invalid hex char: {}", *c as char))
    }
}

fn b64_to_char(v: u8) -> u8 {
    match v {
        0..=25 => b'A' + v,
        26..=51 => b'a' + (v - 26),
        52..=61 => b'0' + (v - 52),
        62 => b'+',
        63 => b'/',
        _ => b'?'
    }
}

fn convert_chunk(chunk: &[u8]) -> [u8; 2] {
    let left = (chunk[0] << 2) | (chunk[1] >> 2);
    let right = ((chunk[1] & 0b11) << 4) | chunk[2];
    [left, right]
}

pub fn convert(s: String) -> Result<String, String> {
    use itertools::Itertools;

    let hex_values: Vec<u8> = s.as_bytes()
        .iter()
        .map(char_to_hex)
        .collect::<Result<Vec<u8>, String>>()?;

    let b64_bytes: Vec<u8> = hex_values
        .into_iter()
        .chunks(3)
        .into_iter()
        .flat_map(|chunk| {
            let chunk: Vec<u8> = chunk.collect();
            if chunk.len() != 3 {
                vec![] 
            } else {
                convert_chunk(&chunk).to_vec()
            }
        })
        .map(b64_to_char)
        .collect();

    Ok(String::from_utf8(b64_bytes).unwrap())
}
