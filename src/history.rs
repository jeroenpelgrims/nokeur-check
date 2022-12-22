use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom, Write};
use std::path::Path;

const HISTORY_FILE: &str = "history.txt";

pub fn add(value: f64) {
    let now = chrono::Local::now().to_rfc3339().to_string();
    let text = format!("{} {}\n", now, value);
    let mut history = OpenOptions::new()
        .create(true)
        .append(true)
        .open(HISTORY_FILE)
        .expect("Can't open history file.");
    history.seek(SeekFrom::End(0)).ok();
    history.write_all(text.as_bytes()).ok();
}

pub fn last() -> Option<f64> {
    if !Path::new(HISTORY_FILE).exists() {
        return None;
    }

    let input = std::fs::read_to_string(HISTORY_FILE).unwrap();
    let last = input.lines().last()?;
    let rate = last.split(" ").last()?;
    let rate_parsed = rate.parse::<f64>().expect("Can't parse history value");

    Some(rate_parsed)
}
