#[cfg(feature = "ex")]
use dotenv::dotenv;
use korea_investment_rs::{
    domestic::trading::{DomesticCreditPossibleOrderRequest, DomesticTrading},
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
        .inquire_credit_possible_order(DomesticCreditPossibleOrderRequest {
            account: &account,
            pdno: "005930",
            ord_unpr: "0",
            ord_dvsn: "01",
            crdt_type: "21",
            cma_evlu_amt_icld_yn: "Y",
            ovrs_icld_yn: "N",
        })
        .await
        .expect("신용매수가능조회 실패");

    println!("body = {:#?}", response.body);
}
