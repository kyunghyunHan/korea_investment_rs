#[cfg(feature = "ex")]
use dotenv::dotenv;
use korea_investment_rs::{oauth::Oauth, provider::KISProvider, types::CustType, utils::ApiHeader};

#[tokio::main]
async fn main() {
    #[cfg(feature = "ex")]
    dotenv().ok();

    let practice = true;

    // 토큰 발급
    let token = Oauth::from_env_with_cache(CustType::P, practice)
        .await
        .expect("토큰 발급 실패");
    #[cfg(feature = "ex")]
    // Provider 생성
    let provider = KISProvider {
        oauth: token,
        header: ApiHeader::personal(),
        market_type: korea_investment_rs::types::MarketType::Domestic, // 국내시장
    };
    #[cfg(feature = "ex")]
    // 종목코드 005930 (삼성전자)
    let result = provider.get_inquire_price("005930").await.unwrap();
    #[cfg(feature = "ex")]

    println!("{:#?}", result);
}
