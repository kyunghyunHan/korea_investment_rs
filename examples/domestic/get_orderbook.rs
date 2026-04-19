#[cfg(feature = "ex")]
use dotenv::dotenv;
use korea_investment_rs::{
    domestic::quotations::DomesticExtendedQuotations,
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

    let response = provider
        .get_orderbook("005930")
        .await
        .expect("호가 조회 실패");

    println!("body = {:#?}", response.body);
}
