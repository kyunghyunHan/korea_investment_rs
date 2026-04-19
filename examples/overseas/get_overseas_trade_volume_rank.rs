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
            OverseasAnalysisEndpoint::TradeVolume,
            &[
                ("AUTH", ""),
                ("EXCD", "NAS"),
                ("NREC", "20"),
                ("CO_YN_PRICECUR", "0"),
                ("KEYB", ""),
            ],
        )
        .await
        .expect("해외 거래량순위 조회 실패");

    println!("body = {:#?}", response.body);
}
