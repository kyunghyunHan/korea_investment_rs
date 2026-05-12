#[cfg(feature = "ex")]
use dotenv::dotenv;
use korea_investment_rs::{
    domestic::quotations::Domestic, provider::KISProvider, types::MarketType,
};

#[tokio::main]
async fn main() {
    #[cfg(feature = "ex")]
    dotenv().ok();

    let provider = KISProvider::new(MarketType::Domestic, true)
        .await
        .expect("Provider 초기화 실패");

    // get_minutes_by_day는 입력한 날짜/시간 기준의 분봉을 조회합니다.
    let result = provider
        .get_minutes_by_day("005930", "20260512", "153000")
        .await
        .expect("조회 실패");

    println!("국내주식 일별 분봉 조회 결과:");
    println!("{}건", result.len());
    for item in result.iter().take(10) {
        println!("{:#?}", item);
    }
}
