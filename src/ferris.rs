pub mod ferris;

use ferris_says::say;
use std::io::{stdout, BufWriter};

fn ferris_say(message: String) {
    let stdout = stdout();
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}