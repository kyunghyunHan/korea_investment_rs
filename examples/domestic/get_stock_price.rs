#[cfg(feature = "ex")]
use dotenv::dotenv;
use korea_investment_rs::{
    domestic::quotations::{ApiHeader, QueryParam, get_inquire_price},
    oauth::{Oauth, OauthType},
    types::CustType,
};

use std::env;
#[tokio::main]
async fn main() {
    #[cfg(feature = "ex")]
    dotenv().ok();
    let app_key = env::var("PUB_KEY").expect("APP_KEY not set in .env file");
    let app_secret = env::var("SCREST_KEY").expect("APP_SECRET not set in .env file");
    let r#type = OauthType::PRACTICE;

    let token = Oauth::from_env(CustType::P, false)
        .await
        .expect("토큰 발급 실패");
    println!("{}", token.token);
    let query = QueryParam::new("J", "005930");
    let header =
        ApiHeader::new(CustType::P, None, None, None, None, None, None, None, None).unwrap();
    let result = get_inquire_price(token, header, query).await.unwrap();
    println!("{:?}", result);
}
