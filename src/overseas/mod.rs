use crate::oauth::Oauth;
use crate::types::CustType;
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::error::Error;
use crate::utils::{ApiHeader, call_api};


// ========================================================
// 1. 해외주식 현재가
// ========================================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverseasPriceQuery<'a> {
    #[serde(rename = "AUTH")]
    pub auth: &'a str, // "P"
    #[serde(rename = "EXCD")]
    pub exchg_code: &'a str, // NAS, NYS, AMS, HKS ...
    #[serde(rename = "SYMB")]
    pub symbol: &'a str, // AAPL, TSLA ...
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverseasPriceOutput {
    pub rsym: String,
    pub open: String,
    pub high: String,
    pub low: String,
    pub last: String,
    pub base: String,
    pub tvol: String,
    pub tamt: String,
    pub perx: String,
    pub pbrx: String,
    pub epsx: String,
    pub bpsx: String,
    pub curr: String,
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
    let url =
        "https://openapi.koreainvestment.com:9443/uapi/overseas-price/v1/quotations/price-detail";

    let response: OverseasPriceResponse = call_api(
        oauth,
        header,
        url,
        "HHDFS76200200",
        &[
            ("AUTH", query.auth),
            ("EXCD", query.exchg_code),
            ("SYMB", query.symbol),
        ],
    )
    .await?;

    if response.rt_cd != "0" {
        return Err(format!("API 오류: {} ({})", response.msg1, response.msg_cd).into());
    }
    Ok(response.output)
}

