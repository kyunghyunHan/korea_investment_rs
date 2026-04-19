use crate::oauth::Oauth;
use crate::provider::KISProvider;
use crate::types::{AccountInfo, ContinuationKey};
use crate::utils::{
    ApiEndpoint, ApiHeader, ApiResponse, RawApiBody, TrId, call_api, call_get_api, call_post_api,
    create_hashkey,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::error::Error;


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
// 3. 해외주식 종목/지수/환율기간별시세 (일/주/월/년)
// ========================================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverseasDailyChartQuery<'a> {
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub market_div_code: &'a str, // N: 해외지수, X: 환율, I: 국채, S: 금선물
    #[serde(rename = "FID_INPUT_ISCD")]
    pub symbol: &'a str, // 종목코드
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub start_date: &'a str, // 시작일자 (YYYYMMDD)
    #[serde(rename = "FID_INPUT_DATE_2")]
    pub end_date: &'a str, // 종료일자 (YYYYMMDD)
    #[serde(rename = "FID_PERIOD_DIV_CODE")]
    pub period_div_code: &'a str, // D:일, W:주, M:월, Y:년
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverseasDailyChartOutput1 {
    pub ovrs_nmix_prdy_vrss: String,
    pub prdy_vrss_sign: String,
    pub prdy_ctrt: String,
    pub ovrs_nmix_prdy_clpr: String,
    pub acml_vol: String,
    pub hts_kor_isnm: String,
    pub ovrs_nmix_prpr: String,
    pub stck_shrn_iscd: String,
    pub prdy_vol: String,
    pub ovrs_prod_oprc: String,
    pub ovrs_prod_hgpr: String,
    pub ovrs_prod_lwpr: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverseasDailyChartOutput2 {
    pub stck_bsop_date: String,
    pub ovrs_nmix_prpr: String,
    pub ovrs_nmix_oprc: String,
    pub ovrs_nmix_hgpr: String,
    pub ovrs_nmix_lwpr: String,
    pub acml_vol: String,
    pub mod_yn: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverseasDailyChartResponse {
    pub rt_cd: String,
    pub msg_cd: String,
    pub msg1: String,
    pub output1: OverseasDailyChartOutput1,
    pub output2: Vec<OverseasDailyChartOutput2>,
}

pub async fn get_overseas_daily_chartprice(
    oauth: &Oauth,
    header: &ApiHeader<'_>,
    q: OverseasDailyChartQuery<'_>,
) -> Result<(OverseasDailyChartOutput1, Vec<OverseasDailyChartOutput2>), Box<dyn Error>> {
    let url =
        "https://openapi.koreainvestment.com:9443/uapi/overseas-price/v1/quotations/inquire-daily-chartprice";

    let resp: OverseasDailyChartResponse = call_api(
        oauth,
        header,
        url,
        "FHKST03030100",
        &[
            ("FID_COND_MRKT_DIV_CODE", q.market_div_code),
            ("FID_INPUT_ISCD", q.symbol),
            ("FID_INPUT_DATE_1", q.start_date),
            ("FID_INPUT_DATE_2", q.end_date),
            ("FID_PERIOD_DIV_CODE", q.period_div_code),
        ],
    )
    .await?;

    if resp.rt_cd != "0" {
        return Err(format!("API 오류: {} ({})", resp.msg1, resp.msg_cd).into());
    }
    Ok((resp.output1, resp.output2))
}

// ========================================================
// 4. 해외주식 기간별 시세 (일/주/월)
// ========================================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverseasDailyPriceQuery<'a> {
    #[serde(rename = "AUTH")]
    pub auth: &'a str, // "" (Null)
    #[serde(rename = "EXCD")]
    pub exchg_code: &'a str, // NAS, NYS, AMS, HKS ...
    #[serde(rename = "SYMB")]
    pub symbol: &'a str, // 종목코드 (ex. TSLA)
    #[serde(rename = "GUBN")]
    pub gubn: &'a str, // 0:일, 1:주, 2:월
    #[serde(rename = "BYMD")]
    pub bymd: &'a str, // 조회기준일자 (YYYYMMDD)
    #[serde(rename = "MODP")]
    pub modp: &'a str, // 0: 미반영, 1: 반영
    #[serde(rename = "KEYB")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyb: Option<&'a str>, // NEXT KEY BUFF
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverseasDailyPriceOutput1 {
    pub rsym: String,
    pub zdiv: String,
    pub nrec: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverseasDailyPriceOutput2 {
    pub xymd: String,
    pub clos: String,
    pub sign: String,
    pub diff: String,
    pub rate: String,
    pub open: String,
    pub high: String,
    pub low: String,
    pub tvol: String,
    pub tamt: String,
    pub pbid: String,
    pub vbid: String,
    pub pask: String,
    pub vask: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverseasDailyPriceResponse {
    pub rt_cd: String,
    pub msg_cd: String,
    pub msg1: String,
    pub output1: OverseasDailyPriceOutput1,
    pub output2: Vec<OverseasDailyPriceOutput2>,
}

pub async fn get_overseas_daily_price(
    oauth: &Oauth,
    header: &ApiHeader<'_>,
    q: OverseasDailyPriceQuery<'_>,
) -> Result<(OverseasDailyPriceOutput1, Vec<OverseasDailyPriceOutput2>), Box<dyn Error>> {
    let url =
        "https://openapi.koreainvestment.com:9443/uapi/overseas-price/v1/quotations/dailyprice";

    let mut params = vec![
        ("AUTH", q.auth),
        ("EXCD", q.exchg_code),
        ("SYMB", q.symbol),
        ("GUBN", q.gubn),
        ("BYMD", q.bymd),
        ("MODP", q.modp),
    ];
    if let Some(keyb) = q.keyb {
        params.push(("KEYB", keyb));
    }

    let resp: OverseasDailyPriceResponse = call_api(
        oauth,
        header,
        url,
        "HHDFS76240000",
        &params,
    )
    .await?;

    if resp.rt_cd != "0" {
        return Err(format!("API 오류: {} ({})", resp.msg1, resp.msg_cd).into());
    }
    Ok((resp.output1, resp.output2))
}

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
// 5. 해외주식 당일분봉 조회
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
// 6. 해외주식 특정일 분봉 조회
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

// ========================================================
// 7. 해외지수 분봉 조회
// ========================================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverseasIndexMinuteQuery<'a> {
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub market_div_code: &'a str, // N: 해외지수, X: 환율, KX: 원화환율
    #[serde(rename = "FID_INPUT_ISCD")]
    pub symbol: &'a str, // 종목번호 (ex. TSLA)
    #[serde(rename = "FID_HOUR_CLS_CODE")]
    pub hour_cls_code: &'a str, // 0: 정규장, 1: 시간외
    #[serde(rename = "FID_PW_DATA_INCU_YN")]
    pub include_past: &'a str, // Y/N
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverseasIndexMinuteOutput1 {
    pub ovrs_nmix_prdy_vrss: String,
    pub prdy_vrss_sign: String,
    pub hts_kor_isnm: String,
    pub prdy_ctrt: String,
    pub ovrs_nmix_prdy_clpr: String,
    pub acml_vol: String,
    pub ovrs_nmix_prpr: String,
    pub stck_shrn_iscd: String,
    pub ovrs_prod_oprc: String,
    pub ovrs_prod_hgpr: String,
    pub ovrs_prod_lwpr: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverseasIndexMinuteOutput2 {
    pub stck_bsop_date: String,
    pub stck_cntg_hour: String,
    pub optn_prpr: String,
    pub optn_oprc: String,
    pub optn_hgpr: String,
    pub optn_lwpr: String,
    pub cntg_vol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverseasIndexMinuteResponse {
    pub rt_cd: String,
    pub msg_cd: String,
    pub msg1: String,
    pub output1: OverseasIndexMinuteOutput1,
    pub output2: Vec<OverseasIndexMinuteOutput2>,
}

pub async fn get_overseas_index_minutes(
    oauth: &Oauth,
    header: &ApiHeader<'_>,
    q: OverseasIndexMinuteQuery<'_>,
) -> Result<(OverseasIndexMinuteOutput1, Vec<OverseasIndexMinuteOutput2>), Box<dyn Error>> {
    let url =
        "https://openapi.koreainvestment.com:9443/uapi/overseas-price/v1/quotations/inquire-time-indexchartprice";

    let resp: OverseasIndexMinuteResponse = call_api(
        oauth,
        header,
        url,
        "FHKST03030200",
        &[
            ("FID_COND_MRKT_DIV_CODE", q.market_div_code),
            ("FID_INPUT_ISCD", q.symbol),
            ("FID_HOUR_CLS_CODE", q.hour_cls_code),
            ("FID_PW_DATA_INCU_YN", q.include_past),
        ],
    )
    .await?;

    if resp.rt_cd != "0" {
        return Err(format!("API 오류: {} ({})", resp.msg1, resp.msg_cd).into());
    }
    Ok((resp.output1, resp.output2))
}

const OVERSEAS_ORDER_BUY_ENDPOINT: ApiEndpoint = ApiEndpoint::new(
    "/uapi/overseas-stock/v1/trading/order",
    TrId::new("TTTT1002U", Some("VTTT1002U")),
);
const OVERSEAS_ORDER_SELL_ENDPOINT: ApiEndpoint = ApiEndpoint::new(
    "/uapi/overseas-stock/v1/trading/order",
    TrId::new("TTTT1006U", Some("VTTT1001U")),
);
const OVERSEAS_ORDER_REVISE_CANCEL_ENDPOINT: ApiEndpoint = ApiEndpoint::new(
    "/uapi/overseas-stock/v1/trading/order-rvsecncl",
    TrId::new("TTTT1004U", Some("VTTT1004U")),
);
const OVERSEAS_BALANCE_ENDPOINT: ApiEndpoint = ApiEndpoint::new(
    "/uapi/overseas-stock/v1/trading/inquire-balance",
    TrId::new("TTTS3012R", Some("VTTS3012R")),
);
const OVERSEAS_CCLD_ENDPOINT: ApiEndpoint = ApiEndpoint::new(
    "/uapi/overseas-stock/v1/trading/inquire-ccnl",
    TrId::new("TTTS3035R", Some("VTTS3035R")),
);
const OVERSEAS_PSAMOUNT_ENDPOINT: ApiEndpoint = ApiEndpoint::new(
    "/uapi/overseas-stock/v1/trading/inquire-psamount",
    TrId::new("TTTS3007R", Some("VTTS3007R")),
);
const OVERSEAS_PRESENT_BALANCE_ENDPOINT: ApiEndpoint = ApiEndpoint::new(
    "/uapi/overseas-stock/v1/trading/inquire-present-balance",
    TrId::new("CTRP6504R", Some("VTRP6504R")),
);
const OVERSEAS_PAYMENT_BALANCE_ENDPOINT: ApiEndpoint = ApiEndpoint::real_only(
    "/uapi/overseas-stock/v1/trading/inquire-paymt-stdr-balance",
    TrId::new("CTRP6010R", None),
);
const OVERSEAS_PERIOD_PROFIT_ENDPOINT: ApiEndpoint = ApiEndpoint::real_only(
    "/uapi/overseas-stock/v1/trading/inquire-period-profit",
    TrId::new("TTTS3039R", None),
);
const OVERSEAS_NCCS_ENDPOINT: ApiEndpoint = ApiEndpoint::real_only(
    "/uapi/overseas-stock/v1/trading/inquire-nccs",
    TrId::new("TTTS3018R", None),
);
const OVERSEAS_PERIOD_TRANS_ENDPOINT: ApiEndpoint = ApiEndpoint::real_only(
    "/uapi/overseas-stock/v1/trading/inquire-period-trans",
    TrId::new("CTOS4001R", None),
);
const OVERSEAS_DAYTIME_BUY_ENDPOINT: ApiEndpoint = ApiEndpoint::real_only(
    "/uapi/overseas-stock/v1/trading/daytime-order",
    TrId::new("TTTS6036U", None),
);
const OVERSEAS_DAYTIME_SELL_ENDPOINT: ApiEndpoint = ApiEndpoint::real_only(
    "/uapi/overseas-stock/v1/trading/daytime-order",
    TrId::new("TTTS6037U", None),
);
const OVERSEAS_DAYTIME_REVISE_CANCEL_ENDPOINT: ApiEndpoint = ApiEndpoint::real_only(
    "/uapi/overseas-stock/v1/trading/daytime-order-rvsecncl",
    TrId::new("TTTS6038U", None),
);
const OVERSEAS_RESERVE_ORDER_ASIA_ENDPOINT: ApiEndpoint = ApiEndpoint::new(
    "/uapi/overseas-stock/v1/trading/order-resv",
    TrId::new("TTTS3013U", Some("VTTS3013U")),
);
const OVERSEAS_RESERVE_ORDER_USA_BUY_ENDPOINT: ApiEndpoint = ApiEndpoint::new(
    "/uapi/overseas-stock/v1/trading/order-resv",
    TrId::new("TTTT3014U", Some("VTTT3014U")),
);
const OVERSEAS_RESERVE_ORDER_USA_SELL_ENDPOINT: ApiEndpoint = ApiEndpoint::new(
    "/uapi/overseas-stock/v1/trading/order-resv",
    TrId::new("TTTT3016U", Some("VTTT3016U")),
);
const OVERSEAS_RESERVE_ORDER_LIST_USA_ENDPOINT: ApiEndpoint = ApiEndpoint::real_only(
    "/uapi/overseas-stock/v1/trading/order-resv-list",
    TrId::new("TTTT3039R", None),
);
const OVERSEAS_RESERVE_ORDER_LIST_ASIA_ENDPOINT: ApiEndpoint = ApiEndpoint::real_only(
    "/uapi/overseas-stock/v1/trading/order-resv-list",
    TrId::new("TTTS3014R", None),
);
const OVERSEAS_RESERVE_ORDER_CANCEL_USA_ENDPOINT: ApiEndpoint = ApiEndpoint::new(
    "/uapi/overseas-stock/v1/trading/order-resv-ccnl",
    TrId::new("TTTT3017U", Some("VTTT3017U")),
);
const OVERSEAS_ASKING_PRICE_ENDPOINT: ApiEndpoint = ApiEndpoint::real_only(
    "/uapi/overseas-price/v1/quotations/inquire-asking-price",
    TrId::new("HHDFS76200100", None),
);
const OVERSEAS_MULTI_PRICE_ENDPOINT: ApiEndpoint = ApiEndpoint::real_only(
    "/uapi/overseas-price/v1/quotations/multprice",
    TrId::new("HHDFS76220000", None),
);

#[derive(Debug, Clone, Serialize)]
pub struct OverseasOrderRequest<'a> {
    #[serde(rename = "CANO")]
    pub cano: &'a str,
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: &'a str,
    #[serde(rename = "OVRS_EXCG_CD")]
    pub ovrs_excg_cd: &'a str,
    #[serde(rename = "PDNO")]
    pub pdno: &'a str,
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: &'a str,
    #[serde(rename = "OVRS_ORD_UNPR")]
    pub ovrs_ord_unpr: &'a str,
    #[serde(rename = "CTAC_TLNO")]
    pub ctac_tlno: &'a str,
    #[serde(rename = "MGCO_APTM_ODNO")]
    pub mgco_aptm_odno: &'a str,
    #[serde(rename = "SLL_TYPE")]
    pub sll_type: &'a str,
    #[serde(rename = "ORD_SVR_DVSN_CD")]
    pub ord_svr_dvsn_cd: &'a str,
    #[serde(rename = "ORD_DVSN")]
    pub ord_dvsn: &'a str,
    #[serde(rename = "START_TIME", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<&'a str>,
    #[serde(rename = "END_TIME", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<&'a str>,
    #[serde(rename = "ALGO_ORD_TMD_DVSN_CD", skip_serializing_if = "Option::is_none")]
    pub algo_ord_tmd_dvsn_cd: Option<&'a str>,
}

#[derive(Debug, Clone, Serialize)]
pub struct OverseasRevisionRequest<'a> {
    #[serde(rename = "CANO")]
    pub cano: &'a str,
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: &'a str,
    #[serde(rename = "OVRS_EXCG_CD")]
    pub ovrs_excg_cd: &'a str,
    #[serde(rename = "PDNO")]
    pub pdno: &'a str,
    #[serde(rename = "ORGN_ODNO")]
    pub orgn_odno: &'a str,
    #[serde(rename = "RVSE_CNCL_DVSN_CD")]
    pub rvse_cncl_dvsn_cd: &'a str,
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: &'a str,
    #[serde(rename = "OVRS_ORD_UNPR")]
    pub ovrs_ord_unpr: &'a str,
    #[serde(rename = "MGCO_APTM_ODNO")]
    pub mgco_aptm_odno: &'a str,
    #[serde(rename = "ORD_SVR_DVSN_CD")]
    pub ord_svr_dvsn_cd: &'a str,
}

#[derive(Debug, Clone)]
pub struct OverseasBalanceRequest<'a> {
    pub account: &'a AccountInfo,
    pub ovrs_excg_cd: &'a str,
    pub tr_crcy_cd: &'a str,
    pub continuation: Option<&'a ContinuationKey>,
}

#[derive(Debug, Clone)]
pub struct OverseasCcldRequest<'a> {
    pub account: &'a AccountInfo,
    pub ovrs_excg_cd: &'a str,
    pub sort_sqn: &'a str,
    pub ctx_area_fk200: &'a str,
    pub ctx_area_nk200: &'a str,
}

#[derive(Debug, Clone)]
pub struct OverseasPossibleAmountRequest<'a> {
    pub account: &'a AccountInfo,
    pub ovrs_excg_cd: &'a str,
    pub ovrs_ord_unpr: &'a str,
    pub item_cd: &'a str,
}

#[derive(Debug, Clone)]
pub struct OverseasPresentBalanceRequest<'a> {
    pub account: &'a AccountInfo,
    pub wcrc_frcr_dvsn_cd: &'a str,
    pub natn_cd: &'a str,
    pub tr_mket_cd: &'a str,
    pub inqr_dvsn_cd: &'a str,
}

#[derive(Debug, Clone)]
pub struct OverseasPaymentBalanceRequest<'a> {
    pub account: &'a AccountInfo,
    pub bass_dt: &'a str,
    pub wcrc_frcr_dvsn_cd: &'a str,
    pub inqr_dvsn_cd: &'a str,
}

