#[cfg(feature = "ex")]
use dotenv::dotenv;
use korea_investment_rs::{
    domestic::quotations::{ApiHeader, QueryParam, get_inquire_price},
    oauth::Oauth,
    types::CustType,
};

#[tokio::main]
async fn main() {
    #[cfg(feature = "ex")]
    dotenv().ok();

    let practice = true;

    // 토큰 발급
    let token = Oauth::from_env_with_cache(CustType::P, practice)
        .await
        .expect("토큰 발급 실패");

    // 개인 고객용 기본 헤더
    let header = ApiHeader::personal();

    // 종목코드 005930 (삼성전자)
    let query = QueryParam::stock("005930");

    // ✅ 참조로 넘김 (&token, &header)
    let result = get_inquire_price(&token, &header, query).await.unwrap();

    println!("{:#?}", result);
}
