use crate::oauth::Oauth;
use crate::provider::KISProvider;
use crate::types::CustType;
use crate::utils::{ApiHeader, call_api};
use serde::{Deserialize, Serialize};
use std::error::Error;
use async_trait::async_trait;
// ======================================================
// Domestic Trait 정의
// ======================================================
#[async_trait]
pub trait Domestic {
    async fn get_inquire_price(&self, stock_code: &str)
    -> Result<StockPriceOutput, Box<dyn Error>>;
    async fn get_inquire_price2(
        &self,
        stock_code: &str,
    ) -> Result<StockPrice2Output, Box<dyn Error>>;
    async fn get_inquire_period_price(
        &self,
        stock_code: &str,
        from: &str,
        to: &str,
        period: &str,
    ) -> Result<PeriodPriceResponse, Box<dyn Error>>;
    async fn get_recent_ticks(&self, stock_code: &str) -> Result<Vec<Tick>, Box<dyn Error>>;
    async fn get_today_minutes(
        &self,
        stock_code: &str,
        interval: &str,
    ) -> Result<Vec<TodayMinuteCandle>, Box<dyn Error>>;
    async fn get_minutes_by_day(
        &self,
        stock_code: &str,
        date: &str,
        interval: &str,
    ) -> Result<Vec<ByDayMinuteCandle>, Box<dyn Error>>;
}

// ======================================================
// 공통 Query & Response Struct
// ======================================================

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

// 현재가 응답
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

