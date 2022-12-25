use std::io::stdin;

pub fn get_str() -> String {
    println!("Input value");
    let mut guess = String::new();
    stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    guess
}