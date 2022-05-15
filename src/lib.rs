use std::fs::File;
use std::io::{prelude::*, BufReader, Write};

pub fn pattern_match(pattern: &str, reader: BufReader<File>, mut writer: impl Write) {
    for (index, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(content) => content,
            Err(e) => panic!("Error reading line: {}: {:?}", index, e),
        };

        if line.contains(pattern) {
            match writeln!(writer, "{}. {}", index + 1, line) {
                Ok(_) => continue,
                Err(e) => panic!("Error writing matched line to stout: {:?}", e),
            };
        }
    }
}
