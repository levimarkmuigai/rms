use std::time::SystemTime;

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

pub fn time_ago(t: SystemTime) -> String {
    let elapsed = SystemTime::now().duration_since(t).unwrap_or_default();

    let secs = elapsed.as_secs();
    let min = secs / 60;
    let hours = min / 60;
    let days = hours / 24;

    match (hours, days) {
        (0, _) => "just now".into(),
        (h, 0) => format!("{} hours ago", h),
        (_, 1) => "yesterday".into(),
        (_, h) => format!("{} days ago", h),
    }
}
