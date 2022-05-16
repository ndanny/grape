use anyhow::{Context, Result};
use clap::Parser;
use grape::pattern_match;
use std::fs::File;
use std::io::BufReader;
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

    pattern_match(&args.pattern, reader);

    Ok(())
}
