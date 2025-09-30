#[cfg(feature = "ex")]
use dotenv::dotenv;
use korea_investment_rs::{
    domestic::quotations::{
        ApiHeader, IDIQuery, get_inquire_daily_itemchartprice, get_inquire_price,
    },
    oauth::{Oauth},
    types::CustType,
};

#[tokio::main]
async fn main() {
    #[cfg(feature = "ex")]
    dotenv().ok();

    // PRACTICE(모의투자) 여부 플래그
    let practice = true;

    // 개인 고객(P) 기준 토큰 발급
    let token = Oauth::from_env(CustType::P, practice)
        .await
        .expect("Failed to get token");

    println!("발급된 토큰: {:?}", token);

    // 예시: 일봉 차트 조회
    let query = IDIQuery::new("J", "005930", "20220101", "20220531", "D", "0");
    let header =
        ApiHeader::new(CustType::P, None, None, None, None, None, None, None, None).unwrap();
    let result = get_inquire_daily_itemchartprice(token, header, query)
        .await
        .unwrap();

    println!("조회 결과: {:?}", result);
}
