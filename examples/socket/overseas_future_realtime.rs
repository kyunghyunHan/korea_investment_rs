#[cfg(feature = "ex")]
use dotenv::dotenv;
use korea_investment_rs::{
    types::CustType,
    websocket::overseas::{OverseasRealtimeClient, types::OverseasRealtimeInfoType},
};

#[tokio::main]
async fn main() {
    #[cfg(feature = "ex")]
    dotenv().ok();

    let client = OverseasRealtimeClient::from_env(CustType::P)
        .await
        .expect("웹소켓 클라이언트 생성 실패");

    let (mut rx, controller) = client
        .start_raw_channel("ESM26", OverseasRealtimeInfoType::FutureOptionTrade)
        .await
        .expect("해외선물옵션 실시간 구독 실패");

    println!("Ctrl+C를 누르면 종료됩니다.");
    loop {
        tokio::select! {
            message = rx.recv() => {
                if let Some(message) = message {
                    println!("{}", message.payload);
                } else {
                    break;
                }
            }
            _ = tokio::signal::ctrl_c() => {
                controller.stop().await.expect("웹소켓 중지 실패");
                break;
            }
        }
    }
}
