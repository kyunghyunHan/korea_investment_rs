#[cfg(feature = "ex")]
use dotenv::dotenv;
use korea_investment_rs::{
    oauth::Oauth,
    overseas::{ApiHeader, OverseasProductInfoQuery, get_overseas_product_info},
    types::CustType,
};

#[tokio::main]
async fn main() {
    #[cfg(feature = "ex")]
    dotenv().ok();

    // ⚡ true = 모의투자 / false = 실전계좌 (해외주식 상품기본정보는 모의투자 미지원)
    let practice = false;

    // ✅ 토큰 발급 (캐싱 지원)
    let token = Oauth::from_env_with_cache(CustType::P, practice)
        .await
        .expect("토큰 발급 실패");

    // ✅ Apple (AAPL), 미국 나스닥(512)
    let query = OverseasProductInfoQuery {
        product_type_code: "512", // 상품유형코드
        product_number: "AAPL",   // 종목코드(티커)
    };

    // ✅ 개인용 기본 헤더 생성
    let header = ApiHeader::personal();

    // ✅ 해외주식 상품기본정보 조회 실행
    let result = get_overseas_product_info(&token, &header, query)
        .await
        .expect("조회 실패");

    println!("📈 해외주식 상품기본정보 조회 결과:");
    println!("{:#?}", result);
}
