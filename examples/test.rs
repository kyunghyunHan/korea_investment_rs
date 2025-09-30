#[cfg(feature = "ex")]
use dotenv::dotenv;
use korea_investment_rs::{
    oauth::Oauth,
    overseas::{ApiHeader, OverseasPriceQuery, get_overseas_price}, // 해외 모듈 사용
    types::CustType,
};

#[tokio::main]
async fn main() {
    #[cfg(feature = "ex")]
    dotenv().ok();

    // 실전계좌 여부 (true면 모의, false면 실전)
    let practice = false;

    // .env 기반으로 개인 고객(P) 토큰 발급
    let token = Oauth::from_env(CustType::P, practice)
        .await
        .expect("토큰 발급 실패");

    println!("발급된 토큰: {}", token.token);

    // ✅ Apple (AAPL), 나스닥 거래소(NAS)
    let query = OverseasPriceQuery {
        auth: "P",         // 사용자 권한정보 (개인)
        exchg_code: "NAS", // 나스닥
        symbol: "AAPL",    // 애플 티커
    };

    // 개인 고객용 기본 헤더
    let header = ApiHeader::personal();

    // ✅ 해외 현재가 조회
    let result = get_overseas_price(&token, &header, query)
        .await
        .expect("조회 실패");

    println!("{:#?}", result);
}
