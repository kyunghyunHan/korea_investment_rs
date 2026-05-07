#[cfg(feature = "ex")]
use dotenv::dotenv;
use korea_investment_rs::{
    futures::{DomesticFutureBalanceRequest, DomesticFutureOptionTrading},
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
        .inquire_future_option_balance(DomesticFutureBalanceRequest {
            account: &account,
            mgna_dvsn: "01",
            excc_stat_cd: "1",
            continuation: None,
        })
        .await
        .expect("선물옵션 잔고 조회 실패");

    println!("body = {:#?}", response.body);
}
