#[cfg(feature = "ex")]
use dotenv::dotenv;
use korea_investment_rs::{
    provider::KISProvider,
    types::MarketType,
};

#[tokio::main]
async fn main() {
    #[cfg(feature = "ex")]
    dotenv().ok();

    let domestic = KISProvider::new(MarketType::Domestic, true)
        .await
        .expect("국내 Provider 생성 실패");
    let overseas = KISProvider::new(MarketType::Overseas, true)
        .await
        .expect("해외 Provider 생성 실패");

    println!("domestic = {:#?}", domestic.market);
    println!("overseas = {:#?}", overseas.market);
}
