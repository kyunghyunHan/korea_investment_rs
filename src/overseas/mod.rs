use crate::oauth::Oauth;
use crate::types::CustType;
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::error::Error;

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

async fn call_api<T: DeserializeOwned>(
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
//
// -------------------- 해외주식 현재가 조회 --------------------
//
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverseasPriceQuery<'a> {
    #[serde(rename = "AUTH")]
    pub auth: &'a str, // 사용자 권한정보
    #[serde(rename = "EXCD")]
    pub exchg_code: &'a str, // 거래소 (NYS, NAS, AMS, HKS, TSE 등)
    #[serde(rename = "SYMB")]
    pub symbol: &'a str, // 종목코드
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverseasPriceOutput {
    pub rsym: String, // 실시간조회종목코드
    pub open: String, // 시가
    pub high: String, // 고가
    pub low: String,  // 저가
    pub last: String, // 현재가
    pub base: String, // 전일종가
    pub tvol: String, // 거래량
    pub tamt: String, // 거래대금
    pub perx: String, // PER
    pub pbrx: String, // PBR
    pub epsx: String, // EPS
    pub bpsx: String, // BPS
    pub curr: String, // 통화
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverseasPriceResponse {
    pub rt_cd: String,
    pub msg_cd: String,
    pub msg1: String,
    pub output: OverseasPriceOutput,
}

pub async fn get_overseas_price(
    oauth: &Oauth,
    header: &ApiHeader<'_>,
    query: OverseasPriceQuery<'_>,
) -> Result<OverseasPriceOutput, Box<dyn Error>> {
    let url = "https://openapi.koreainvestment.com:9443/uapi/overseas-price/v1/quotations/price-detail";

    let response: OverseasPriceResponse = call_api(
        oauth,
        header,
        url,
        "HHDFS76200200", // 해외현재가
        &[("AUTH", query.auth), ("EXCD", query.exchg_code), ("SYMB", query.symbol)],
    )
    .await?;

    if response.rt_cd != "0" {
        return Err(format!("API 응답 오류: {} ({})", response.msg1, response.msg_cd).into());
    }

    Ok(response.output)
}

//
// -------------------- 해외주식 일봉 차트 조회 --------------------
//
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverseasDailyQuery<'a> {
    #[serde(rename = "AUTH")]
    pub auth: &'a str,
    #[serde(rename = "EXCD")]
    pub exchg_code: &'a str,
    #[serde(rename = "SYMB")]
    pub symbol: &'a str,
    #[serde(rename = "GUBN")]
    pub gubn: &'a str, // 기간구분 (0:일, 1:주, 2:월)
    #[serde(rename = "BYMD")]
    pub bymd: &'a str, // 조회 시작일 (yyyymmdd)
    #[serde(rename = "MODP")]
    pub modp: &'a str, // 수정주가 반영여부 (0:미반영, 1:반영)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverseasDailyOutput {
    pub xymd: String, // 기준일자
    pub open: String,
    pub high: String,
    pub low: String,
    pub last: String,
    pub tvol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverseasDailyResponse {
    pub rt_cd: String,
    pub msg_cd: String,
    pub msg1: String,
    pub output: Vec<OverseasDailyOutput>,
}

pub async fn get_overseas_dailyprice(
    oauth: &Oauth,
    header: &ApiHeader<'_>,
    query: OverseasDailyQuery<'_>,
) -> Result<Vec<OverseasDailyOutput>, Box<dyn Error>> {
    let url = "https://openapi.koreainvestment.com:9443/uapi/overseas-price/v1/quotations/dailyprice";

    let response: OverseasDailyResponse = call_api(
        oauth,
        header,
        url,
        "HHDFS76240000", // 해외일별시세
        &[
            ("AUTH", query.auth),
            ("EXCD", query.exchg_code),
            ("SYMB", query.symbol),
            ("GUBN", query.gubn),
            ("BYMD", query.bymd),
            ("MODP", query.modp),
        ],
    )
    .await?;

    if response.rt_cd != "0" {
        return Err(format!("API 응답 오류: {} ({})", response.msg1, response.msg_cd).into());
    }

    Ok(response.output)
}
