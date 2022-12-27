use std::fs::File;
use serde::{Serialize, Deserialize};
use twitter_v2::TwitterApi;
use twitter_v2::authorization::BearerToken;
use twitter_v2::query::{TweetField, UserField};

pub async fn tweets() -> Result<String, String> {
    let twitter_property: TwitterProperty = get_twitter_property();
    let bearer_token: String = twitter_property.bearertoken;

    let auth = BearerToken::new(bearer_token);
    let tweet = TwitterApi::new(auth)
        .get_tweet(1261326399320715264)
        .tweet_fields([TweetField::AuthorId, TweetField::CreatedAt])
        .send()
        .await?
        .into_data()
        .expect("this tweet should exist");
    assert_eq!(tweet.id, 1261326399320715264);
    assert_eq!(tweet.author_id.unwrap(), 2244994945);
    // assert_eq!(tweet.created_at.unwrap(), datetime!(2020-05-15 16:03:42 UTC));

    /*
    let my_followers = TwitterApi::new(auth)
        .with_user_ctx()
        .await?
        .get_my_followers()
        .user_fields([UserField::Username])
        .max_results(20)
        .send()
        .await?
        .into_data();
        */
    
    Ok(tweet.text)
}

#[derive(Serialize, Deserialize)]
struct TwitterProperty {
    consumerkey: String, 
    consumersecret: String, 
    accesstoken: String, 
    accesstokensecret: String, 
    bearertoken: String
}

fn get_twitter_property() -> TwitterProperty {
    let filename = "properties.json";
    let file = File::open(filename)
        .expect("failed to read JSON file");
    serde_json::from_reader(file)
        .expect("failed to deserialize")
}