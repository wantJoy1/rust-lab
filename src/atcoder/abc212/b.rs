fn main() {
    let x: String = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        let mut iter = s.split_whitespace().map(|i| i.parse::<i32>().unwrap());
        (iter.next().unwrap(), iter.next().unwrap())
    };

    println!("{}", x);
}

fn all_equal(x: i32) -> bool {
    let digits: Vec<i32> = x.to_string().chars().map(|ch| ch.to_digit(10).unwrap());

    let first: i32 = digits.first.unwrap();
    digits.all(|i| i == first)
}
