#[cfg(feature = "ex")]
use dotenv::dotenv;
use korea_investment_rs::{
    oauth::Oauth,
    overseas::{OverseasPriceQuery, get_overseas_price},
    types::CustType,
    utils::ApiHeader,
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

    // ✅ Apple (AAPL), 나스닥(NAS)
    let query = OverseasPriceQuery {
        auth: "P",         // 사용자 권한정보 (개인)
        exchg_code: "NAS", // 거래소 코드 (NYSE, NAS, AMS 등)
        symbol: "AAPL",    // 종목코드(티커)
    };

    // ✅ 개인용 기본 헤더 생성
    let header = ApiHeader::personal();

    // ✅ 해외 현재가 조회 실행
    let result = get_overseas_price(&token, &header, query)
        .await
        .expect("조회 실패");

    println!("📈 해외주식 현재가 조회 결과:");
    println!("{:#?}", result);
}
