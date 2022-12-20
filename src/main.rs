use ferris_says::say;
use std::io::{stdout, BufWriter, stdin};
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Hello, world!");

    println!("Input the function name:");
    let funcs = ["ferris", "guessing"];
    for name in &funcs {
        print!("{}, ", name);
    }

    match get_str().as_str() {
        "ferris" => ferris(String::from("Hello fellow Rustaceans!")),
        "guessing" => guessing(),
        &_ => println!("invalid name"),
    }
}

fn ferris(message: String) {
    let stdout = stdout();
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}

fn guessing() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");

        let guess: u32 = match get_str().trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}

fn get_str() -> String {
    let mut guess = String::new();
    stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let guess: String = guess;
    return guess;
}