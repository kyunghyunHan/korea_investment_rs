use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;
#[derive(Deserialize, Debug)]
struct TokenResponse {
    approval_key: String,
}

#[derive(Debug)]
pub struct ApproveOauth {
    pub app_key: String,
    pub app_secret: String,
    pub approval_key: String,
}
impl ApproveOauth {
    /// create Oauth
    ///
    /// # Examples
    /// ```
    ///     dotenv().ok();
    ///     let app_key = env::var("PUB_KEY").expect("APP_KEY not set in .env file");
    ///     let app_secret = env::var("SCREST_KEY").expect("APP_SECRET not set in .env file");
    ///     let r#type = OauthType::PRACTICE;
    ///     let token = Oauth::new(app_key, app_secret, r#type).await.unwrap();
    ///     println!("{:?}", token);
    /// ```
    pub async fn new(app_key: String, app_secret: String) -> Result<Self, Box<dyn Error>> {
        let client = reqwest::Client::new();

        let url = "https://openapi.koreainvestment.com:9443/oauth2/Approval";

        let body = json!({
            "grant_type": "client_credentials",
            "appkey": app_key,
            "appsecret": app_secret
        });

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let response = client.post(url).headers(headers).json(&body).send().await?;

        let approval_response: TokenResponse = response.json().await?;

        Ok(Self {
            app_key,
            app_secret,
            approval_key: (approval_response.approval_key),
        })
    }
}
