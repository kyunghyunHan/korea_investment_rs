#[cfg(feature = "ex")]
use dotenv::dotenv;
use korea_investment_rs::{
    types::CustType,
    websocket::overseas::{
        OverseasRealtimeClient,
        models::OverseasDelayedTransactionPriceData,
        types::OverseasRealtimeInfoType,
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "ex")]
    dotenv().ok();

    let client = OverseasRealtimeClient::from_env(CustType::P).await?;

    let controller = client
        .start_delayed_transaction_price("DNASAAPL", |data: OverseasDelayedTransactionPriceData| {
            println!("종목코드: {}", data.symb);
            println!("현재가: {}", data.last);
            println!("전일대비: {} ({}%)", data.diff, data.rate);
            println!("거래량: {}", data.tvol);
        })
        .await?;

    println!("Ctrl+C를 누르면 종료됩니다.");
    tokio::signal::ctrl_c().await?;
    controller.stop().await?;
    Ok(())
}

#[allow(dead_code)]
pub async fn example_usage() -> Result<(), Box<dyn std::error::Error>> {
    let client = OverseasRealtimeClient::from_env(CustType::P).await?;

    let controller = client
        .start_delayed_transaction_price("DNASAAPL", |data| {
            println!("실시간 데이터: {:?}", data);
        })
        .await?;

    let (mut data_rx, controller2) = client
        .start_stream_channel::<OverseasDelayedTransactionPriceData>(
            "DNASAAPL",
            OverseasRealtimeInfoType::DelayedTradePrice,
        )
        .await?;

    tokio::spawn(async move {
        while let Some(data) = data_rx.recv().await {
            println!("채널 데이터: {:?}", data);
        }
    });

    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    controller.stop().await?;
    controller2.stop().await?;
    Ok(())
}
