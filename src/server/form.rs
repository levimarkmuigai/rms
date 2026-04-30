use std::collections::HashMap;

pub fn parse(body: &[u8]) -> HashMap<String, String> {
    String::from_utf8_lossy(body)
        .split('&')
        .filter_map(|pair| pair.split_once("=").map(|(k, v)| (decode(k), decode(v))))
        .collect()
}

fn decode(s: &str) -> String {
    s.replace('+', " ")
        .split('%')
        .enumerate()
        .map(|(i, chunk)| {
            if i == 0 {
                return chunk.to_string();
            }
            if chunk.len() >= 2 {
                if let Ok(b) = u8::from_str_radix(&chunk[..2], 16) {
                    return format!("{}{}", b as char, &chunk[2..]);
                }
            }
            format!("%{chunk}")
        })
        .collect()
}
