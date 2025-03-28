#[cfg(feature = "ex")]
use dotenv::dotenv;
use korea_investment_rs::{
    domestic::quotations::{
        ApiHeader, IDIQuery, get_inquire_daily_itemchartprice, get_inquire_price,
    },
    oauth::{Oauth, OauthType},
    types::CustType,
};

use std::env;
#[tokio::main]
async fn main() {
    #[cfg(feature = "ex")]
    dotenv().ok();
    // let app_key = env::var("PUB_KEY").expect("APP_KEY not set in .env file");
    // let app_secret = env::var("SCREST_KEY").expect("APP_SECRET not set in .env file");
    let r#type = OauthType::PRACTICE;
    // let token = Oauth::new(app_key, app_secret, r#type).await.unwrap();
    let token = Oauth::from_env(CustType::P, None, None, None, None, None)
        .await
        .unwrap();
    // let query = IDIQuery::new("J", "005930", "20220101", "20220531", "D", "0");
    // let header = ApiHeader::new(Custtype::P, None, None, None, None, None).unwrap();
    // let result = get_inquire_daily_itemchartprice(token, header, query)
    //     .await
    //     .unwrap();
    println!("{:?}", token);
}
