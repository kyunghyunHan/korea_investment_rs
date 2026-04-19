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

    // ⚠ 모의투자 미지원 API
    let practice = true;

    let provider = KISProvider::new(MarketType::Domestic, practice)
        .await
        .expect("프로바이더 생성 실패");

    // 업종 코드 예시: 코스피(0001), 코스닥(1001), 코스피200(2001)
    let market_div_code = "U"; // 업종
    let index_code = "0001";   // 코스피

    let result = provider
        .get_inquire_index_price(market_div_code, index_code)
        .await
        .expect("조회 실패");

    println!("📈 국내업종 현재지수 조회 결과:");
    println!("{:#?}", result);
}
