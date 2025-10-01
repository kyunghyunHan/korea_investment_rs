#[cfg(feature = "ex")]
use dotenv::dotenv;
use korea_investment_rs::{
    domestic::quotations::Domestic, // ✅ 트레잇 반드시 가져와야 메서드 인식됨
    provider::KISProvider,
    types::MarketType,
};

#[tokio::main]
async fn main() {
    #[cfg(feature = "ex")]
    dotenv().ok();

    // ⚡ true = 모의투자 / false = 실전계좌
    let practice = true;

    // ✅ Provider 생성 (국내 주식)
    let provider = KISProvider::new(MarketType::Domestic, practice)
        .await
        .expect("Provider 초기화 실패");

    // ✅ 삼성전자 (005930), 2024년 전체 일봉 조회
    let result = provider
        .get_inquire_period_price("005930", "20240101", "20241231", "D") // "D" = 일봉
        .await
        .expect("조회 실패");

    println!("📊 국내주식 기간별 시세 (삼성전자 2024년 일봉)");
    println!("{:#?}", result.output2); // 일자별 캔들 데이터
}
