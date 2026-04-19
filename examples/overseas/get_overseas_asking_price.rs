#[cfg(feature = "ex")]
use dotenv::dotenv;
use korea_investment_rs::{
    overseas::OverseasTrading,
    provider::KISProvider,
    types::MarketType,
};

#[tokio::main]
async fn main() {
    #[cfg(feature = "ex")]
    dotenv().ok();

    let provider = KISProvider::new(MarketType::Overseas, false)
        .await
        .expect("Provider 초기화 실패");

    let response = provider
        .get_overseas_asking_price("NAS", "AAPL")
        .await
        .expect("해외 호가 조회 실패");

    println!("body = {:#?}", response.body);
}
