const DECLARATION_OF_INDEPENDENCE: &str = include_str!("declaration.txt");

fn char_to_hex(c: &u8) -> Result<u8, String> {
    match c {
        b'0'..=b'9' => Ok(c - b'0'),
        b'a'..=b'f' => Ok(c - b'a' + 10),
        b'A'..=b'F' => Ok(c - b'A' + 10),
        _ => Err(format!("Invalid hex char: {}", *c as char)),
    }
}

fn get_freqs(bytes: &[u8]) -> [f64; 256] {
    let inv_sz: f64 = 1.0 / (bytes.len() as f64);
    let mut freqs: [f64; 256] = [0.0; 256];
    for b in bytes.into_iter() {
        freqs[*b as usize] += inv_sz;
    }
    freqs
}

fn get_english_freqs() -> [f64; 256] {
    let doi_str = DECLARATION_OF_INDEPENDENCE.to_string();
    get_freqs(doi_str.as_bytes())
}

fn dot(l: &[f64; 256], r: &[f64; 256]) -> f64 {
    l.iter().zip(r.iter()).map(|(x, y)| x * y).sum()
}

fn combine(s: &[u8], key: u8) -> Vec<u8> {
    s.into_iter().map(|b| b ^ key).collect()
}

pub fn decode(s: &str) -> Result<String, String> {
    let hex_bytes: Vec<u8> = s
        .as_bytes()
        .iter()
        .map(|c| char_to_hex(c))
        .collect::<Result<Vec<u8>, String>>()?;

    let decoded_bytes: Vec<u8> = hex_bytes
        .chunks(2)
        .map(|chunk| {
            if chunk.len() == 2 {
                Ok((chunk[0] << 4) + chunk[1])
            } else {
                Err("Odd number of hex digits".to_string())
            }
        })
        .collect::<Result<Vec<u8>, String>>()?;

    let eng_freqs = get_english_freqs();

    let options: [Vec<u8>; 256] = std::array::from_fn(|key| combine(&decoded_bytes, key as u8));

    let similarities: [f64; 256] = options
        .iter()
        .map(|bytes| {
            let freqs = get_freqs(bytes);
            dot(&freqs, &eng_freqs)
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    let best_idx = similarities
        .iter()
        .enumerate()
        .fold((0, f64::NEG_INFINITY), |(best_i, best_val), (i, &val)| {
            if val > best_val {
                (i, val)
            } else {
                (best_i, best_val)
            }
        })
        .0;

    let best_match: &[u8] = &options[best_idx];

    Ok(String::from_utf8(best_match.to_vec()).unwrap())
}
