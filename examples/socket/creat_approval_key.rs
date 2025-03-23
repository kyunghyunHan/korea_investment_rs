#[cfg(feature = "ex")]
use dotenv::dotenv;
use korea_investment_rs::websocket::oauth::ApproveOauth;
use std::env;
#[tokio::main]
async fn main() {
    #[cfg(feature = "ex")]
    dotenv().ok();

    let app_key = env::var("PUB_KEY").expect("APP_KEY not set in .env file");
    let app_secret = env::var("SCREST_KEY").expect("APP_SECRET not set in .env file");
    let token = ApproveOauth::new(app_key, app_secret).await.unwrap();
    println!("{:?}", token);
}
