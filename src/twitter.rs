pub mod twitter;

use std::fs::File;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use twitter_v2::TwitterApi;
use twitter_v2::authorization::{Oauth2Token, BearerToken};
use twitter_v2::query::{TweetField, UserField};

/*
async fn tweets () {
    let property: Property = get_property();

    let consumer_key: String = property.twitter_consumerkey;
    let consumer_key_secret: String = property.twitter_consumersecret;
    let access_token: String = property.twitter_accesstoken;
    let access_token_secret: String = property.twitter_accesstokensecret;

    let auth = BearerToken::new();
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
*/

 #[derive(Serialize, Deserialize)]
 struct Property {
    twitter_consumerkey: String, 
    twitter_consumersecret: String, 
    twitter_accesstoken: String, 
    twitter_accesstokensecret: String, 
 }

fn get_property() -> Property {
    let filename = "properties.json";
    let file = fs::File::open(filename)
        .expect("failed to read JSON file");
    serde_json::from_reader(file)
        .expect("failed to deserialize")
}