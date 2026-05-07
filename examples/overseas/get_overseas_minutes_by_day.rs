#[cfg(feature = "ex")]
use dotenv::dotenv;
use korea_investment_rs::{
    oauth::Oauth,
    overseas::{OverseasByDayMinuteQuery, get_overseas_minutes_by_day},
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

    let query = OverseasByDayMinuteQuery {
        auth: "",
        exchg_code: "NAS",
        symbol: "TQQQ",
        bymd: "20240131", // 조회일자(YYYYMMDD)
        nmin: "1",        // 분봉 간격
    };

    let output = get_overseas_minutes_by_day(&token, &header, query)
        .await
        .expect("조회 실패");

    println!("📈 해외주식 특정일 분봉 조회 결과: {}건", output.len());
    for item in output.iter().take(10) {
        println!("{:#?}", item);
    }
}