#[derive(Debug, Clone)]
pub struct OverseasPeriodProfitRequest<'a> {
    pub account: &'a AccountInfo,
    pub ovrs_excg_cd: &'a str,
    pub natn_cd: &'a str,
    pub crcy_cd: &'a str,
    pub pdno: &'a str,
    pub inqr_strt_dt: &'a str,
    pub inqr_end_dt: &'a str,
    pub wcrc_frcr_dvsn_cd: &'a str,
    pub continuation: Option<&'a ContinuationKey>,
}

#[derive(Debug, Clone)]
pub struct OverseasNccsRequest<'a> {
    pub account: &'a AccountInfo,
    pub ovrs_excg_cd: &'a str,
    pub sort_sqn: &'a str,
    pub continuation: Option<&'a ContinuationKey>,
}

#[derive(Debug, Clone)]
pub struct OverseasPeriodTransactionRequest<'a> {
    pub account: &'a AccountInfo,
    pub erlm_strt_dt: &'a str,
    pub erlm_end_dt: &'a str,
    pub ovrs_excg_cd: &'a str,
    pub pdno: &'a str,
    pub sll_buy_dvsn_cd: &'a str,
    pub loan_dvsn_cd: &'a str,
    pub continuation: Option<&'a ContinuationKey>,
}

