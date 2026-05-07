#[cfg(feature = "ex")]
use dotenv::dotenv;
use korea_investment_rs::{
    futures::{OverseasFutureOptionGetEndpoint, OverseasFutureOptionTrading},
    provider::KISProvider,
    types::MarketType,
};

#[tokio::main]
async fn main() {
    #[cfg(feature = "ex")]
    dotenv().ok();

    let provider = KISProvider::new(MarketType::Overseas, false)
        .await
        .expect("Provider 초기화 실패");

    let response = provider
        .get_overseas_future_option_raw(
            OverseasFutureOptionGetEndpoint::Price,
            &[("SRS_CD", "ESM26")],
        )
        .await
        .expect("해외선물옵션 현재가 조회 실패");

    println!("body = {:#?}", response.body);
}
