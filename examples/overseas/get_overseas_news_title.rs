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
                ("AUTH", ""),
                ("EXCD", "NAS"),
                ("SYMB", "AAPL"),
                ("GUBN", "0"),
                ("BYMD", ""),
                ("MODP", "0"),
            ],
        )
        .await
        .expect("해외뉴스종합 조회 실패");

    println!("body = {:#?}", response.body);
}
