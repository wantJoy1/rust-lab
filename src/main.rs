mod util;
mod guess;
mod ferris;
mod twitter;
mod jpbank;

use util::get_str;
use guess::guessing;
use ferris::ferris_say;
// use twitter::tweets;
use jpbank::parse_csv;

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
        0 => ferris_say(get_str()),
        1 => guessing(),
        // 2 => println!("{}", tweets().unwrap()),
        3 => parse_csv(),
        _ => println!("invalid number"),
    }
}