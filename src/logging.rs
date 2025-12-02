use std::fs::OpenOptions;
use std::io::{self, Write};

use chrono::Local;

const LOG_FILE: &str = "kero.log";

fn write_entry(level: &str, message: &str) -> io::Result<()> {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_FILE)?;
    writeln!(file, "[{timestamp}] {level}: {message}")
}

pub fn info(message: impl AsRef<str>) {
    let _ = write_entry("INFO", message.as_ref());
}

pub fn error(message: impl AsRef<str>) {
    let _ = write_entry("ERROR", message.as_ref());
}

pub fn info_lines<T: AsRef<str>>(lines: &[T]) {
    for line in lines {
        info(line.as_ref());
    }
}
