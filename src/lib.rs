use std::fs::File;
use std::io::{prelude::*, BufReader, Result, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub fn pattern_match(pattern: &str, reader: BufReader<File>) {
    for (index, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(content) => content,
            Err(e) => panic!("Error reading line {}: {:?}", index, e),
        };

        if line.contains(pattern) {
            match write_matched_line(&line, pattern) {
                Ok(_) => continue,
                Err(e) => panic!("Error writing line to stdout: {:?}", e),
            };
        }
    }
}

fn write_matched_line(line: &str, pattern: &str) -> Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let chunks: Vec<&str> = line.split(pattern).collect();

    for (index, chunk) in chunks.iter().enumerate() {
        stdout.set_color(ColorSpec::new().set_fg(None))?;
        write!(&mut stdout, "{}", chunk)?;

        if index != chunks.len() - 1 {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
            write!(&mut stdout, "{}", pattern)?;
        }
    }

    writeln!(&mut stdout, "")?;

    Ok(())
}
