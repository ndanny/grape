use anyhow::{Context, Result};
use clap::Parser;
use std::fs::File;
use std::io::{prelude::*, BufReader, Write};
use std::path::PathBuf;

#[cfg(test)]
mod tests;

#[derive(Debug, Parser)]
struct Cli {
    pattern: String,

    #[clap(parse(from_os_str))]
    path: PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let path = args.path.clone();

    let file = File::open(args.path).with_context(|| format!("could not read file {:?}", path))?;
    let reader = BufReader::new(file);

    pattern_match(&args.pattern, reader, &mut std::io::stdout());

    Ok(())
}

fn pattern_match(pattern: &str, reader: BufReader<File>, mut writer: impl Write) {
    for (index, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(content) => content,
            Err(e) => panic!("Error reading line: {}: {:?}", index, e)
        };

        if line.contains(pattern) {
            match writeln!(writer, "{}. {}", index + 1, line) {
                Ok(_) => continue,
                Err(e) => panic!("Error writing matched line to stout: {:?}", e)
            };
        }
    }
}
