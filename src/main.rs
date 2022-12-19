use ferris_says::say;
use std::io::{stdout, BufWriter, stdin};

fn main() {
    println!("Hello, world!");
    ferris_say(String::from("Hello fellow Rustaceans!"));
    guessing();
}

fn ferris_say(message: String) {
    let stdout = stdout();
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}

fn guessing() {
    println!("Guess the number!");
    println!("Please input your guess.");
    let mut guess = String::new();

    stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");


    println!("You guessed: {}", guess);
}