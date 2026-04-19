#[cfg(feature = "ex")]
use dotenv::dotenv;
use korea_investment_rs::{
    domestic::trading::{DomesticReserveOrderListRequest, DomesticTrading},
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
        .inquire_reserve_orders(DomesticReserveOrderListRequest {
            account: &account,
            rsvn_ord_end_dt: "20261231",
            rsvn_ord_seq: "",
            tmnl_mdia_kind_cd: "00",
            prcs_dvsn_cd: "0",
            cncl_yn: "Y",
            pdno: "",
            sll_buy_dvsn_cd: "",
            continuation: None,
        })
        .await
        .expect("예약주문조회 실패");

    println!("body = {:#?}", response.body);
}
