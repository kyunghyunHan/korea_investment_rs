#[cfg(feature = "ex")]
use dotenv::dotenv;
use korea_investment_rs::{
    oauth::Oauth,
    overseas::{OverseasTodayMinuteQuery, get_overseas_today_minutes},
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

    let query = OverseasTodayMinuteQuery {
        auth: "",
        exchg_code: "NAS",
        symbol: "TQQQ",
        nmin: "5",           // 1, 2, 5 등
        include_prev_day: "1", // 0: 당일, 1: 전일 포함
        next: "",            // 처음 조회 "", 다음 조회 "1"
        record_count: "120", // 최대 120
        fill: "",
        next_key: "",
    };

    let (output1, output2) = get_overseas_today_minutes(&token, &header, query)
        .await
        .expect("조회 실패");

    println!("📈 해외주식 분봉 조회 결과:");
    println!("output1: {:#?}", output1);
    println!("output2: {}건", output2.len());
    for item in output2.iter().take(10) {
        println!("{:#?}", item);
    }
}
