#[cfg(feature = "ex")]
use dotenv::dotenv;
use korea_investment_rs::{
    domestic::analysis::{DomesticAnalysis, DomesticAnalysisEndpoint},
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
        .get_analysis_raw(
            DomesticAnalysisEndpoint::ConditionSearchTitles,
            &[("user_id", ""), ("seq", "")],
        )
        .await
        .expect("조건검색 목록조회 실패");

    println!("body = {:#?}", response.body);
}
