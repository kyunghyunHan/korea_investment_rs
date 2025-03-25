use korea_investment_rs::websocket::overseas::OverseasRealtimeClient;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 방법 1: 환경 변수에서 키 로드
    let client = OverseasRealtimeClient::from_env()?;

    // 방법 2: 직접 키 제공
    // let client = OverseasRealtimeClient::new(
    //     "your_app_key".to_string(),
    //     "your_app_secret".to_string()
    // );

    println!("해외 실시간 데이터 수신 시작...");

    // 콜백 함수를 사용하여 데이터 처리
    let controller = client
        .start_stream("DNASAAPL", |data| {
            println!("애플 실시간 데이터:");
            println!("  종목코드: {}", data.symb);
            println!("  현재가: {}", data.last);
            println!("  전일대비: {} ({}%)", data.diff, data.rate);
            println!("  거래량: {}", data.tvol);
        })
        .await?;

    // 또는 채널을 사용하여 데이터 처리
    // let (mut data_rx, controller) = client.start_stream_channel("DNASAAPL").await?;
    //
    // tokio::spawn(async move {
    //     while let Some(data) = data_rx.recv().await {
    //         println!("애플 실시간 데이터: 현재가 {}, 등락률 {}%", data.last, data.rate);
    //     }
    // });

    println!("Ctrl+C를 눌러 종료하세요.");

    // 프로그램이 종료되지 않도록 대기
    tokio::signal::ctrl_c().await?;

    // 스트림 중지
    controller.stop().await?;
    println!("스트림이 중지되었습니다.");

    Ok(())
}
