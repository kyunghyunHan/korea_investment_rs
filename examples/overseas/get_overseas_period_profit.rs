#[cfg(feature = "ex")]
use dotenv::dotenv;
use korea_investment_rs::{
    overseas::{OverseasPeriodProfitRequest, OverseasTrading},
    provider::KISProvider,
    types::{AccountInfo, MarketType},
};

#[tokio::main]
async fn main() {
    #[cfg(feature = "ex")]
    dotenv().ok();

    let provider = KISProvider::new(MarketType::Overseas, false)
        .await
        .expect("Provider 초기화 실패");
    let account = AccountInfo::from_env().expect("계좌 환경변수 로드 실패");

    let response = provider
        .inquire_overseas_period_profit(OverseasPeriodProfitRequest {
            account: &account,
            ovrs_excg_cd: "",
            natn_cd: "",
            crcy_cd: "",
            pdno: "",
            inqr_strt_dt: "20240101",
            inqr_end_dt: "20241231",
            wcrc_frcr_dvsn_cd: "01",
            continuation: None,
        })
        .await
        .expect("해외 기간손익 조회 실패");

    println!("body = {:#?}", response.body);
}
