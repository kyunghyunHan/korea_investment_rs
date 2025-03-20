#[cfg(feature = "ex")]
use dotenv::dotenv;
use korea_investment_rs::{
    domestic::quotations::{
        ApiHeader, Custtype, IDIQuery, get_inquire_daily_itemchartprice, get_inquire_price,
    },
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
    println!("{}", token.token);
    let query = IDIQuery::new("J", "20220411", "20220509", "000660", "0", "D");

    println!("{:?}:", query);

    let header = ApiHeader::new(Custtype::P, None, None, None, None, None).unwrap();
    println!("{:?}:", header);

    let result = get_inquire_daily_itemchartprice(token, header, query)
        .await
        .unwrap();
    println!("{:?}", result);
}
