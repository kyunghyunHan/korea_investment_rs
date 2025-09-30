use crate::oauth::Oauth;
use crate::types::CustType;
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::error::Error;

/// 공통 API 호출 함수
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

//
// -------------------- 현재가 조회 --------------------
//
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryParam<'a> {
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub market_division_code: &'a str,
    #[serde(rename = "FID_INPUT_ISCD")]
    pub stock_code: &'a str,
}

impl<'a> QueryParam<'a> {
    pub fn stock(stock_code: &'a str) -> Self {
        Self {
            market_division_code: "J",
            stock_code,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockPriceOutput {
    pub stck_prpr: String,    // 현재가
    pub prdy_vrss: String,    // 전일 대비
    pub prdy_ctrt: String,    // 전일 대비율
    pub acml_tr_pbmn: String, // 누적 거래대금
    pub acml_vol: String,     // 누적 거래량
    pub stck_oprc: String,    // 시가
    pub stck_hgpr: String,    // 고가
    pub stck_lwpr: String,    // 저가
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockPriceResponse {
    pub rt_cd: String,
    pub msg_cd: String,
    pub msg1: String,
    pub output: StockPriceOutput,
}

pub async fn get_inquire_price(
    oauth: &Oauth,
    header: &ApiHeader<'_>,
    query: QueryParam<'_>,
) -> Result<StockPriceOutput, Box<dyn Error>> {
    let url =
        "https://openapi.koreainvestment.com:9443/uapi/domestic-stock/v1/quotations/inquire-price";

    let response: StockPriceResponse = call_api(
        oauth,
        header,
        url,
        "FHKST01010100",
        &[
            ("FID_COND_MRKT_DIV_CODE", query.market_division_code),
            ("FID_INPUT_ISCD", query.stock_code),
        ],
    )
    .await?;

    if response.rt_cd != "0" {
        return Err(format!("API 응답 오류: {} ({})", response.msg1, response.msg_cd).into());
    }

    Ok(response.output)
}

//
// -------------------- 일봉 차트 조회 --------------------
//
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IDIQuery<'a> {
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mark_div_code: &'a str,
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: &'a str,
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: &'a str,
    #[serde(rename = "FID_INPUT_DATE_2")]
    pub fid_input_date_2: &'a str,
    #[serde(rename = "FID_PERIOD_DIV_CODE")]
    pub fid_period_div_code: &'a str,
    #[serde(rename = "FID_ORG_ADJ_PRC")]
    pub fid_org_adj_prc: &'a str,
}

impl<'a> IDIQuery<'a> {
    pub fn daily(iscd: &'a str, from: &'a str, to: &'a str) -> Self {
        Self {
            fid_cond_mark_div_code: "J",
            fid_input_iscd: iscd,
            fid_input_date_1: from,
            fid_input_date_2: to,
            fid_period_div_code: "D", // 일봉
            fid_org_adj_prc: "0",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ITIOutput1 {
    pub stck_prpr: String,    // 현재가
    pub acml_tr_pbmn: String, // 거래대금
    pub acml_vol: String,     // 거래량
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ITIOutput2 {
    pub stck_bsop_date: String, // 기준일
    pub stck_clpr: String,      // 종가
    pub stck_oprc: String,      // 시가
    pub stck_hgpr: String,      // 고가
    pub stck_lwpr: String,      // 저가
    pub acml_vol: String,       // 거래량
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ITIResponse {
    pub rt_cd: String,
    pub msg_cd: String,
    pub msg1: String,
    pub output1: ITIOutput1,
    pub output2: Vec<ITIOutput2>,
}

pub async fn get_inquire_daily_itemchartprice(
    oauth: &Oauth,
    header: &ApiHeader<'_>,
    query: IDIQuery<'_>,
) -> Result<ITIResponse, Box<dyn Error>> {
    let url = "https://openapi.koreainvestment.com:9443/uapi/domestic-stock/v1/quotations/inquire-daily-itemchartprice";

    let response: ITIResponse = call_api(
        oauth,
        header,
        url,
        "FHKST03010100",
        &[
            ("FID_COND_MRKT_DIV_CODE", query.fid_cond_mark_div_code),
            ("FID_INPUT_ISCD", query.fid_input_iscd),
            ("FID_INPUT_DATE_1", query.fid_input_date_1),
            ("FID_INPUT_DATE_2", query.fid_input_date_2),
            ("FID_PERIOD_DIV_CODE", query.fid_period_div_code),
            ("FID_ORG_ADJ_PRC", query.fid_org_adj_prc),
        ],
    )
    .await?;

    if response.rt_cd != "0" {
        return Err(format!("API 응답 오류: {} ({})", response.msg1, response.msg_cd).into());
    }

    Ok(response)
}