#[derive(Debug, Clone, Serialize)]
pub struct OverseasDaytimeOrderRequest<'a> {
    #[serde(rename = "CANO")]
    pub cano: &'a str,
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: &'a str,
    #[serde(rename = "OVRS_EXCG_CD")]
    pub ovrs_excg_cd: &'a str,
    #[serde(rename = "PDNO")]
    pub pdno: &'a str,
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: &'a str,
    #[serde(rename = "OVRS_ORD_UNPR")]
    pub ovrs_ord_unpr: &'a str,
    #[serde(rename = "CTAC_TLNO")]
    pub ctac_tlno: &'a str,
    #[serde(rename = "MGCO_APTM_ODNO")]
    pub mgco_aptm_odno: &'a str,
    #[serde(rename = "ORD_SVR_DVSN_CD")]
    pub ord_svr_dvsn_cd: &'a str,
    #[serde(rename = "ORD_DVSN")]
    pub ord_dvsn: &'a str,
}

#[derive(Debug, Clone, Serialize)]
pub struct OverseasDaytimeRevisionRequest<'a> {
    #[serde(rename = "CANO")]
    pub cano: &'a str,
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: &'a str,
    #[serde(rename = "OVRS_EXCG_CD")]
    pub ovrs_excg_cd: &'a str,
    #[serde(rename = "PDNO")]
    pub pdno: &'a str,
    #[serde(rename = "ORGN_ODNO")]
    pub orgn_odno: &'a str,
    #[serde(rename = "RVSE_CNCL_DVSN_CD")]
    pub rvse_cncl_dvsn_cd: &'a str,
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: &'a str,
    #[serde(rename = "OVRS_ORD_UNPR")]
    pub ovrs_ord_unpr: &'a str,
    #[serde(rename = "CTAC_TLNO")]
    pub ctac_tlno: &'a str,
    #[serde(rename = "MGCO_APTM_ODNO")]
    pub mgco_aptm_odno: &'a str,
    #[serde(rename = "ORD_SVR_DVSN_CD")]
    pub ord_svr_dvsn_cd: &'a str,
}

