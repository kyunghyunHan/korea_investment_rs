#[cfg(feature = "ex")]
use dotenv::dotenv;
use korea_investment_rs::{
    domestic::trading::{DomesticTrading, PensionBalanceRequest},
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
        .inquire_pension_balance(PensionBalanceRequest {
            account: &account,
            acca_dvsn_cd: "00",
            inqr_dvsn: "00",
            continuation: None,
        })
        .await
        .expect("퇴직연금 잔고조회 실패");

    println!("body = {:#?}", response.body);
}
