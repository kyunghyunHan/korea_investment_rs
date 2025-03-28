use crate::types::CustType;
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use std::error::Error;
#[derive(Deserialize, Debug)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: i32,
}
#[derive(Debug)]
pub enum OauthType {
    PRACTICE,
    IMITATION,
}

#[derive(Debug)]
pub struct Oauth {
    pub app_key: String,
    pub app_secret: String,
    pub token: String,
    pub cust_type: CustType,
}
impl Oauth {
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

    /// 새로운 클라이언트 생성
    pub async fn new(
        app_key: String,
        app_secret: String,
        cust_type: CustType,
    ) -> Result<Self, Box<dyn Error>> {
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

        Ok(Self {
            app_key,
            app_secret,
            token: (token_response.access_token),
            cust_type,
        })
    }

    /// 환경 변수에서 클라이언트 생성
    pub async fn from_env(cust_type: CustType) -> Result<Self, Box<dyn Error>> {
        {
            #[cfg(feature = "ex")]
            dotenv().ok();

            let app_key = env::var("PUB_KEY").expect("APP_KEY not set in .env file");
            let app_secret = env::var("SCREST_KEY").expect("APP_SECRET not set in .env file");
            let client = reqwest::Client::new();

            let url = "https://openapi.koreainvestment.com:9443/oauth2/Approval";

            let body = json!({
                "grant_type": "client_credentials",
                "appkey": app_key,
                "secretkey": app_secret
            });

            let mut headers = HeaderMap::new();
            headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

            let response = client.post(url).headers(headers).json(&body).send().await?;

            let token_response: TokenResponse = response.json().await?;
            Ok(Self {
                app_key,
                app_secret,
                token: (token_response.access_token),
                cust_type,
            })
        }
    }
}
