#[cfg(feature = "ex")]
use dotenv::dotenv;
use korea_investment_rs::{
    bond::DomesticBondApi,
    provider::KISProvider,
    types::MarketType,
};

#[tokio::main]
async fn main() {
    #[cfg(feature = "ex")]
    dotenv().ok();

    let provider = KISProvider::new(MarketType::Domestic, false)
        .await
        .expect("Provider 초기화 실패");

    let response = provider
        .get_bond_price("KR2033022D33")
        .await
        .expect("채권 시세 조회 실패");

    println!("body = {:#?}", response.body);
}
