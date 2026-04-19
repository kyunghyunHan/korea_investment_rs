#[cfg(feature = "ex")]
use dotenv::dotenv;
use korea_investment_rs::{
    domestic::trading::{DomesticRealizedProfitBalanceRequest, DomesticTrading},
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
        .inquire_balance_realized_pl(DomesticRealizedProfitBalanceRequest {
            account: &account,
            afhr_flpr_yn: "N",
            ofl_yn: "",
            inqr_dvsn: "00",
            unpr_dvsn: "01",
            fund_sttl_icld_yn: "N",
            fncg_amt_auto_rdpt_yn: "N",
            prcs_dvsn: "00",
            cost_icld_yn: "Y",
            continuation: None,
        })
        .await
        .expect("실현손익 잔고 조회 실패");

    println!("body = {:#?}", response.body);
}
