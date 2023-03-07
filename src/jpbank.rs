use std::env;

pub fn parse_csv() {
    let vec: Vec<String> = env::args().collect();

    for i in vec {
        println!("{}", i);
    }
}