#[cfg(feature = "ex")]
use dotenv::dotenv;
#[cfg(feature = "ex")]
use korea_investment_rs::provider;
use korea_investment_rs::{
    domestic::quotations::Domestic,
    oauth::Oauth,
    provider::KISProvider,
    types::{CustType, MarketType},
    utils::ApiHeader,
};

#[tokio::main]
async fn main() {
    #[cfg(feature = "ex")]
    dotenv().ok();

    #[cfg(feature = "ex")]
    // Provider 생성
    let provider = KISProvider::new(MarketType::Domestic, true)
        .await
        .expect("Provider 초기화 실패");

    #[cfg(feature = "ex")]
    // 종목코드 005930 (삼성전자)
    let result = provider.get_inquire_price("005930").await;
    #[cfg(feature = "ex")]

    println!("{:#?}", result);
}
