#[cfg(feature = "ex")]
use dotenv::dotenv;
use korea_investment_rs::{
    overseas::{OverseasReserveListMarket, OverseasReserveOrderListRequest, OverseasTrading},
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
        .inquire_overseas_reserve_orders(OverseasReserveOrderListRequest {
            account: &account,
            inqr_strt_dt: "20260101",
            inqr_end_dt: "20261231",
            inqr_dvsn_cd: "00",
            prdt_type_cd: "",
            ovrs_excg_cd: "",
            market: OverseasReserveListMarket::Usa,
            continuation: None,
        })
        .await
        .expect("해외 예약주문조회 실패");

    println!("body = {:#?}", response.body);
}
