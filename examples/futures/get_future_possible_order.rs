#[cfg(feature = "ex")]
use dotenv::dotenv;
use korea_investment_rs::{
    futures::{DomesticFutureOptionTrading, DomesticFuturePossibleOrderRequest},
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
        .inquire_future_option_possible_order(DomesticFuturePossibleOrderRequest {
            account: &account,
            pdno: "101S03",
            sll_buy_dvsn_cd: "02",
            unit_price: "0",
            ord_dvsn_cd: "02",
        })
        .await
        .expect("선물옵션 주문가능 조회 실패");

    println!("body = {:#?}", response.body);
}