// ========================================================
// 2. 해외주식 상품기본정보
// ========================================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverseasProductInfoQuery<'a> {
    #[serde(rename = "PRDT_TYPE_CD")]
    pub product_type_code: &'a str,
    #[serde(rename = "PDNO")]
    pub product_number: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverseasProductInfoOutput {
    pub std_pdno: String,
    pub prdt_eng_name: String,
    pub natn_cd: String,
    pub natn_name: String,
    pub tr_mket_cd: String,
    pub tr_mket_name: String,
    pub ovrs_excg_cd: String,
    pub ovrs_excg_name: String,
    pub tr_crcy_cd: String,
    pub ovrs_papr: String,
    pub crcy_name: String,
    pub ovrs_stck_dvsn_cd: String,
    pub prdt_clsf_cd: String,
    pub prdt_clsf_name: String,
    pub sll_unit_qty: String,
    pub buy_unit_qty: String,
    pub tr_unit_amt: String,
    pub lstg_stck_num: String,
    pub lstg_dt: String,
    pub ovrs_stck_tr_stop_dvsn_cd: String,
    pub lstg_abol_item_yn: String,
    pub ovrs_stck_prdt_grp_no: String,
    pub lstg_yn: String,
    pub tax_levy_yn: String,
    pub ovrs_stck_erlm_rosn_cd: String,
    pub ovrs_stck_hist_rght_dvsn_cd: String,
    pub chng_bf_pdno: String,
    pub prdt_type_cd_2: String,
    pub ovrs_item_name: String,
    pub sedol_no: String,
    pub blbg_tckr_text: String,
    pub ovrs_stck_etf_risk_drtp_cd: String,
    pub etp_chas_erng_rt_dbnb: String,
    pub istt_usge_isin_cd: String,
    pub mint_svc_yn: String,
    pub mint_svc_yn_chng_dt: String,
    pub prdt_name: String,
    pub lei_cd: String,
    pub ovrs_stck_stop_rson_cd: String,
    pub lstg_abol_dt: String,
    pub mini_stk_tr_stat_dvsn_cd: String,
    pub mint_frst_svc_erlm_dt: String,
    pub mint_dcpt_trad_psbl_yn: String,
    pub mint_fnum_trad_psbl_yn: String,
    pub mint_cblc_cvsn_ipsb_yn: String,
    pub ptp_item_yn: String,
    pub ptp_item_trfx_exmt_yn: String,
    pub ptp_item_trfx_exmt_strt_dt: String,
    pub ptp_item_trfx_exmt_end_dt: String,
    pub dtm_tr_psbl_yn: String,
    pub sdrf_stop_ecls_yn: String,
    pub sdrf_stop_ecls_erlm_dt: String,
    pub memo_text1: String,
    pub ovrs_now_pric1: String,
    pub last_rcvg_dtime: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverseasProductInfoResponse {
    pub rt_cd: String,
    pub msg_cd: String,
    pub msg1: String,
    pub output: OverseasProductInfoOutput,
}

pub async fn get_overseas_product_info(
    oauth: &Oauth,
    header: &ApiHeader<'_>,
    query: OverseasProductInfoQuery<'_>,
) -> Result<OverseasProductInfoOutput, Box<dyn Error>> {
    let url =
        "https://openapi.koreainvestment.com:9443/uapi/overseas-price/v1/quotations/search-info";

    let response: OverseasProductInfoResponse = call_api(
        oauth,
        header,
        url,
        "CTPF1702R",
        &[
            ("PRDT_TYPE_CD", query.product_type_code),
            ("PDNO", query.product_number),
        ],
    )
    .await?;

    if response.rt_cd != "0" {
        return Err(format!("API 오류: {} ({})", response.msg1, response.msg_cd).into());
    }
    Ok(response.output)
}

// ========================================================
// 3. 해외주식 기간별 시세 (일/주/월)
// ========================================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverseasPeriodQuery<'a> {
    #[serde(rename = "AUTH")]
    pub auth: &'a str,
    #[serde(rename = "EXCD")]
    pub exchg_code: &'a str,
    #[serde(rename = "SYMB")]
    pub symbol: &'a str,
    #[serde(rename = "GUBN")]
    pub gubn: &'a str, // 0:일, 1:주, 2:월
    #[serde(rename = "BYMD")]
    pub bymd: &'a str, // 조회 시작일 (yyyymmdd)
    #[serde(rename = "MODP")]
    pub modp: &'a str, // 수정주가 반영 여부 (0,1)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverseasPeriodOutput {
    pub xymd: String,
    pub open: String,
    pub high: String,
    pub low: String,
    pub last: String,
    pub tvol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverseasPeriodResponse {
    pub rt_cd: String,
    pub msg_cd: String,
    pub msg1: String,
    pub output: Vec<OverseasPeriodOutput>,
}

pub async fn get_overseas_period_price(
    oauth: &Oauth,
    header: &ApiHeader<'_>,
    q: OverseasPeriodQuery<'_>,
) -> Result<Vec<OverseasPeriodOutput>, Box<dyn Error>> {
    let url =
        "https://openapi.koreainvestment.com:9443/uapi/overseas-price/v1/quotations/dailyprice";

    let resp: OverseasPeriodResponse = call_api(
        oauth,
        header,
        url,
        "HHDFS76240000",
        &[
            ("AUTH", q.auth),
            ("EXCD", q.exchg_code),
            ("SYMB", q.symbol),
            ("GUBN", q.gubn),
            ("BYMD", q.bymd),
            ("MODP", q.modp),
        ],
    )
    .await?;

    if resp.rt_cd != "0" {
        return Err(format!("API 오류: {} ({})", resp.msg1, resp.msg_cd).into());
    }
    Ok(resp.output)
}

// ========================================================
// 4. 해외주식 당일분봉 조회
// ========================================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverseasTodayMinuteQuery<'a> {
    #[serde(rename = "AUTH")]
    pub auth: &'a str,
    #[serde(rename = "EXCD")]
    pub exchg_code: &'a str,
    #[serde(rename = "SYMB")]
    pub symbol: &'a str,
    #[serde(rename = "NMIN")]
    pub nmin: &'a str, // "1","5","10"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverseasTodayMinute {
    pub xymd: String,
    pub xtime: String,
    pub open: String,
    pub high: String,
    pub low: String,
    pub last: String,
    pub tvol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverseasTodayMinuteResponse {
    pub rt_cd: String,
    pub msg_cd: String,
    pub msg1: String,
    pub output: Vec<OverseasTodayMinute>,
}

pub async fn get_overseas_today_minutes(
    oauth: &Oauth,
    header: &ApiHeader<'_>,
    q: OverseasTodayMinuteQuery<'_>,
) -> Result<Vec<OverseasTodayMinute>, Box<dyn Error>> {
    let url = "https://openapi.koreainvestment.com:9443/uapi/overseas-price/v1/quotations/inquire-timechartprice";

    let resp: OverseasTodayMinuteResponse = call_api(
        oauth,
        header,
        url,
        "HHDFS76200300",
        &[
            ("AUTH", q.auth),
            ("EXCD", q.exchg_code),
            ("SYMB", q.symbol),
            ("NMIN", q.nmin),
        ],
    )
    .await?;

    if resp.rt_cd != "0" {
        return Err(format!("API 오류: {} ({})", resp.msg1, resp.msg_cd).into());
    }
    Ok(resp.output)
}

// ========================================================
// 5. 해외주식 특정일 분봉 조회
// ========================================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverseasByDayMinuteQuery<'a> {
    #[serde(rename = "AUTH")]
    pub auth: &'a str,
    #[serde(rename = "EXCD")]
    pub exchg_code: &'a str,
    #[serde(rename = "SYMB")]
    pub symbol: &'a str,
    #[serde(rename = "BYMD")]
    pub bymd: &'a str, // 조회일자 (yyyymmdd)
    #[serde(rename = "NMIN")]
    pub nmin: &'a str, // 분봉 간격
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverseasByDayMinute {
    pub xymd: String,
    pub xtime: String,
    pub open: String,
    pub high: String,
    pub low: String,
    pub last: String,
    pub tvol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverseasByDayMinuteResponse {
    pub rt_cd: String,
    pub msg_cd: String,
    pub msg1: String,
    pub output: Vec<OverseasByDayMinute>,
}

pub async fn get_overseas_minutes_by_day(
    oauth: &Oauth,
    header: &ApiHeader<'_>,
    q: OverseasByDayMinuteQuery<'_>,
) -> Result<Vec<OverseasByDayMinute>, Box<dyn Error>> {
    let url = "https://openapi.koreainvestment.com:9443/uapi/overseas-price/v1/quotations/inquire-time-dailychartprice";

    let resp: OverseasByDayMinuteResponse = call_api(
        oauth,
        header,
        url,
        "HHDFS76200330",
        &[
            ("AUTH", q.auth),
            ("EXCD", q.exchg_code),
            ("SYMB", q.symbol),
            ("BYMD", q.bymd),
            ("NMIN", q.nmin),
        ],
    )
    .await?;

    if resp.rt_cd != "0" {
        return Err(format!("API 오류: {} ({})", resp.msg1, resp.msg_cd).into());
    }
    Ok(resp.output)
}
