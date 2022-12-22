use ferris_says::say;
use std::io::{stdout, BufWriter, stdin};
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Hello, world!");

    println!("Input the function name:");
    let funcs = ["ferris", "guessing"];
    for (i, &func_name) in funcs.iter().enumerate() {
        print!("{}:{}, ", i, func_name);
    }
    println!();

    let func_id: usize = get_str()
        .trim()
        .parse()
        .expect("invalid input");

    match func_id {
        0 => ferris(String::from("Hello fellow Rustaceans!")),
        1 => guessing(),
        _ => println!("invalid number"),
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
    guess
}

fn tweets () {
    use twitter_v2::TwitterApi;
    use twitter_v2::authorization::{Oauth2Token, BearerToken};
    use twitter_v2::query::{TweetField, UserField};

    let auth = BearerToken::new(std::env::var("APP_BEARER_TOKEN").unwrap());
    let tweet = TwitterApi::new(auth)
        .get_tweet(1261326399320715264)
        .tweet_fields([TweetField::AuthorId, TweetField::CreatedAt])
        .send()
        .await?
        .into_data()
        .expect("this tweet should exist");
    assert_eq!(tweet.id, 1261326399320715264);
    assert_eq!(tweet.author_id.unwrap(), 2244994945);
    assert_eq!(tweet.created_at.unwrap(), datetime!(2020-05-15 16:03:42 UTC));

    let auth: Oauth2Token = serde_json::from_str(&stored_oauth2_token)?;
    let my_followers = TwitterApi::new(auth)
        .with_user_ctx()
        .await?
        .get_my_followers()
        .user_fields([UserField::Username])
        .max_results(20)
        .send()
        .await?
        .into_data();
}