// 시세2 응답
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockPrice2Output {
    pub stck_prpr: String,
    pub prdy_vrss: String,
    pub prdy_ctrt: String,
    pub acml_tr_pbmn: String,
    pub acml_vol: String,
    pub stck_oprc: String,
    pub stck_hgpr: String,
    pub stck_lwpr: String,
    pub stck_llam: String,
    pub stck_mxpr: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockPrice2Response {
    pub rt_cd: String,
    pub msg_cd: String,
    pub msg1: String,
    pub output: StockPrice2Output,
}

// 기간별 조회
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeriodPriceQuery<'a> {
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: &'a str,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeriodPriceOutput1 {
    pub stck_prpr: String,
    pub prdy_vrss: String,
    pub prdy_ctrt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeriodPriceOutput2 {
    pub stck_bsop_date: String,
    pub stck_clpr: String,
    pub stck_oprc: String,
    pub stck_hgpr: String,
    pub stck_lwpr: String,
    pub acml_vol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeriodPriceResponse {
    pub rt_cd: String,
    pub msg_cd: String,
    pub msg1: String,
    pub output1: PeriodPriceOutput1,
    pub output2: Vec<PeriodPriceOutput2>,
}

// 틱 체결
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tick {
    pub stck_cntg_hour: String,
    pub stck_prpr: String,
    pub cntg_vol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicksResponse {
    pub rt_cd: String,
    pub msg_cd: String,
    pub msg1: String,
    pub output: Vec<Tick>,
}

// 당일 분봉
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodayMinuteCandle {
    pub stck_cntg_hour: String,
    pub stck_prpr: String,
    pub stck_oprc: String,
    pub stck_hgpr: String,
    pub stck_lwpr: String,
    pub acml_vol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodayMinuteResponse {
    pub rt_cd: String,
    pub msg_cd: String,
    pub msg1: String,
    pub output: Vec<TodayMinuteCandle>,
}

// 특정일 분봉
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ByDayMinuteCandle {
    pub stck_cntg_hour: String,
    pub stck_prpr: String,
    pub stck_oprc: String,
    pub stck_hgpr: String,
    pub stck_lwpr: String,
    pub acml_vol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ByDayMinutesResponse {
    pub rt_cd: String,
    pub msg_cd: String,
    pub msg1: String,
    pub output: Vec<ByDayMinuteCandle>,
}

// ======================================================
// Domestic 구현체
// ======================================================
#[async_trait]

impl Domestic for KISProvider {
    async fn get_inquire_price(
        &self,
        stock_code: &str,
    ) -> Result<StockPriceOutput, Box<dyn Error>> {
        let query = QueryParam::stock(stock_code);
        let url = "https://openapi.koreainvestment.com:9443/uapi/domestic-stock/v1/quotations/inquire-price";

        let response: StockPriceResponse = call_api(
            &self.oauth,
            &self.header,
            url,
            "FHKST01010100",
            &[
                ("FID_COND_MRKT_DIV_CODE", query.market_division_code),
                ("FID_INPUT_ISCD", query.stock_code),
            ],
        )
        .await?;

        if response.rt_cd != "0" {
            return Err(format!("API 오류: {} ({})", response.msg1, response.msg_cd).into());
        }
        Ok(response.output)
    }

    async fn get_inquire_price2(
        &self,
        stock_code: &str,
    ) -> Result<StockPrice2Output, Box<dyn Error>> {
        let query = QueryParam::stock(stock_code);
        let url = "https://openapi.koreainvestment.com:9443/uapi/domestic-stock/v1/quotations/inquire-price-2";

        let response: StockPrice2Response = call_api(
            &self.oauth,
            &self.header,
            url,
            "FHPST01010000",
            &[
                ("FID_COND_MRKT_DIV_CODE", query.market_division_code),
                ("FID_INPUT_ISCD", query.stock_code),
            ],
        )
        .await?;

        if response.rt_cd != "0" {
            return Err(format!("API 오류: {} ({})", response.msg1, response.msg_cd).into());
        }
        Ok(response.output)
    }

    async fn get_inquire_period_price(
        &self,
        stock_code: &str,
        from: &str,
        to: &str,
        period: &str,
    ) -> Result<PeriodPriceResponse, Box<dyn Error>> {
        let query = PeriodPriceQuery {
            fid_cond_mrkt_div_code: "J",
            fid_input_iscd: stock_code,
            fid_input_date_1: from,
            fid_input_date_2: to,
            fid_period_div_code: period,
            fid_org_adj_prc: "0",
        };
        let url = "https://openapi.koreainvestment.com:9443/uapi/domestic-stock/v1/quotations/inquire-daily-itemchartprice";

        let response: PeriodPriceResponse = call_api(
            &self.oauth,
            &self.header,
            url,
            "FHKST03010100",
            &[
                ("FID_COND_MRKT_DIV_CODE", query.fid_cond_mrkt_div_code),
                ("FID_INPUT_ISCD", query.fid_input_iscd),
                ("FID_INPUT_DATE_1", query.fid_input_date_1),
                ("FID_INPUT_DATE_2", query.fid_input_date_2),
                ("FID_PERIOD_DIV_CODE", query.fid_period_div_code),
                ("FID_ORG_ADJ_PRC", query.fid_org_adj_prc),
            ],
        )
        .await?;

        if response.rt_cd != "0" {
            return Err(format!("API 오류: {} ({})", response.msg1, response.msg_cd).into());
        }
        Ok(response)
    }

    async fn get_recent_ticks(&self, stock_code: &str) -> Result<Vec<Tick>, Box<dyn Error>> {
        let query = QueryParam::stock(stock_code);
        let url = "https://openapi.koreainvestment.com:9443/uapi/domestic-stock/v1/quotations/inquire-ccnl";

        let response: TicksResponse = call_api(
            &self.oauth,
            &self.header,
            url,
            "FHKST01010300",
            &[
                ("FID_COND_MRKT_DIV_CODE", query.market_division_code),
                ("FID_INPUT_ISCD", query.stock_code),
            ],
        )
        .await?;

        if response.rt_cd != "0" {
            return Err(format!("API 오류: {} ({})", response.msg1, response.msg_cd).into());
        }
        Ok(response.output)
    }

    async fn get_today_minutes(
        &self,
        stock_code: &str,
        interval: &str,
    ) -> Result<Vec<TodayMinuteCandle>, Box<dyn Error>> {
        let url = "https://openapi.koreainvestment.com:9443/uapi/domestic-stock/v1/quotations/inquire-timechartprice";

        let response: TodayMinuteResponse = call_api(
            &self.oauth,
            &self.header,
            url,
            "FHKST03010200",
            &[
                ("FID_COND_MRKT_DIV_CODE", "J"),
                ("FID_INPUT_ISCD", stock_code),
                ("FID_INPUT_HOUR_1", interval),
            ],
        )
        .await?;

        if response.rt_cd != "0" {
            return Err(format!("API 오류: {} ({})", response.msg1, response.msg_cd).into());
        }
        Ok(response.output)
    }

    async fn get_minutes_by_day(
        &self,
        stock_code: &str,
        date: &str,
        interval: &str,
    ) -> Result<Vec<ByDayMinuteCandle>, Box<dyn Error>> {
        let url = "https://openapi.koreainvestment.com:9443/uapi/domestic-stock/v1/quotations/inquire-time-dailychartprice";

        let response: ByDayMinutesResponse = call_api(
            &self.oauth,
            &self.header,
            url,
            "FHKST03010230",
            &[
                ("FID_COND_MRKT_DIV_CODE", "J"),
                ("FID_INPUT_ISCD", stock_code),
                ("FID_INPUT_DATE_1", date),
                ("FID_INPUT_HOUR_1", interval),
            ],
        )
        .await?;

        if response.rt_cd != "0" {
            return Err(format!("API 오류: {} ({})", response.msg1, response.msg_cd).into());
        }
        Ok(response.output)
    }
}
