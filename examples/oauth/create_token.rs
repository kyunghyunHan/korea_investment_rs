#[cfg(feature = "ex")]
use dotenv::dotenv;
use korea_investment_rs::{oauth::Oauth, types::CustType};

#[tokio::main]
async fn main() {
    #[cfg(feature = "ex")]
    dotenv().ok();

    let practice = true;
    let token = Oauth::from_env_with_cache(CustType::P, practice)
        .await
        .expect("토큰 발급 실패");

    println!("{:#?}", token);
}
