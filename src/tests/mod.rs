use std::fs::File;
use std::io::BufReader;
use crate::pattern_match;

#[test]
fn test_pattern_match() {
    let mut result = Vec::new();
    let pattern = "gold";

    let file = File::open("resources/metals").unwrap();
    let reader = BufReader::new(file);
    pattern_match(pattern, reader, &mut result);
    assert_eq!(result, b"2. white gold\n3. yellow gold\n6. rose gold\n");
}
