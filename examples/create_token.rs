use dotenv::dotenv;
use korea_investment_rs::oauth::{Oauth, OauthType};
use std::env;
#[tokio::main]
async fn main() {
    dotenv().ok();

    let app_key = env::var("PUB_KEY").expect("APP_KEY not set in .env file");
    let app_secret = env::var("SCREST_KEY").expect("APP_SECRET not set in .env file");
    let r#type = OauthType::PRACTICE;
    let token = Oauth::new(app_key, app_secret, r#type).await.unwrap();
    println!("{:?}", token);
}
