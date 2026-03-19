#[cfg(feature = "ex")]
use dotenv::dotenv;
use korea_investment_rs::{
    oauth::Oauth,
    overseas::{get_overseas_index_minutes, OverseasIndexMinuteQuery},
    types::CustType,
    utils::ApiHeader,
};

#[tokio::main]
async fn main() {
    #[cfg(feature = "ex")]
    dotenv().ok();

    // ⚠ 모의투자 미지원 API
    let practice = false;

    let token = Oauth::from_env_with_cache(CustType::P, practice)
        .await
        .expect("토큰 발급 실패");

    let header = ApiHeader::personal();

    let query = OverseasIndexMinuteQuery {
        market_div_code: "N", // 해외지수
        symbol: "NDX",        // 예시 심볼 (필요에 맞게 변경)
        hour_cls_code: "0",   // 정규장
        include_past: "Y",    // 과거 데이터 포함
    };

    let result = get_overseas_index_minutes(&token, &header, query)
        .await
        .expect("조회 실패");

    println!("📈 해외지수 분봉 조회 결과:");
    println!("output1: {:#?}", result.0);
    println!("output2: {:#?}", result.1);
}
