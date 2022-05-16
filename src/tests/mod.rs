use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("grape")?;
    
    cmd.arg("let").arg("path/to/null/file");
    cmd.assert().failure().stderr(predicate::str::contains("could not read file"));

    Ok(())
}

#[test]
fn find_pattern_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("pirates.txt")?;
    file.write_str("Luffy\nChopper\nNami\nSanji\nZorro\nUsopp\n")?;

    let mut cmd = Command::cargo_bin("grape")?;
    cmd.arg("Chopper").arg(file.path());
    cmd.assert().success().stdout(predicate::str::contains("Chopper"));

    Ok(())
}
