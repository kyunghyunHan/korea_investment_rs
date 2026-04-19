#[cfg(feature = "ex")]
use dotenv::dotenv;
use korea_investment_rs::{
    domestic::trading::{DomesticIntegratedMarginRequest, DomesticTrading},
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
        .inquire_integrated_margin(DomesticIntegratedMarginRequest {
            account: &account,
            cma_evlu_amt_icld_yn: "N",
            wcrc_frcr_dvsn_cd: "02",
            fwex_ctrt_frcr_dvsn_cd: "02",
        })
        .await
        .expect("주식통합증거금 현황 조회 실패");

    println!("body = {:#?}", response.body);
}
