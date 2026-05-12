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

    // get_today_minutes는 당일 기준 시간 이전의 최근 분봉을 조회합니다.
    let result = provider
        .get_today_minutes("005930", "093000")
        .await
        .expect("조회 실패");

    println!("국내주식 당일 분봉 조회 결과:");
    println!("{}건", result.len());
    for item in result.iter().take(10) {
        println!("{:#?}", item);
    }
}
