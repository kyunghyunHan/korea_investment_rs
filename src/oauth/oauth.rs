use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;
#[derive(Deserialize, Debug)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: i32,
}
pub struct Oauth {
    app_key: String,
    app_secret: String,
}
impl Oauth {
    async fn get_access_token(app_key: &str, app_secret: &str) -> Result<String, Box<dyn Error>> {
        let client = reqwest::Client::new();

        let url = "https://openapi.koreainvestment.com:9443/oauth2/tokenP";

        let body = json!({
            "grant_type": "client_credentials",
            "appkey": app_key,
            "appsecret": app_secret
        });

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let response = client.post(url).headers(headers).json(&body).send().await?;

        let token_response: TokenResponse = response.json().await?;
        println!("{:?}", token_response);
        Ok(token_response.access_token)
    }
}