#[derive(Debug, Clone, Serialize)]
pub struct OverseasReserveOrderRequest<'a> {
    #[serde(rename = "CANO")]
    pub cano: &'a str,
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: &'a str,
    #[serde(rename = "SLL_BUY_DVSN_CD", skip_serializing_if = "Option::is_none")]
    pub sll_buy_dvsn_cd: Option<&'a str>,
    #[serde(rename = "RVSE_CNCL_DVSN_CD", skip_serializing_if = "Option::is_none")]
    pub rvse_cncl_dvsn_cd: Option<&'a str>,
    #[serde(rename = "PDNO")]
    pub pdno: &'a str,
    #[serde(rename = "PRDT_TYPE_CD", skip_serializing_if = "Option::is_none")]
    pub prdt_type_cd: Option<&'a str>,
    #[serde(rename = "OVRS_EXCG_CD")]
    pub ovrs_excg_cd: &'a str,
    #[serde(rename = "FT_ORD_QTY")]
    pub ft_ord_qty: &'a str,
    #[serde(rename = "FT_ORD_UNPR3")]
    pub ft_ord_unpr3: &'a str,
    #[serde(rename = "ORD_SVR_DVSN_CD", skip_serializing_if = "Option::is_none")]
    pub ord_svr_dvsn_cd: Option<&'a str>,
    #[serde(rename = "RSVN_ORD_RCIT_DT", skip_serializing_if = "Option::is_none")]
    pub rsvn_ord_rcit_dt: Option<&'a str>,
    #[serde(rename = "ORD_DVSN", skip_serializing_if = "Option::is_none")]
    pub ord_dvsn: Option<&'a str>,
    #[serde(rename = "OVRS_RSVN_ODNO", skip_serializing_if = "Option::is_none")]
    pub ovrs_rsvn_odno: Option<&'a str>,
    #[serde(rename = "ALGO_ORD_TMD_DVSN_CD", skip_serializing_if = "Option::is_none")]
    pub algo_ord_tmd_dvsn_cd: Option<&'a str>,
}

