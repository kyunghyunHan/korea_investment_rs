#[cfg(feature = "ex")]
use dotenv::dotenv;
use korea_investment_rs::{
    oauth::Oauth,
    overseas::{OverseasDailyPriceQuery, get_overseas_daily_price},
    types::CustType,
    utils::ApiHeader,
};

#[tokio::main]
async fn main() {
    #[cfg(feature = "ex")]
    dotenv().ok();

    // 해외 시세 API는 실전 도메인 기준으로 조회합니다.
    let practice = false;

    let token = Oauth::from_env_with_cache(CustType::P, practice)
        .await
        .expect("토큰 발급 실패");

    let header = ApiHeader::personal();

    let query = OverseasDailyPriceQuery {
        auth: "",
        exchg_code: "NAS",
        symbol: "TQQQ",
        gubn: "0",       // 0: 일, 1: 주, 2: 월
        bymd: "20240131", // 조회 기준일자(YYYYMMDD)
        modp: "0",       // 0: 수정주가 미반영, 1: 반영
        keyb: None,
    };

    let (output1, output2) = get_overseas_daily_price(&token, &header, query)
        .await
        .expect("조회 실패");

    println!("📈 해외주식 기간별 시세 조회 결과:");
    println!("output1: {:#?}", output1);
    println!("output2: {:#?}", output2);
}
