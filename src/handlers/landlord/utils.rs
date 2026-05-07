pub fn kes(amount: i32) -> String {
    let s = amount.to_string();
    let with_commas = s
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(std::str::from_utf8)
        .collect::<Result<Vec<_>, _>>()
        .unwrap_or_default()
        .join(",");
    format!("Ksh {with_commas}")
}
