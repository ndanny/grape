use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use crate::pattern_match;
use predicates::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::process::Command;

#[test]
fn pattern_matching() {
    let mut result = Vec::new();
    let pattern = "gold";

    let file = File::open("resources/metals").unwrap();
    let reader = BufReader::new(file);
    pattern_match(pattern, reader, &mut result);
    assert_eq!(result, b"2. white gold\n3. yellow gold\n6. rose gold\n");
}

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("grape")?;
    
    cmd.arg("let").arg("path/to/null/file");
    cmd.assert().failure().stderr(predicate::str::contains("could not read file"));

    Ok(())
}

#[test]
fn find_pattern_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("strawhats.txt")?;
    file.write_str("Luffy\nChopper\nNami\nSanji\nZorro\nUsopp\n")?;

    let mut cmd = Command::cargo_bin("grape")?;
    cmd.arg("Chopper").arg(file.path());
    cmd.assert().success().stdout(predicate::str::contains("Chopper\n"));

    Ok(())
}
