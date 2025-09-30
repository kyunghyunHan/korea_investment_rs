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
//
// -------------------- 주식현재가 시세2 --------------------
//

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockPrice2Output {
    pub rprs_mrkt_kor_name: String,    // 대표 시장 한글 명
    pub new_hgpr_lwpr_cls_code: Option<String>, // 신 고가 저가 구분 코드
    pub mxpr_llam_cls_code: Option<String>,     // 상하한가 구분 코드
    pub crdt_able_yn: Option<String>,           // 신용 가능 여부
    pub stck_mxpr: Option<String>,              // 주식 상한가
    pub elw_pblc_yn: Option<String>,            // ELW 발행 여부
    pub prdy_clpr_vrss_oprc_rate: Option<String>, // 전일 종가 대비 시가2 비율
    pub crdt_rate: Option<String>,              // 신용 비율
    pub marg_rate: Option<String>,              // 증거금 비율
    pub lwpr_vrss_prpr: Option<String>,         // 최저가 대비 현재가
    pub lwpr_vrss_prpr_sign: Option<String>,    // 최저가 대비 현재가 부호
    pub prdy_clpr_vrss_lwpr_rate: Option<String>, // 전일 종가 대비 최저가 비율
    pub stck_lwpr: String,                      // 주식 최저가
    pub hgpr_vrss_prpr: Option<String>,         // 최고가 대비 현재가
    pub hgpr_vrss_prpr_sign: Option<String>,    // 최고가 대비 현재가 부호
    pub prdy_clpr_vrss_hgpr_rate: Option<String>, // 전일 종가 대비 최고가 비율
    pub stck_hgpr: String,                      // 주식 최고가
    pub oprc_vrss_prpr: Option<String>,         // 시가2 대비 현재가
    pub oprc_vrss_prpr_sign: Option<String>,    // 시가2 대비 현재가 부호
    pub mang_issu_yn: Option<String>,           // 관리 종목 여부
    pub divi_app_cls_code: Option<String>,      // 동시호가배분처리코드
    pub short_over_yn: Option<String>,          // 단기과열여부
    pub mrkt_warn_cls_code: Option<String>,     // 시장경고코드
    pub invt_caful_yn: Option<String>,          // 투자유의여부
    pub stange_runup_yn: Option<String>,        // 이상급등여부
    pub ssts_hot_yn: Option<String>,            // 공매도과열 여부
    pub low_current_yn: Option<String>,         // 저유동성 종목 여부
    pub vi_cls_code: Option<String>,            // VI적용구분코드
    pub short_over_cls_code: Option<String>,    // 단기과열구분코드
    pub stck_llam: String,                      // 주식 하한가
    pub new_lstn_cls_name: Option<String>,      // 신규 상장 구분 명
    pub vlnt_deal_cls_name: Option<String>,     // 임의 매매 구분 명
    pub flng_cls_name: Option<String>,          // 락 구분 이름
    pub revl_issu_reas_name: Option<String>,    // 재평가 종목 사유 명
    pub mrkt_warn_cls_name: Option<String>,     // 시장 경고 구분 명
    pub stck_sdpr: String,                      // 주식 기준가
    pub bstp_cls_code: String,                  // 업종 구분 코드
    pub stck_prdy_clpr: String,                 // 주식 전일 종가
    pub insn_pbnt_yn: Option<String>,           // 불성실 공시 여부
    pub fcam_mod_cls_name: Option<String>,      // 액면가 변경 구분 명
    pub stck_prpr: String,                      // 주식 현재가
    pub prdy_vrss: String,                      // 전일 대비
    pub prdy_vrss_sign: String,                 // 전일 대비 부호
    pub prdy_ctrt: String,                      // 전일 대비율
    pub acml_tr_pbmn: String,                   // 누적 거래 대금
    pub acml_vol: String,                       // 누적 거래량
    pub prdy_vrss_vol_rate: Option<String>,     // 전일 대비 거래량 비율
    pub bstp_kor_isnm: Option<String>,          // 업종 한글 종목명
    pub sltr_yn: Option<String>,                // 정리매매 여부
    pub trht_yn: Option<String>,                // 거래정지 여부
    pub oprc_rang_cont_yn: Option<String>,      // 시가 범위 연장 여부
    pub vlnt_fin_cls_code: Option<String>,      // 임의 종료 구분 코드
    pub stck_oprc: String,                      // 주식 시가2
    pub prdy_vol: String,                       // 전일 거래량
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockPrice2Response {
    pub rt_cd: String,
    pub msg_cd: String,
    pub msg1: String,
    pub output: StockPrice2Output,
}

/// 주식현재가 시세2 조회
pub async fn get_inquire_price2(
    oauth: &Oauth,
    header: &ApiHeader<'_>,
    query: QueryParam<'_>,
) -> Result<StockPrice2Output, Box<dyn Error>> {
    let url =
        "https://openapi.koreainvestment.com:9443/uapi/domestic-stock/v1/quotations/inquire-price-2";

    let response: StockPrice2Response = call_api(
        oauth,
        header,
        url,
        "FHPST01010000", // ✅ 주식현재가 시세2 전용 TR ID
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
