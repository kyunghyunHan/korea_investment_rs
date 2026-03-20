#[cfg(feature = "ex")]
use dotenv::dotenv;
use korea_investment_rs::{
    oauth::Oauth,
    types::CustType,
    utils::{ApiHeader, call_api},
};
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
struct DailyChartResponse {
    rt_cd: String,
    msg_cd: String,
    msg1: String,
    #[serde(default)]
    output1: Value,
    #[serde(default)]
    output2: Value,
}

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

    // ✅ 개인용 기본 헤더 생성
    let header = ApiHeader::personal();

    // ✅ 조회 파라미터 (필요에 맞게 변경)
    let div = "N";          // 시장 분류 코드
    let itm_no = "KOSPQ";    // 종목번호(티커)
    let st_dt = "20240101"; // 시작일자(YYYYMMDD)
    let ed_dt = "20240131"; // 종료일자(YYYYMMDD)
    let period = "D";       // 기간분류코드

    let url = "https://openapi.koreainvestment.com:9443/uapi/overseas-price/v1/quotations/inquire-daily-chartprice";
    let tr_id = "FHKST03030100"; // 해외주식 종목/지수/환율기간별시세(일/주/월/년)

    let response: DailyChartResponse = call_api(
        &token,
        &header,
        url,
        tr_id,
        &[
            ("FID_COND_MRKT_DIV_CODE", div),
            ("FID_INPUT_ISCD", itm_no),
            ("FID_INPUT_DATE_1", st_dt),
            ("FID_INPUT_DATE_2", ed_dt),
            ("FID_PERIOD_DIV_CODE", period),
        ],
    )
    .await
    .expect("조회 실패");

    if response.rt_cd == "0" && !response.msg1.contains("조회할 자료가 없습니다") {
        println!("📈 해외주식 기간별 시세 조회 결과 (output1):");
        println!("{:#?}", response.output1);
    } else {
        println!("{} , {}", response.msg_cd, response.msg1);
    }
}
