use crate::oauth::Oauth;
use crate::types::CustType;
use reqwest::header::CONTENT_TYPE;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::error::Error;

//
// -------------------- 공통 Header --------------------
//
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiHeader<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personalseckey: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tr_cont: Option<&'a str>,
    pub custtype: CustType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seq_no: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_addr: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hashkey: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gt_uid: Option<&'a str>,
}

impl<'a> ApiHeader<'a> {
    pub fn personal() -> Self {
        Self {
            personalseckey: None,
            tr_cont: None,
            custtype: CustType::P,
            seq_no: None,
            mac_address: None,
            phone_number: None,
            ip_addr: None,
            hashkey: None,
            gt_uid: None,
        }
    }
}

/// 공통 API 호출 함수
pub async fn call_api<T: DeserializeOwned>(
    oauth: &Oauth,
    header: &ApiHeader<'_>,
    url: &str,
    tr_id: &str,
    query: &[(&str, &str)],
) -> Result<T, Box<dyn Error>> {
    let client = reqwest::Client::new();

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(
        "authorization",
        HeaderValue::from_str(&format!("Bearer {}", oauth.token))?,
    );
    headers.insert("appkey", HeaderValue::from_str(&oauth.app_key)?);
    headers.insert("appsecret", HeaderValue::from_str(&oauth.app_secret)?);
    if let Some(pk) = header.personalseckey {
        headers.insert("personalseckey", HeaderValue::from_str(pk)?);
    }
    headers.insert("tr_id", HeaderValue::from_str(tr_id)?);

    let response = client.get(url).headers(headers).query(query).send().await?;
    let status = response.status();
    if !status.is_success() {
        let error_text = response.text().await?;
        return Err(format!("API 요청 실패 ({}): {}", status, error_text).into());
    }

    let response_data: T = response.json().await?;
    Ok(response_data)
}
