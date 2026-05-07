#[cfg(feature = "ex")]
use dotenv::dotenv;
use korea_investment_rs::{
    futures::{DomesticFutureCcldRequest, DomesticFutureOptionTrading},
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
        .inquire_future_option_ccld(DomesticFutureCcldRequest {
            account: &account,
            start_order_date: "20260101",
            end_order_date: "20260131",
            sll_buy_dvsn_cd: "00",
            ccld_nccs_dvsn: "00",
            sort_sqn: "DS",
            pdno: "",
            start_odno: "",
            market_id_code: "",
            continuation: None,
        })
        .await
        .expect("선물옵션 체결 조회 실패");

    println!("body = {:#?}", response.body);
}
