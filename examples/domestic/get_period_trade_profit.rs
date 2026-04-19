#[cfg(feature = "ex")]
use dotenv::dotenv;
use korea_investment_rs::{
    domestic::trading::{DomesticPeriodTradeProfitRequest, DomesticTrading},
    provider::KISProvider,
    types::{AccountInfo, MarketType},
};

#[tokio::main]
async fn main() {
    #[cfg(feature = "ex")]
    dotenv().ok();

    let provider = KISProvider::new(MarketType::Domestic, false)
        .await
        .expect("Provider 초기화 실패");
    let account = AccountInfo::from_env().expect("계좌 환경변수 로드 실패");

    let response = provider
        .inquire_period_trade_profit(DomesticPeriodTradeProfitRequest {
            account: &account,
            sort_dvsn: "00",
            pdno: "",
            inqr_strt_dt: "20240101",
            inqr_end_dt: "20241231",
            cblc_dvsn: "00",
            continuation: None,
        })
        .await
        .expect("기간별 매매손익 조회 실패");

    println!("body = {:#?}", response.body);
}
