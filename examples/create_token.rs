use dotenv::dotenv;
use korea_investment_rs::oauth::Oauth;
use std::env;
use std::slice::GetDisjointMutError;
#[tokio::main]
async fn main() {
    dotenv().ok();

    let app_key = env::var("PUB_KEY").expect("APP_KEY not set in .env file");
    let app_secret = env::var("SCREST_KEY").expect("APP_SECRET not set in .env file");

    let oauth = Oauth::new(app_key, app_secret);
    let token = Oauth::get_access_token(&oauth).await.unwrap();
    println!("{}", token);
}
