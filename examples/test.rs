#[cfg(feature = "ex")]
use dotenv::dotenv;
use korea_investment_rs::{
    domestic::quotations::{PeriodPriceQuery, get_inquire_period_price},
    oauth::Oauth,
    provider::KISProvider,
    types::{CustType, MarketType},
    utils::ApiHeader,
};

#[tokio::main]
async fn main() {
    #[cfg(feature = "ex")]
    let provider_domestic = KISProvider::new(MarketType::Domestic, true).await.unwrap();
    #[cfg(feature = "ex")]
    println!("{:?}", provider_domestic);
}
