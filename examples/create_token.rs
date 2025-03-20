use korea_investment_rs::oauth::Oauth;
#[tokio::main]
async fn main() {
    let app_key = "".to_string();
    let app_secret = "".to_string();
    let oauth = Oauth::new(app_key, app_secret);
    let token = Oauth::get_access_token(&oauth).await.unwrap();
}
