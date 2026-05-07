#[cfg(feature = "ex")]
use dotenv::dotenv;
use korea_investment_rs::{
    overseas::{OverseasAnalysis, OverseasAnalysisEndpoint},
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
        .get_overseas_analysis_raw(
            OverseasAnalysisEndpoint::NewsTitle,
            &[
                ("INFO_GB", ""),
                ("CLASS_CD", ""),
                ("NATION_CD", "US"),
                ("EXCHANGE_CD", ""),
                ("SYMB", "AAPL"),
                ("DATA_DT", ""),
                ("DATA_TM", ""),
                ("CTS", ""),
            ],
        )
        .await
        .expect("해외뉴스종합 조회 실패");

    println!("body = {:#?}", response.body);
}
