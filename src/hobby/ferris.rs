use ferris_says::say;
use std::io::{stdout, BufWriter};

pub fn ferris_say(message: String) {
    let stdout = stdout();
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}