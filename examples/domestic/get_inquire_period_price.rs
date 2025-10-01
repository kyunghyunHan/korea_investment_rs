#[cfg(feature = "ex")]
use dotenv::dotenv;
use korea_investment_rs::{
    oauth::Oauth,
    domestic::{ApiHeader, PeriodPriceQuery, get_inquire_period_price},
    types::CustType,
};

#[tokio::main]
async fn main() {
    #[cfg(feature = "ex")]
    dotenv().ok();

    // ⚡ true = 모의투자 / false = 실전계좌
    let practice = true;

    // ✅ 토큰 발급 (캐싱 지원)
    let token = Oauth::from_env_with_cache(CustType::P, practice)
        .await
        .expect("토큰 발급 실패");

    // ✅ 삼성전자 (005930), 2024년 전체 일봉 조회
    let query = PeriodPriceQuery::daily(
        "005930",    // 종목코드
        "20240101",  // 시작일
        "20241231",  // 종료일
    );

    // ✅ 개인 고객용 기본 헤더
    let header = ApiHeader::personal();

    // ✅ 국내주식 기간별 시세 조회
    let result = get_inquire_period_price(&token, &header, query)
        .await
        .expect("조회 실패");

    println!("📊 국내주식 기간별 시세 (삼성전자 2024년 일봉)");
    println!("{:#?}", result.output2); // 일자별 캔들 데이터
}
