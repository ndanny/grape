# grape - it's kinda like grep

grape is a command line utility like grep. It is written in Rust and it is very lightweight, fast, and efficient. Although it is barebones at the moment, future iterations will add onto its utility.

### Setup
0. Clone repo
1. Make sure you have Rust installed `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. In the root dir, run `cargo build --release`
3. To use, run `target/release/grape <search_pattern> <path_to_file>`
