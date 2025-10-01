#[cfg(feature = "ex")]
use dotenv::dotenv;
use korea_investment_rs::{
    domestic::quotations::{QueryParam, get_inquire_price},
    oauth::Oauth,
    types::CustType,
    utils::ApiHeader,
};

#[tokio::main]
async fn main() {
    #[cfg(feature = "ex")]
    dotenv().ok();

    // PRACTICE(모의투자) 여부 선택
    let practice = true;

    // .env 기반으로 개인 고객(P) 토큰 발급
    let token = Oauth::from_env_with_cache(CustType::P, practice)
        .await
        .expect("토큰 발급 실패");
    // 종목코드: 삼성전자(005930)
    let query = QueryParam::stock("005930");
    // 개인 고객용 기본 헤더
    let header = ApiHeader::personal();

    // ✅ 참조로 넘겨야 함 (&token, &header)
    let result = get_inquire_price(&token, &header, query)
        .await
        .expect("조회 실패");

    println!("{:#?}", result);
}
