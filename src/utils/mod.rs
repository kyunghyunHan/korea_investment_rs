use crate::oauth::Oauth;
use crate::types::CustType;
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue};
use reqwest::{Method, Response};
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::BTreeMap;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct ApiHeader<'a> {
    pub personalseckey: Option<&'a str>,
    pub tr_cont: Option<&'a str>,
    pub custtype: CustType,
    pub seq_no: Option<&'a str>,
    pub mac_address: Option<&'a str>,
    pub phone_number: Option<&'a str>,
    pub ip_addr: Option<&'a str>,
    pub hashkey: Option<&'a str>,
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

    pub fn with_tr_cont(mut self, tr_cont: Option<&'a str>) -> Self {
        self.tr_cont = tr_cont;
        self
    }

    pub fn with_hashkey(mut self, hashkey: Option<&'a str>) -> Self {
        self.hashkey = hashkey;
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TrId {
    pub real: &'static str,
    pub practice: Option<&'static str>,
}

impl TrId {
    pub const fn new(real: &'static str, practice: Option<&'static str>) -> Self {
        Self { real, practice }
    }

    pub fn select(self, practice: bool) -> Result<&'static str, Box<dyn Error>> {
        if practice {
            self.practice
                .ok_or_else(|| "모의투자 미지원 API입니다".into())
        } else {
            Ok(self.real)
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ApiEndpoint {
    pub path: &'static str,
    pub tr_id: TrId,
    pub real_domain: &'static str,
    pub practice_domain: Option<&'static str>,
}

impl ApiEndpoint {
    pub const fn new(path: &'static str, tr_id: TrId) -> Self {
        Self {
            path,
            tr_id,
            real_domain: "https://openapi.koreainvestment.com:9443",
            practice_domain: Some("https://openapivts.koreainvestment.com:29443"),
        }
    }

    pub const fn real_only(path: &'static str, tr_id: TrId) -> Self {
        Self {
            path,
            tr_id,
            real_domain: "https://openapi.koreainvestment.com:9443",
            practice_domain: None,
        }
    }

    pub fn url(self, practice: bool) -> Result<String, Box<dyn Error>> {
        let domain = if practice {
            self.practice_domain
                .ok_or_else(|| "모의투자 미지원 API입니다".to_string())?
        } else {
            self.real_domain
        };
        Ok(format!("{domain}{}", self.path))
    }
}

#[derive(Debug, Clone)]
pub struct ResponseHeaders {
    pub tr_id: Option<String>,
    pub tr_cont: Option<String>,
    pub gt_uid: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ApiResponse<T> {
    pub headers: ResponseHeaders,
    pub body: T,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct RawApiBody {
    pub rt_cd: String,
    pub msg_cd: String,
    pub msg1: String,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

impl RawApiBody {
    pub fn ensure_success(&self) -> Result<(), Box<dyn Error>> {
        if self.rt_cd == "0" {
            Ok(())
        } else {
            Err(format!("API 오류: {} ({})", self.msg1, self.msg_cd).into())
        }
    }
}

fn build_headers(
    oauth: &Oauth,
    header: &ApiHeader<'_>,
    tr_id: &str,
) -> Result<HeaderMap, Box<dyn Error>> {
    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/json; charset=utf-8"),
    );
    headers.insert(
        "authorization",
        HeaderValue::from_str(&format!("Bearer {}", oauth.token))?,
    );
    headers.insert("appkey", HeaderValue::from_str(&oauth.app_key)?);
    headers.insert("appsecret", HeaderValue::from_str(&oauth.app_secret)?);
    headers.insert("tr_id", HeaderValue::from_str(tr_id)?);
    headers.insert(
        "custtype",
        HeaderValue::from_static(match header.custtype {
            CustType::B => "B",
            CustType::P => "P",
        }),
    );

    if let Some(value) = header.personalseckey {
        headers.insert("personalseckey", HeaderValue::from_str(value)?);
    }
    if let Some(value) = header.tr_cont {
        headers.insert("tr_cont", HeaderValue::from_str(value)?);
    }
    if let Some(value) = header.seq_no {
        headers.insert("seq_no", HeaderValue::from_str(value)?);
    }
    if let Some(value) = header.mac_address {
        headers.insert("mac_address", HeaderValue::from_str(value)?);
    }
    if let Some(value) = header.phone_number {
        headers.insert("phone_number", HeaderValue::from_str(value)?);
    }
    if let Some(value) = header.ip_addr {
        headers.insert("ip_addr", HeaderValue::from_str(value)?);
    }
    if let Some(value) = header.hashkey {
        headers.insert("hashkey", HeaderValue::from_str(value)?);
    }
    if let Some(value) = header.gt_uid {
        headers.insert("gt_uid", HeaderValue::from_str(value)?);
    }

    Ok(headers)
}

fn response_headers(response: &Response) -> ResponseHeaders {
    let headers = response.headers();
    ResponseHeaders {
        tr_id: headers
            .get("tr_id")
            .and_then(|value| value.to_str().ok())
            .map(ToOwned::to_owned),
        tr_cont: headers
            .get("tr_cont")
            .and_then(|value| value.to_str().ok())
            .map(ToOwned::to_owned),
        gt_uid: headers
            .get("gt_uid")
            .and_then(|value| value.to_str().ok())
            .map(ToOwned::to_owned),
    }
}

async fn send_request<T: DeserializeOwned, B: Serialize + ?Sized>(
    oauth: &Oauth,
    header: &ApiHeader<'_>,
    practice: bool,
    endpoint: ApiEndpoint,
    method: Method,
    query: &[(&str, &str)],
    body: Option<&B>,
) -> Result<ApiResponse<T>, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let url = endpoint.url(practice)?;
    let tr_id = endpoint.tr_id.select(practice)?;
    let headers = build_headers(oauth, header, tr_id)?;

    let mut request = client.request(method, &url).headers(headers).query(query);
    if let Some(body) = body {
        request = request.json(body);
    }

    let response = request.send().await?;
    let meta = response_headers(&response);
    let status = response.status();
    if !status.is_success() {
        let error_text = response.text().await?;
        return Err(format!("API 요청 실패 ({}): {}", status, error_text).into());
    }

    let response_data: T = response.json().await?;
    Ok(ApiResponse {
        headers: meta,
        body: response_data,
    })
}

pub async fn call_get_api<T: DeserializeOwned>(
    oauth: &Oauth,
    header: &ApiHeader<'_>,
    practice: bool,
    endpoint: ApiEndpoint,
    query: &[(&str, &str)],
) -> Result<ApiResponse<T>, Box<dyn Error>> {
    send_request::<T, Value>(
        oauth,
        header,
        practice,
        endpoint,
        Method::GET,
        query,
        None,
    )
    .await
}

pub async fn call_post_api<T: DeserializeOwned, B: Serialize + ?Sized>(
    oauth: &Oauth,
    header: &ApiHeader<'_>,
    practice: bool,
    endpoint: ApiEndpoint,
    body: &B,
) -> Result<ApiResponse<T>, Box<dyn Error>> {
    send_request(
        oauth,
        header,
        practice,
        endpoint,
        Method::POST,
        &[],
        Some(body),
    )
    .await
}

pub async fn call_api<T: DeserializeOwned>(
    oauth: &Oauth,
    header: &ApiHeader<'_>,
    url: &str,
    tr_id: &str,
    query: &[(&str, &str)],
) -> Result<T, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let headers = build_headers(oauth, header, tr_id)?;
    let response = client.get(url).headers(headers).query(query).send().await?;
    let status = response.status();
    if !status.is_success() {
        let error_text = response.text().await?;
        return Err(format!("API 요청 실패 ({}): {}", status, error_text).into());
    }
    Ok(response.json().await?)
}

pub async fn create_hashkey<T: Serialize>(
    oauth: &Oauth,
    practice: bool,
    body: &T,
) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let domain = if practice {
        "https://openapivts.koreainvestment.com:29443"
    } else {
        "https://openapi.koreainvestment.com:9443"
    };
    let response = client
        .post(format!("{domain}/uapi/hashkey"))
        .header(CONTENT_TYPE, "application/json; charset=utf-8")
        .header("appkey", &oauth.app_key)
        .header("appsecret", &oauth.app_secret)
        .json(body)
        .send()
        .await?;

    let status = response.status();
    if !status.is_success() {
        let error_text = response.text().await?;
        return Err(format!("Hashkey 요청 실패 ({}): {}", status, error_text).into());
    }

    let payload: Value = response.json().await?;
    payload
        .get("HASH")
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
        .ok_or_else(|| "Hashkey 응답에 HASH 필드가 없습니다".into())
}