#[derive(Debug, Clone)]
pub enum OverseasReserveMarket {
    UsaBuy,
    UsaSell,
    Asia,
}

#[derive(Debug, Clone)]
pub enum OverseasReserveListMarket {
    Usa,
    Asia,
}

#[derive(Debug, Clone)]
pub struct OverseasReserveOrderListRequest<'a> {
    pub account: &'a AccountInfo,
    pub inqr_strt_dt: &'a str,
    pub inqr_end_dt: &'a str,
    pub inqr_dvsn_cd: &'a str,
    pub prdt_type_cd: &'a str,
    pub ovrs_excg_cd: &'a str,
    pub market: OverseasReserveListMarket,
    pub continuation: Option<&'a ContinuationKey>,
}

#[derive(Debug, Clone, Serialize)]
pub struct OverseasReserveOrderCancelRequest<'a> {
    #[serde(rename = "CANO")]
    pub cano: &'a str,
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: &'a str,
    #[serde(rename = "RSYN_ORD_RCIT_DT")]
    pub rsyn_ord_rcit_dt: &'a str,
    #[serde(rename = "OVRS_RSVN_ODNO")]
    pub ovrs_rsvn_odno: &'a str,
}

#[derive(Debug, Clone)]
pub struct OverseasMultiPriceRequest<'a> {
    pub items: Vec<(&'a str, &'a str)>,
    pub continuation: bool,
}

