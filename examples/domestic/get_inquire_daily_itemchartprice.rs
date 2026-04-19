#[cfg(feature = "ex")]
use dotenv::dotenv;
use korea_investment_rs::{
    domestic::quotations::Domestic,
    provider::KISProvider,
    types::MarketType,
};

#[tokio::main]
async fn main() {
    #[cfg(feature = "ex")]
    dotenv().ok();

    let provider = KISProvider::new(MarketType::Domestic, true)
        .await
        .expect("Provider 초기화 실패");

    let result = provider
        .get_inquire_period_price("005930", "20240101", "20241231", "D")
        .await
        .expect("조회 실패");

    println!("{:#?}", result.output2);
}
