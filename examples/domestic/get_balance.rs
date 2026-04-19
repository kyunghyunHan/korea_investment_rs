#[cfg(feature = "ex")]
use dotenv::dotenv;
use korea_investment_rs::{
    domestic::trading::{DomesticBalanceRequest, DomesticTrading},
    provider::KISProvider,
    types::{AccountInfo, MarketType},
};

#[tokio::main]
async fn main() {
    #[cfg(feature = "ex")]
    dotenv().ok();

    let provider = KISProvider::new(MarketType::Domestic, true)
        .await
        .expect("Provider 초기화 실패");
    let account = AccountInfo::from_env().expect("계좌 환경변수 로드 실패");

    let response = provider
        .inquire_balance(DomesticBalanceRequest::new(&account))
        .await
        .expect("잔고 조회 실패");

    println!("headers = {:#?}", response.headers);
    println!("body = {:#?}", response.body);
}