#[async_trait]
pub trait OverseasTrading {
    async fn place_overseas_buy_order(
        &self,
        request: OverseasOrderRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>>;
    async fn place_overseas_sell_order(
        &self,
        request: OverseasOrderRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>>;
    async fn revise_or_cancel_overseas_order(
        &self,
        request: OverseasRevisionRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>>;
    async fn inquire_overseas_balance(
        &self,
        request: OverseasBalanceRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>>;
    async fn inquire_overseas_ccld(
        &self,
        request: OverseasCcldRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>>;
    async fn inquire_overseas_possible_amount(
        &self,
        request: OverseasPossibleAmountRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>>;
    async fn inquire_overseas_present_balance(
        &self,
        request: OverseasPresentBalanceRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>>;
    async fn inquire_overseas_payment_balance(
        &self,
        request: OverseasPaymentBalanceRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>>;
    async fn inquire_overseas_period_profit(
        &self,
        request: OverseasPeriodProfitRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>>;
    async fn inquire_overseas_nccs(
        &self,
        request: OverseasNccsRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>>;
    async fn inquire_overseas_period_transaction(
        &self,
        request: OverseasPeriodTransactionRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>>;
    async fn place_overseas_daytime_buy_order(
        &self,
        request: OverseasDaytimeOrderRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>>;
    async fn place_overseas_daytime_sell_order(
        &self,
        request: OverseasDaytimeOrderRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>>;
    async fn revise_or_cancel_overseas_daytime_order(
        &self,
        request: OverseasDaytimeRevisionRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>>;
    async fn place_overseas_reserve_order(
        &self,
        market: OverseasReserveMarket,
        request: OverseasReserveOrderRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>>;
    async fn inquire_overseas_reserve_orders(
        &self,
        request: OverseasReserveOrderListRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>>;
    async fn cancel_overseas_reserve_order(
        &self,
        request: OverseasReserveOrderCancelRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>>;
    async fn get_overseas_asking_price(
        &self,
        exchange_code: &str,
        symbol: &str,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>>;
    async fn get_overseas_multi_price(
        &self,
        request: OverseasMultiPriceRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>>;
}

async fn post_overseas_with_hashkey<T: Serialize>(
    provider: &KISProvider,
    endpoint: ApiEndpoint,
    body: &T,
) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>> {
    let hashkey = create_hashkey(&provider.oauth, provider.practice, body).await?;
    let header = ApiHeader::personal().with_hashkey(Some(hashkey.as_str()));
    let response = call_post_api::<RawApiBody, _>(
        &provider.oauth,
        &header,
        provider.practice,
        endpoint,
        body,
    )
    .await?;
    response.body.ensure_success()?;
    Ok(response)
}

#[async_trait]
impl OverseasTrading for KISProvider {
    async fn place_overseas_buy_order(
        &self,
        request: OverseasOrderRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>> {
        post_overseas_with_hashkey(self, OVERSEAS_ORDER_BUY_ENDPOINT, &request).await
    }

    async fn place_overseas_sell_order(
        &self,
        request: OverseasOrderRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>> {
        post_overseas_with_hashkey(self, OVERSEAS_ORDER_SELL_ENDPOINT, &request).await
    }

    async fn revise_or_cancel_overseas_order(
        &self,
        request: OverseasRevisionRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>> {
        post_overseas_with_hashkey(self, OVERSEAS_ORDER_REVISE_CANCEL_ENDPOINT, &request).await
    }

    async fn inquire_overseas_balance(
        &self,
        request: OverseasBalanceRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>> {
        let continuation = request.continuation.cloned().unwrap_or_default();
        let header = self
            .header
            .clone()
            .with_tr_cont(request.continuation.map(|_| "N"));
        let response = call_get_api::<RawApiBody>(
            &self.oauth,
            &header,
            self.practice,
            OVERSEAS_BALANCE_ENDPOINT,
            &[
                ("CANO", &request.account.cano),
                ("ACNT_PRDT_CD", &request.account.acnt_prdt_cd),
                ("OVRS_EXCG_CD", request.ovrs_excg_cd),
                ("TR_CRCY_CD", request.tr_crcy_cd),
                ("CTX_AREA_FK200", &continuation.fk),
                ("CTX_AREA_NK200", &continuation.nk),
            ],
        )
        .await?;
        response.body.ensure_success()?;
        Ok(response)
    }

    async fn inquire_overseas_ccld(
        &self,
        request: OverseasCcldRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>> {
        let response = call_get_api::<RawApiBody>(
            &self.oauth,
            &self.header,
            self.practice,
            OVERSEAS_CCLD_ENDPOINT,
            &[
                ("CANO", &request.account.cano),
                ("ACNT_PRDT_CD", &request.account.acnt_prdt_cd),
                ("OVRS_EXCG_CD", request.ovrs_excg_cd),
                ("SORT_SQN", request.sort_sqn),
                ("CTX_AREA_FK200", request.ctx_area_fk200),
                ("CTX_AREA_NK200", request.ctx_area_nk200),
            ],
        )
        .await?;
        response.body.ensure_success()?;
        Ok(response)
    }

    async fn inquire_overseas_possible_amount(
        &self,
        request: OverseasPossibleAmountRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>> {
        let response = call_get_api::<RawApiBody>(
            &self.oauth,
            &self.header,
            self.practice,
            OVERSEAS_PSAMOUNT_ENDPOINT,
            &[
                ("CANO", &request.account.cano),
                ("ACNT_PRDT_CD", &request.account.acnt_prdt_cd),
                ("OVRS_EXCG_CD", request.ovrs_excg_cd),
                ("OVRS_ORD_UNPR", request.ovrs_ord_unpr),
                ("ITEM_CD", request.item_cd),
            ],
        )
        .await?;
        response.body.ensure_success()?;
        Ok(response)
    }

    async fn inquire_overseas_present_balance(
        &self,
        request: OverseasPresentBalanceRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>> {
        let response = call_get_api::<RawApiBody>(
            &self.oauth,
            &self.header,
            self.practice,
            OVERSEAS_PRESENT_BALANCE_ENDPOINT,
            &[
                ("CANO", &request.account.cano),
                ("ACNT_PRDT_CD", &request.account.acnt_prdt_cd),
                ("WCRC_FRCR_DVSN_CD", request.wcrc_frcr_dvsn_cd),
                ("NATN_CD", request.natn_cd),
                ("TR_MKET_CD", request.tr_mket_cd),
                ("INQR_DVSN_CD", request.inqr_dvsn_cd),
            ],
        )
        .await?;
        response.body.ensure_success()?;
        Ok(response)
    }

    async fn inquire_overseas_payment_balance(
        &self,
        request: OverseasPaymentBalanceRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>> {
        let response = call_get_api::<RawApiBody>(
            &self.oauth,
            &self.header,
            self.practice,
            OVERSEAS_PAYMENT_BALANCE_ENDPOINT,
            &[
                ("CANO", &request.account.cano),
                ("ACNT_PRDT_CD", &request.account.acnt_prdt_cd),
                ("BASS_DT", request.bass_dt),
                ("WCRC_FRCR_DVSN_CD", request.wcrc_frcr_dvsn_cd),
                ("INQR_DVSN_CD", request.inqr_dvsn_cd),
            ],
        )
        .await?;
        response.body.ensure_success()?;
        Ok(response)
    }

    async fn inquire_overseas_period_profit(
        &self,
        request: OverseasPeriodProfitRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>> {
        let continuation = request.continuation.cloned().unwrap_or_default();
        let header = self
            .header
            .clone()
            .with_tr_cont(request.continuation.map(|_| "N"));
        let response = call_get_api::<RawApiBody>(
            &self.oauth,
            &header,
            self.practice,
            OVERSEAS_PERIOD_PROFIT_ENDPOINT,
            &[
                ("CANO", &request.account.cano),
                ("ACNT_PRDT_CD", &request.account.acnt_prdt_cd),
                ("OVRS_EXCG_CD", request.ovrs_excg_cd),
                ("NATN_CD", request.natn_cd),
                ("CRCY_CD", request.crcy_cd),
                ("PDNO", request.pdno),
                ("INQR_STRT_DT", request.inqr_strt_dt),
                ("INQR_END_DT", request.inqr_end_dt),
                ("WCRC_FRCR_DVSN_CD", request.wcrc_frcr_dvsn_cd),
                ("CTX_AREA_FK200", &continuation.fk),
                ("CTX_AREA_NK200", &continuation.nk),
            ],
        )
        .await?;
        response.body.ensure_success()?;
        Ok(response)
    }

    async fn inquire_overseas_nccs(
        &self,
        request: OverseasNccsRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>> {
        let continuation = request.continuation.cloned().unwrap_or_default();
        let header = self
            .header
            .clone()
            .with_tr_cont(request.continuation.map(|_| "N"));
        let response = call_get_api::<RawApiBody>(
            &self.oauth,
            &header,
            self.practice,
            OVERSEAS_NCCS_ENDPOINT,
            &[
                ("CANO", &request.account.cano),
                ("ACNT_PRDT_CD", &request.account.acnt_prdt_cd),
                ("OVRS_EXCG_CD", request.ovrs_excg_cd),
                ("SORT_SQN", request.sort_sqn),
                ("CTX_AREA_FK200", &continuation.fk),
                ("CTX_AREA_NK200", &continuation.nk),
            ],
        )
        .await?;
        response.body.ensure_success()?;
        Ok(response)
    }

    async fn inquire_overseas_period_transaction(
        &self,
        request: OverseasPeriodTransactionRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>> {
        let continuation = request.continuation.cloned().unwrap_or_default();
        let header = self
            .header
            .clone()
            .with_tr_cont(request.continuation.map(|_| "N"));
        let response = call_get_api::<RawApiBody>(
            &self.oauth,
            &header,
            self.practice,
            OVERSEAS_PERIOD_TRANS_ENDPOINT,
            &[
                ("CANO", &request.account.cano),
                ("ACNT_PRDT_CD", &request.account.acnt_prdt_cd),
                ("ERLM_STRT_DT", request.erlm_strt_dt),
                ("ERLM_END_DT", request.erlm_end_dt),
                ("OVRS_EXCG_CD", request.ovrs_excg_cd),
                ("PDNO", request.pdno),
                ("SLL_BUY_DVSN_CD", request.sll_buy_dvsn_cd),
                ("LOAN_DVSN_CD", request.loan_dvsn_cd),
                ("CTX_AREA_FK100", &continuation.fk),
                ("CTX_AREA_NK100", &continuation.nk),
            ],
        )
        .await?;
        response.body.ensure_success()?;
        Ok(response)
    }

    async fn place_overseas_daytime_buy_order(
        &self,
        request: OverseasDaytimeOrderRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>> {
        post_overseas_with_hashkey(self, OVERSEAS_DAYTIME_BUY_ENDPOINT, &request).await
    }

    async fn place_overseas_daytime_sell_order(
        &self,
        request: OverseasDaytimeOrderRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>> {
        post_overseas_with_hashkey(self, OVERSEAS_DAYTIME_SELL_ENDPOINT, &request).await
    }

    async fn revise_or_cancel_overseas_daytime_order(
        &self,
        request: OverseasDaytimeRevisionRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>> {
        post_overseas_with_hashkey(self, OVERSEAS_DAYTIME_REVISE_CANCEL_ENDPOINT, &request).await
    }

    async fn place_overseas_reserve_order(
        &self,
        market: OverseasReserveMarket,
        request: OverseasReserveOrderRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>> {
        let endpoint = match market {
            OverseasReserveMarket::UsaBuy => OVERSEAS_RESERVE_ORDER_USA_BUY_ENDPOINT,
            OverseasReserveMarket::UsaSell => OVERSEAS_RESERVE_ORDER_USA_SELL_ENDPOINT,
            OverseasReserveMarket::Asia => OVERSEAS_RESERVE_ORDER_ASIA_ENDPOINT,
        };
        post_overseas_with_hashkey(self, endpoint, &request).await
    }

    async fn inquire_overseas_reserve_orders(
        &self,
        request: OverseasReserveOrderListRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>> {
        let continuation = request.continuation.cloned().unwrap_or_default();
        let header = self
            .header
            .clone()
            .with_tr_cont(request.continuation.map(|_| "N"));
        let endpoint = match request.market {
            OverseasReserveListMarket::Usa => OVERSEAS_RESERVE_ORDER_LIST_USA_ENDPOINT,
            OverseasReserveListMarket::Asia => OVERSEAS_RESERVE_ORDER_LIST_ASIA_ENDPOINT,
        };
        let response = call_get_api::<RawApiBody>(
            &self.oauth,
            &header,
            self.practice,
            endpoint,
            &[
                ("CANO", &request.account.cano),
                ("ACNT_PRDT_CD", &request.account.acnt_prdt_cd),
                ("INQR_STRT_DT", request.inqr_strt_dt),
                ("INQR_END_DT", request.inqr_end_dt),
                ("INQR_DVSN_CD", request.inqr_dvsn_cd),
                ("PRDT_TYPE_CD", request.prdt_type_cd),
                ("OVRS_EXCG_CD", request.ovrs_excg_cd),
                ("CTX_AREA_FK200", &continuation.fk),
                ("CTX_AREA_NK200", &continuation.nk),
            ],
        )
        .await?;
        response.body.ensure_success()?;
        Ok(response)
    }

    async fn cancel_overseas_reserve_order(
        &self,
        request: OverseasReserveOrderCancelRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>> {
        post_overseas_with_hashkey(self, OVERSEAS_RESERVE_ORDER_CANCEL_USA_ENDPOINT, &request).await
    }

    async fn get_overseas_asking_price(
        &self,
        exchange_code: &str,
        symbol: &str,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>> {
        let response = call_get_api::<RawApiBody>(
            &self.oauth,
            &self.header,
            self.practice,
            OVERSEAS_ASKING_PRICE_ENDPOINT,
            &[("AUTH", ""), ("EXCD", exchange_code), ("SYMB", symbol)],
        )
        .await?;
        response.body.ensure_success()?;
        Ok(response)
    }

    async fn get_overseas_multi_price(
        &self,
        request: OverseasMultiPriceRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>> {
        let nrec = request.items.len().min(10).to_string();
        let mut pairs: Vec<(String, String)> = vec![
            ("AUTH".to_string(), "".to_string()),
            ("NREC".to_string(), nrec),
        ];
        for (index, (exchange, symbol)) in request.items.iter().take(10).enumerate() {
            let slot = index + 1;
            pairs.push((format!("EXCD_{slot:02}"), (*exchange).to_string()));
            pairs.push((format!("SYMB_{slot:02}"), (*symbol).to_string()));
        }
        let refs: Vec<(&str, &str)> = pairs
            .iter()
            .map(|(key, value)| (key.as_str(), value.as_str()))
            .collect();
        let header = self
            .header
            .clone()
            .with_tr_cont(request.continuation.then_some("N"));
        let response = call_get_api::<RawApiBody>(
            &self.oauth,
            &header,
            self.practice,
            OVERSEAS_MULTI_PRICE_ENDPOINT,
            &refs,
        )
        .await?;
        response.body.ensure_success()?;
        Ok(response)
    }
}
