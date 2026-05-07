#[cfg(feature = "ex")]
use dotenv::dotenv;
use korea_investment_rs::{
    futures::{DomesticFutureOptionTrading, DomesticFuturePriceRequest},
    provider::KISProvider,
    types::MarketType,
};

#[tokio::main]
async fn main() {
    #[cfg(feature = "ex")]
    dotenv().ok();

    let provider = KISProvider::new(MarketType::Domestic, false)
        .await
        .expect("Provider 초기화 실패");

    let response = provider
        .get_future_option_price(DomesticFuturePriceRequest {
            market_div_code: "F",
            input_iscd: "101S03",
        })
        .await
        .expect("선물옵션 현재가 조회 실패");

    println!("body = {:#?}", response.body);
}
