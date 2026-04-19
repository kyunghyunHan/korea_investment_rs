#[cfg(feature = "ex")]
use dotenv::dotenv;
use korea_investment_rs::{
    domestic::analysis::{DomesticAnalysis, DomesticRankingEndpoint},
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
        .get_ranking_raw(
            DomesticRankingEndpoint::VolumeRank,
            &[
                ("FID_COND_MRKT_DIV_CODE", "J"),
                ("FID_COND_SCR_DIV_CODE", "20171"),
                ("FID_INPUT_ISCD", "0000"),
                ("FID_DIV_CLS_CODE", "0"),
                ("FID_BLNG_CLS_CODE", "0"),
                ("FID_TRGT_CLS_CODE", "111111111"),
                ("FID_TRGT_EXLS_CLS_CODE", "000000"),
                ("FID_INPUT_PRICE_1", ""),
                ("FID_INPUT_PRICE_2", ""),
                ("FID_VOL_CNT", ""),
                ("FID_INPUT_DATE_1", ""),
            ],
        )
        .await
        .expect("거래량순위 조회 실패");

    println!("body = {:#?}", response.body);
}
