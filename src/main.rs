use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let out = b"Hello, World from Rust!";
    let width = 24;
    let mut writer = BufWriter::new(stdout.lock());
    say(out, width, &mut writer).unwrap();
}
