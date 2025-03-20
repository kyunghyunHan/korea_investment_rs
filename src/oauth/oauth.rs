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
#[derive(Debug)]
pub struct Oauth {
    pub app_key: String,
    pub app_secret: String,
}
impl Oauth {
    pub fn new(app_key: String, app_secret: String) -> Self {
        Self {
            app_key,
            app_secret,
        }
    }
    pub async fn get_access_token(&self) -> Result<String, Box<dyn Error>> {
        let client = reqwest::Client::new();

        let url = "https://openapi.koreainvestment.com:9443/oauth2/tokenP";

        let body = json!({
            "grant_type": "client_credentials",
            "appkey": self.app_key,
            "appsecret": self.app_secret
        });

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let response = client.post(url).headers(headers).json(&body).send().await?;

        let token_response: TokenResponse = response.json().await?;
        Ok(token_response.access_token)
    }
}
