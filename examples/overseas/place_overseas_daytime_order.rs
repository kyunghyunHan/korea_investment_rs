#[cfg(feature = "ex")]
use dotenv::dotenv;
use korea_investment_rs::{
    overseas::{OverseasDaytimeOrderRequest, OverseasTrading},
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
        .place_overseas_daytime_buy_order(OverseasDaytimeOrderRequest {
            cano: &account.cano,
            acnt_prdt_cd: &account.acnt_prdt_cd,
            ovrs_excg_cd: "NASD",
            pdno: "AAPL",
            ord_qty: "1",
            ovrs_ord_unpr: "0",
            ctac_tlno: "",
            mgco_aptm_odno: "",
            ord_svr_dvsn_cd: "0",
            ord_dvsn: "00",
        })
        .await
        .expect("미국주간주문 실패");

    println!("body = {:#?}", response.body);
}
