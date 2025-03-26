use korea_investment_rs::types::CustType;
use korea_investment_rs::websocket::overseas::{
    OverseasRealtimeClient, OverseasRealtimeData, OverseasRealtimeInfoType,
};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 클라이언트 생성
    let client = OverseasRealtimeClient::from_env(CustType::P).await?;

    // 애플 주식 데이터 스트림 시작 (나스닥)
    // OverseasRealtimeData 타입으로 데이터를 받음
    let controller = client
        .start_stream::<OverseasRealtimeData>(
            "RBAQAAPL", // 애플 주식 심볼
            // OverseasRealtimeInfoType::RealTimeQuote,
            |data| {
                // 콜백 함수에서 데이터 처리
                println!(
                    "종목: {}, 현재가: {}, 등락률: {}%",
                    data.symb, data.last, data.rate
                );

                // 현재가가 특정 값을 초과하면 알림 생성
                if let Ok(price) = data.last.parse::<f64>() {
                    if price > 180.0 {
                        println!("주의: 애플 주가 180달러 초과!");
                    }
                }
            },
        )
        .await?;

        println!("Ctrl+C를 눌러 종료하세요.");

        // 프로그램이 종료되지 않도록 대기
        tokio::signal::ctrl_c().await?;
    
        // 스트림 중지
        controller.stop().await?;
        println!("스트림이 중지되었습니다.");

    Ok(())
}
