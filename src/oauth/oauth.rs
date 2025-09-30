use crate::types::CustType;
#[cfg(feature = "ex")]
use dotenv::dotenv;
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue};
use serde::{self, Deserialize, Serialize};
use serde_json::json;
use std::{
    env, error::Error, fs,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Deserialize, Debug)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: i32,
    access_token_token_expired: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CachedToken {
    token: String,
    created_at: u64,   // 발급된 시각 (UNIX timestamp)
    expires_in: i32,   // 만료 시간 (초 단위)
}

#[derive(Debug, Serialize)]
pub struct Oauth {
    pub app_key: String,
    pub app_secret: String,
    pub token: String,
    pub cust_type: CustType,
}

impl Oauth {
    /// 캐시된 토큰 불러오기
    fn load_cached_token() -> Option<CachedToken> {
        let data = fs::read_to_string("token.json").ok()?;
        serde_json::from_str(&data).ok()
    }

    /// 토큰 발급 (API 호출)
    async fn issue_new_token(
        app_key: String,
        app_secret: String,
        cust_type: CustType,
        practice: bool,
    ) -> Result<Self, Box<dyn Error>> {
        let client = reqwest::Client::new();
        let domain = if practice {
            "https://openapivts.koreainvestment.com:29443"
        } else {
            "https://openapi.koreainvestment.com:9443"
        };
        let url = format!("{}/oauth2/tokenP", domain);

        let body = json!({
            "grant_type": "client_credentials",
            "appkey": app_key,
            "appsecret": app_secret
        });

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json; charset=UTF-8"));

        let response = client.post(&url).headers(headers).json(&body).send().await?;
        let token_response: TokenResponse = response.json().await?;

        // 캐시 저장
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let cached = CachedToken {
            token: token_response.access_token.clone(),
            created_at: now,
            expires_in: token_response.expires_in,
        };
        fs::write("token.json", serde_json::to_string_pretty(&cached)?)?;

        Ok(Self {
            app_key,
            app_secret,
            token: token_response.access_token,
            cust_type,
        })
    }

    /// 환경변수 + 캐시 활용 (자동 업데이트 포함)
    pub async fn from_env_with_cache(cust_type: CustType, practice: bool) -> Result<Self, Box<dyn Error>> {
        #[cfg(feature = "ex")]
        dotenv().ok();

        let app_key = env::var("PUB_KEY").expect("PUB_KEY not set in .env file");
        let app_secret = env::var("SCREST_KEY").expect("SCREST_KEY not set in .env file");

        if let Some(cached) = Self::load_cached_token() {
            let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
            let expiry_time = cached.created_at + cached.expires_in as u64;

            if now < expiry_time {
                // 아직 유효
                println!("⏳ Token still valid, using cached token");
                return Ok(Self {
                    app_key,
                    app_secret,
                    token: cached.token,
                    cust_type,
                });
            } else {
                // 만료 → 새 토큰 발급
                println!("🔄 Token expired, requesting new one...");
                return Self::issue_new_token(app_key, app_secret, cust_type, practice).await;
            }
        }

        // 캐시 없으면 새로 발급
        println!("🆕 No token.json found, requesting new one...");
        Self::issue_new_token(app_key, app_secret, cust_type, practice).await
    }
}
