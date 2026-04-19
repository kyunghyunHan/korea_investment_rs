#[cfg(feature = "ex")]
use dotenv::dotenv;
use korea_investment_rs::{
    types::CustType,
    websocket::domestic::{DomesticRealtimeClient, DomesticRealtimeInfoType},
};

#[tokio::main]
async fn main() {
    #[cfg(feature = "ex")]
    dotenv().ok();

    let client = DomesticRealtimeClient::from_env(CustType::P)
        .await
        .expect("웹소켓 클라이언트 생성 실패");

    let (mut rx, _controller) = client
        .start_raw_channel("005930", DomesticRealtimeInfoType::StockTradeKrX)
        .await
        .expect("실시간 구독 실패");

    if let Some(message) = rx.recv().await {
        println!("{}", message.payload);
    }
}
