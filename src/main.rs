use anyhow::{Context, Result};
use clap::Parser;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::PathBuf;

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

    pattern_match(&args.pattern, reader);

    Ok(())
}

fn pattern_match(pattern: &str, reader: BufReader<File>) {
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if line.contains(pattern) {
            println!("{}. {}", index + 1, line);
        }
    }
}
