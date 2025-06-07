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
    for b in bytes.iter() {
        freqs[*b as usize] += inv_sz;
    }
    freqs
}

fn get_english_freqs() -> [f64; 256] {
    let doi_str = DECLARATION_OF_INDEPENDENCE.to_string();
    get_freqs(doi_str.as_bytes())
}

struct Candidate {
    similarity: f64,
    content: Vec<u8>,
}

fn xor_word(key: u8, bytes: &Vec<u8>) -> Vec<u8> {
    bytes.iter().map(|b| b ^ key).collect()
}

fn get_similarity(content: &Vec<u8>, freqs: &[f64; 256]) -> f64 {
    let inv_sz = 1.0 / (content.len() as f64);
    content
        .iter()
        .fold(0.0, |acc, b| acc + freqs[*b as usize] * inv_sz)
}

fn make_candidate(bytes: &Vec<u8>, freqs: &[f64; 256]) -> Candidate {
    (0..=255).fold(
        Candidate {
            similarity: 0.0,
            content: vec![],
        },
        |acc, i| {
            let content = xor_word(i, bytes);
            let similarity = get_similarity(&content, freqs);
            if similarity >= acc.similarity {
                Candidate {
                    content: content,
                    similarity: similarity,
                }
            } else {
                acc
            }
        },
    )
}

pub fn decode_line(line: &str) -> Result<Vec<u8>, String> {
    let hex_bytes: Vec<u8> = line
        .as_bytes()
        .iter()
        .map(|c| char_to_hex(c))
        .collect::<Result<Vec<u8>, String>>()?;

    hex_bytes
        .chunks(2)
        .map(|chunk| {
            if chunk.len() == 2 {
                Ok((chunk[0] << 4) + chunk[1])
            } else {
                Err("Odd number of hex digits".to_string())
            }
        })
        .collect::<Result<Vec<u8>, String>>()
}

pub fn decode(contents: &str) -> Result<String, String> {
    let lines: Vec<&str> = contents.lines().collect();
    let eng_freqs = get_english_freqs();

    let candidates = lines
        .iter()
        .map(|line| {
            let bytes = decode_line(line)?;
            Ok(make_candidate(&bytes, &eng_freqs))
        })
        .collect::<Result<Vec<Candidate>, String>>()?;

    let winner = candidates
        .into_iter()
        .fold(
            Candidate {
                content: vec![],
                similarity: 0.0,
            },
            |acc, c| {
                if c.similarity >= acc.similarity {
                    c
                } else {
                    acc
                }
            },
        )
        .content;

    Ok(String::from_utf8(winner.to_vec()).unwrap())
}
