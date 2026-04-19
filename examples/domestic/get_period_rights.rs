#[cfg(feature = "ex")]
use dotenv::dotenv;
use korea_investment_rs::{
    domestic::trading::{DomesticPeriodRightsRequest, DomesticTrading},
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
        .inquire_period_rights(DomesticPeriodRightsRequest {
            account: &account,
            cust_rncno25: "",
            hmid: "",
            inqr_strt_dt: "20240101",
            inqr_end_dt: "20241231",
            rght_type_cd: "",
            pdno: "",
            prdt_type_cd: "",
            continuation: None,
        })
        .await
        .expect("기간별계좌권리현황조회 실패");

    println!("body = {:#?}", response.body);
}
