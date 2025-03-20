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
pub enum OauthType {
    PRACTICE,
    IMITATION,
}
#[derive(Debug)]
pub struct Oauth {
    pub app_key: String,
    pub app_secret: String,
    pub token: String,
    pub r#type: OauthType,
}
impl Oauth {
    pub async fn new(
        app_key: String,
        app_secret: String,
        r#type: OauthType,
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
            r#type,
        })
    }
}
// async fn get_samsung_stock_price(app_key: &str, app_secret: &str, access_token: &str) {
//     let client = reqwest::Client::new();

//     let url =
//         "https://openapi.koreainvestment.com:9443/uapi/domestic-stock/v1/quotations/inquire-price";

//     let mut headers = HeaderMap::new();
//     headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
//     headers.insert(
//         "authorization",
//         HeaderValue::from_str(&format!("Bearer {}", access_token)).unwrap(),
//     );
//     headers.insert("appkey", HeaderValue::from_str(app_key).unwrap());
//     headers.insert("appsecret", HeaderValue::from_str(app_secret).unwrap());
//     headers.insert("tr_id", HeaderValue::from_static("FHKST01010100")); // 주식 현재가 시세 조회

//     let response = client
//         .get(url)
//         .headers(headers)
//         .query(&[
//             ("FID_COND_MRKT_DIV_CODE", "J"), // 주식 시장 구분 코드 (J:주식)
//             ("FID_INPUT_ISCD", "005930"),    // 삼성전자 종목코드
//         ])
//         .send()
//         .await
//         .unwrap();

//     let response_json: serde_json::Value = response.json().await.unwrap();
//     println!("{}", response_json);
//     // Ok(stock_data)
// }
