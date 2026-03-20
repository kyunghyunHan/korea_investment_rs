#[cfg(feature = "ex")]
use dotenv::dotenv;
use korea_investment_rs::{
    oauth::Oauth,
    overseas::{get_overseas_daily_chartprice, OverseasDailyChartQuery},
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

    // ✅ 개인용 기본 헤더 생성
    let header = ApiHeader::personal();

    // ✅ 조회 파라미터 (필요에 맞게 변경)
    let div = "N";          // 시장 분류 코드 (N: 해외지수, X: 환율, I: 국채, S: 금선물)
    let itm_no = "KOSPQ";   // 종목번호(티커)
    let st_dt = "20240101"; // 시작일자(YYYYMMDD)
    let ed_dt = "20240131"; // 종료일자(YYYYMMDD)
    let period = "D";       // 기간분류코드 (D/W/M/Y)

    let query = OverseasDailyChartQuery {
        market_div_code: div,
        symbol: itm_no,
        start_date: st_dt,
        end_date: ed_dt,
        period_div_code: period,
    };

    let (output1, output2) = get_overseas_daily_chartprice(&token, &header, query)
        .await
        .expect("조회 실패");

    println!("📈 해외주식 종목/지수/환율기간별시세 조회 결과:");
    println!("output1: {:#?}", output1);
    println!("output2: {:#?}", output2);
}
