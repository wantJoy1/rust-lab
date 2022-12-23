use std::io::stdin;

fn main() {
    println!("Hello, world!");

    println!("Input the function name:");
    let funcs = ["ferris", "guessing", "tweets"];
    for (i, &func_name) in funcs.iter().enumerate() {
        print!("{}:{}, ", i, func_name);
    }
    println!();

    let func_id: usize = get_str()
        .trim()
        .parse()
        .expect("invalid input");

    match func_id {
        0 => ferris(get_str()),
        1 => guessing(),
        2 => tweets(),
        _ => println!("invalid number"),
    }
}

fn get_str() -> String {
    println!("Input value");
    let mut guess = String::new();
    stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    guess
}