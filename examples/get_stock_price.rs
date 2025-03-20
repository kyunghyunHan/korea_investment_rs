#[cfg(feature = "ex")]
use dotenv::dotenv;
use korea_investment_rs::{
    domestic::quotations::{ApiHeader, Custtype, QueryParam, get_stock_price},
    oauth::{Oauth, OauthType},
};

use std::env;
#[tokio::main]
async fn main() {
    #[cfg(feature = "ex")]
    dotenv().ok();

    let app_key = env::var("PUB_KEY").expect("APP_KEY not set in .env file");
    let app_secret = env::var("SCREST_KEY").expect("APP_SECRET not set in .env file");
    let r#type = OauthType::PRACTICE;
    let token = Oauth::new(app_key, app_secret, r#type).await.unwrap();
    let query = QueryParam::new("J", "005930");
    let header = ApiHeader::new(Custtype::P, None, None, None, None, None).unwrap();
    let result = get_stock_price(token, header, query).await.unwrap();
    println!("{:?}", result);
}
