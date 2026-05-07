use crate::provider::KISProvider;
use crate::types::{AccountInfo, ContinuationKey};
use crate::utils::{
    ApiEndpoint, ApiHeader, ApiResponse, RawApiBody, TrId, call_get_api, call_post_api,
    create_hashkey,
};
use async_trait::async_trait;
use serde::Serialize;
use std::error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FutureOptionSession {
    Day,
    Night,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DomesticFutureOptionGetEndpoint {
    DisplayBoardCallPut,
    DisplayBoardFutures,
    DisplayBoardOptionList,
    DisplayBoardTop,
    ExpectedPriceTrend,
    AskingPrice,
    Balance,
    BalanceSettlementProfitLoss,
    BalanceValuationProfitLoss,
    Ccld,
    CcldByBaseTime,
    DailyAmountFee,
    DailyChartPrice,
    Deposit,
    NightBalance,
    NightCcld,
    Price,
    PossibleNightOrder,
    PossibleOrder,
    TimeChartPrice,
    NightMarginDetail,
}

impl DomesticFutureOptionGetEndpoint {
    fn endpoint(self) -> ApiEndpoint {
        match self {
            Self::DisplayBoardCallPut => ApiEndpoint::real_only(
                "/uapi/domestic-futureoption/v1/quotations/display-board-callput",
                TrId::new("FHPIF05030100", None),
            ),
            Self::DisplayBoardFutures => ApiEndpoint::real_only(
                "/uapi/domestic-futureoption/v1/quotations/display-board-futures",
                TrId::new("FHPIF05030200", None),
            ),
            Self::DisplayBoardOptionList => ApiEndpoint::real_only(
                "/uapi/domestic-futureoption/v1/quotations/display-board-option-list",
                TrId::new("FHPIO056104C0", None),
            ),
            Self::DisplayBoardTop => ApiEndpoint::real_only(
                "/uapi/domestic-futureoption/v1/quotations/display-board-top",
                TrId::new("FHPIF05030000", None),
            ),
            Self::ExpectedPriceTrend => ApiEndpoint::real_only(
                "/uapi/domestic-futureoption/v1/quotations/exp-price-trend",
                TrId::new("FHPIF05110100", None),
            ),
            Self::AskingPrice => ApiEndpoint::new(
                "/uapi/domestic-futureoption/v1/quotations/inquire-asking-price",
                TrId::new("FHMIF10010000", Some("FHMIF10010000")),
            ),
            Self::Balance => ApiEndpoint::new(
                "/uapi/domestic-futureoption/v1/trading/inquire-balance",
                TrId::new("CTFO6118R", Some("VTFO6118R")),
            ),
            Self::BalanceSettlementProfitLoss => ApiEndpoint::real_only(
                "/uapi/domestic-futureoption/v1/trading/inquire-balance-settlement-pl",
                TrId::new("CTFO6117R", None),
            ),
            Self::BalanceValuationProfitLoss => ApiEndpoint::real_only(
                "/uapi/domestic-futureoption/v1/trading/inquire-balance-valuation-pl",
                TrId::new("CTFO6159R", None),
            ),
            Self::Ccld => ApiEndpoint::new(
                "/uapi/domestic-futureoption/v1/trading/inquire-ccnl",
                TrId::new("TTTO5201R", Some("VTTO5201R")),
            ),
            Self::CcldByBaseTime => ApiEndpoint::real_only(
                "/uapi/domestic-futureoption/v1/trading/inquire-ccnl-bstime",
                TrId::new("CTFO5139R", None),
            ),
            Self::DailyAmountFee => ApiEndpoint::real_only(
                "/uapi/domestic-futureoption/v1/trading/inquire-daily-amount-fee",
                TrId::new("CTFO6119R", None),
            ),
            Self::DailyChartPrice => ApiEndpoint::new(
                "/uapi/domestic-futureoption/v1/quotations/inquire-daily-fuopchartprice",
                TrId::new("FHKIF03020100", Some("FHKIF03020100")),
            ),
            Self::Deposit => ApiEndpoint::real_only(
                "/uapi/domestic-futureoption/v1/trading/inquire-deposit",
                TrId::new("CTRP6550R", None),
            ),
            Self::NightBalance => ApiEndpoint::real_only(
                "/uapi/domestic-futureoption/v1/trading/inquire-ngt-balance",
                TrId::new("CTFN6118R", None),
            ),
            Self::NightCcld => ApiEndpoint::real_only(
                "/uapi/domestic-futureoption/v1/trading/inquire-ngt-ccnl",
                TrId::new("STTN5201R", None),
            ),
            Self::Price => ApiEndpoint::new(
                "/uapi/domestic-futureoption/v1/quotations/inquire-price",
                TrId::new("FHMIF10000000", Some("FHMIF10000000")),
            ),
            Self::PossibleNightOrder => ApiEndpoint::real_only(
                "/uapi/domestic-futureoption/v1/trading/inquire-psbl-ngt-order",
                TrId::new("STTN5105R", None),
            ),
            Self::PossibleOrder => ApiEndpoint::new(
                "/uapi/domestic-futureoption/v1/trading/inquire-psbl-order",
                TrId::new("TTTO5105R", Some("VTTO5105R")),
            ),
            Self::TimeChartPrice => ApiEndpoint::real_only(
                "/uapi/domestic-futureoption/v1/quotations/inquire-time-fuopchartprice",
                TrId::new("FHKIF03020200", None),
            ),
            Self::NightMarginDetail => ApiEndpoint::real_only(
                "/uapi/domestic-futureoption/v1/trading/ngt-margin-detail",
                TrId::new("CTFN7107R", None),
            ),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverseasFutureOptionGetEndpoint {
    DailyCcld,
    AskingPrice,
    Ccld,
    DailyCcldAccount,
    DailyOrder,
    Deposit,
    PeriodCcld,
    PeriodTransaction,
    Price,
    PossibleAmount,
    FutureTimeChartPrice,
    OptionTimeChartPrice,
    Unpaid,
    InvestorUnpaidTrend,
    MarginDetail,
    MarketTime,
    MonthlyCcld,
    OptionAskingPrice,
    OptionDailyCcld,
    OptionDetail,
    OptionMonthlyCcld,
    OptionPrice,
    OptionTickCcld,
    OptionWeeklyCcld,
    SearchContractDetail,
    SearchOptionDetail,
    StockDetail,
    TickCcld,
    WeeklyCcld,
}

impl OverseasFutureOptionGetEndpoint {
    fn endpoint(self) -> ApiEndpoint {
        match self {
            Self::DailyCcld => ApiEndpoint::real_only(
                "/uapi/overseas-futureoption/v1/quotations/daily-ccnl",
                TrId::new("HHDFC55020100", None),
            ),
            Self::AskingPrice => ApiEndpoint::real_only(
                "/uapi/overseas-futureoption/v1/quotations/inquire-asking-price",
                TrId::new("HHDFC86000000", None),
            ),
            Self::Ccld => ApiEndpoint::real_only(
                "/uapi/overseas-futureoption/v1/trading/inquire-ccld",
                TrId::new("OTFM3116R", None),
            ),
            Self::DailyCcldAccount => ApiEndpoint::real_only(
                "/uapi/overseas-futureoption/v1/trading/inquire-daily-ccld",
                TrId::new("OTFM3122R", None),
            ),
            Self::DailyOrder => ApiEndpoint::real_only(
                "/uapi/overseas-futureoption/v1/trading/inquire-daily-order",
                TrId::new("OTFM3120R", None),
            ),
            Self::Deposit => ApiEndpoint::real_only(
                "/uapi/overseas-futureoption/v1/trading/inquire-deposit",
                TrId::new("OTFM1411R", None),
            ),
            Self::PeriodCcld => ApiEndpoint::real_only(
                "/uapi/overseas-futureoption/v1/trading/inquire-period-ccld",
                TrId::new("OTFM3118R", None),
            ),
            Self::PeriodTransaction => ApiEndpoint::real_only(
                "/uapi/overseas-futureoption/v1/trading/inquire-period-trans",
                TrId::new("OTFM3114R", None),
            ),
            Self::Price => ApiEndpoint::real_only(
                "/uapi/overseas-futureoption/v1/quotations/inquire-price",
                TrId::new("HHDFC55010000", None),
            ),
            Self::PossibleAmount => ApiEndpoint::real_only(
                "/uapi/overseas-futureoption/v1/trading/inquire-psamount",
                TrId::new("OTFM3304R", None),
            ),
            Self::FutureTimeChartPrice => ApiEndpoint::real_only(
                "/uapi/overseas-futureoption/v1/quotations/inquire-time-futurechartprice",
                TrId::new("HHDFC55020400", None),
            ),
            Self::OptionTimeChartPrice => ApiEndpoint::real_only(
                "/uapi/overseas-futureoption/v1/quotations/inquire-time-optchartprice",
                TrId::new("HHDFO55020100", None),
            ),
            Self::Unpaid => ApiEndpoint::real_only(
                "/uapi/overseas-futureoption/v1/trading/inquire-unpd",
                TrId::new("OTFM1412R", None),
            ),
            Self::InvestorUnpaidTrend => ApiEndpoint::real_only(
                "/uapi/overseas-futureoption/v1/quotations/investor-unpd-trend",
                TrId::new("HHDDB95030000", None),
            ),
            Self::MarginDetail => ApiEndpoint::real_only(
                "/uapi/overseas-futureoption/v1/trading/margin-detail",
                TrId::new("OTFM3115R", None),
            ),
            Self::MarketTime => ApiEndpoint::real_only(
                "/uapi/overseas-futureoption/v1/quotations/market-time",
                TrId::new("OTFM2229R", None),
            ),
            Self::MonthlyCcld => ApiEndpoint::real_only(
                "/uapi/overseas-futureoption/v1/quotations/monthly-ccnl",
                TrId::new("HHDFC55020300", None),
            ),
            Self::OptionAskingPrice => ApiEndpoint::real_only(
                "/uapi/overseas-futureoption/v1/quotations/opt-asking-price",
                TrId::new("HHDFO86000000", None),
            ),
            Self::OptionDailyCcld => ApiEndpoint::real_only(
                "/uapi/overseas-futureoption/v1/quotations/opt-daily-ccnl",
                TrId::new("HHDFO55020100", None),
            ),
            Self::OptionDetail => ApiEndpoint::real_only(
                "/uapi/overseas-futureoption/v1/quotations/opt-detail",
                TrId::new("HHDFO55010100", None),
            ),
            Self::OptionMonthlyCcld => ApiEndpoint::real_only(
                "/uapi/overseas-futureoption/v1/quotations/opt-monthly-ccnl",
                TrId::new("HHDFO55020300", None),
            ),
            Self::OptionPrice => ApiEndpoint::real_only(
                "/uapi/overseas-futureoption/v1/quotations/opt-price",
                TrId::new("HHDFO55010000", None),
            ),
            Self::OptionTickCcld => ApiEndpoint::real_only(
                "/uapi/overseas-futureoption/v1/quotations/opt-tick-ccnl",
                TrId::new("HHDFO55020200", None),
            ),
            Self::OptionWeeklyCcld => ApiEndpoint::real_only(
                "/uapi/overseas-futureoption/v1/quotations/opt-weekly-ccnl",
                TrId::new("HHDFO55020000", None),
            ),
            Self::SearchContractDetail => ApiEndpoint::real_only(
                "/uapi/overseas-futureoption/v1/quotations/search-contract-detail",
                TrId::new("HHDFC55200000", None),
            ),
            Self::SearchOptionDetail => ApiEndpoint::real_only(
                "/uapi/overseas-futureoption/v1/quotations/search-opt-detail",
                TrId::new("HHDFO55200000", None),
            ),
            Self::StockDetail => ApiEndpoint::real_only(
                "/uapi/overseas-futureoption/v1/quotations/stock-detail",
                TrId::new("HHDFC55010100", None),
            ),
            Self::TickCcld => ApiEndpoint::real_only(
                "/uapi/overseas-futureoption/v1/quotations/tick-ccnl",
                TrId::new("HHDFC55020200", None),
            ),
            Self::WeeklyCcld => ApiEndpoint::real_only(
                "/uapi/overseas-futureoption/v1/quotations/weekly-ccnl",
                TrId::new("HHDFC55020000", None),
            ),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct DomesticFutureOrderRequest<'a> {
    #[serde(rename = "ORD_PRCS_DVSN_CD")]
    pub ord_prcs_dvsn_cd: &'a str,
    #[serde(rename = "CANO")]
    pub cano: &'a str,
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: &'a str,
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: &'a str,
    #[serde(rename = "SHTN_PDNO")]
    pub shtn_pdno: &'a str,
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: &'a str,
    #[serde(rename = "UNIT_PRICE")]
    pub unit_price: &'a str,
    #[serde(rename = "NMPR_TYPE_CD")]
    pub nmpr_type_cd: &'a str,
    #[serde(rename = "KRX_NMPR_CNDT_CD")]
    pub krx_nmpr_cndt_cd: &'a str,
    #[serde(rename = "ORD_DVSN_CD")]
    pub ord_dvsn_cd: &'a str,
    #[serde(rename = "CTAC_TLNO")]
    pub ctac_tlno: &'a str,
    #[serde(rename = "FUOP_ITEM_DVSN_CD")]
    pub fuop_item_dvsn_cd: &'a str,
}

#[derive(Debug, Clone, Serialize)]
pub struct DomesticFutureOrderRevisionRequest<'a> {
    #[serde(rename = "ORD_PRCS_DVSN_CD")]
    pub ord_prcs_dvsn_cd: &'a str,
    #[serde(rename = "CANO")]
    pub cano: &'a str,
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: &'a str,
    #[serde(rename = "RVSE_CNCL_DVSN_CD")]
    pub rvse_cncl_dvsn_cd: &'a str,
    #[serde(rename = "ORGN_ODNO")]
    pub orgn_odno: &'a str,
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: &'a str,
    #[serde(rename = "UNIT_PRICE")]
    pub unit_price: &'a str,
    #[serde(rename = "NMPR_TYPE_CD")]
    pub nmpr_type_cd: &'a str,
    #[serde(rename = "KRX_NMPR_CNDT_CD")]
    pub krx_nmpr_cndt_cd: &'a str,
    #[serde(rename = "RMN_QTY_YN")]
    pub rmn_qty_yn: &'a str,
    #[serde(rename = "ORD_DVSN_CD")]
    pub ord_dvsn_cd: &'a str,
    #[serde(rename = "FUOP_ITEM_DVSN_CD")]
    pub fuop_item_dvsn_cd: &'a str,
}

#[derive(Debug, Clone)]
pub struct DomesticFutureBalanceRequest<'a> {
    pub account: &'a AccountInfo,
    pub mgna_dvsn: &'a str,
    pub excc_stat_cd: &'a str,
    pub continuation: Option<&'a ContinuationKey>,
}

#[derive(Debug, Clone)]
pub struct DomesticFutureCcldRequest<'a> {
    pub account: &'a AccountInfo,
    pub start_order_date: &'a str,
    pub end_order_date: &'a str,
    pub sll_buy_dvsn_cd: &'a str,
    pub ccld_nccs_dvsn: &'a str,
    pub sort_sqn: &'a str,
    pub pdno: &'a str,
    pub start_odno: &'a str,
    pub market_id_code: &'a str,
    pub continuation: Option<&'a ContinuationKey>,
}

#[derive(Debug, Clone)]
pub struct DomesticFuturePossibleOrderRequest<'a> {
    pub account: &'a AccountInfo,
    pub pdno: &'a str,
    pub sll_buy_dvsn_cd: &'a str,
    pub unit_price: &'a str,
    pub ord_dvsn_cd: &'a str,
}

#[derive(Debug, Clone)]
pub struct DomesticFuturePriceRequest<'a> {
    pub market_div_code: &'a str,
    pub input_iscd: &'a str,
}

#[derive(Debug, Clone)]
pub struct DomesticFutureTimeChartRequest<'a> {
    pub market_div_code: &'a str,
    pub input_iscd: &'a str,
    pub hour_cls_code: &'a str,
    pub include_past_data: &'a str,
    pub include_fake_tick: &'a str,
    pub input_date: &'a str,
    pub input_hour: &'a str,
}

#[derive(Debug, Clone, Serialize)]
pub struct OverseasFutureOrderRequest<'a> {
    #[serde(rename = "CANO")]
    pub cano: &'a str,
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: &'a str,
    #[serde(rename = "OVRS_FUTR_FX_PDNO")]
    pub ovrs_futr_fx_pdno: &'a str,
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: &'a str,
    #[serde(rename = "FM_LQD_USTL_CCLD_DT")]
    pub fm_lqd_ustl_ccld_dt: &'a str,
    #[serde(rename = "FM_LQD_USTL_CCNO")]
    pub fm_lqd_ustl_ccno: &'a str,
    #[serde(rename = "PRIC_DVSN_CD")]
    pub pric_dvsn_cd: &'a str,
    #[serde(rename = "FM_LIMIT_ORD_PRIC")]
    pub fm_limit_ord_pric: &'a str,
    #[serde(rename = "FM_STOP_ORD_PRIC")]
    pub fm_stop_ord_pric: &'a str,
    #[serde(rename = "FM_ORD_QTY")]
    pub fm_ord_qty: &'a str,
    #[serde(rename = "FM_LQD_LMT_ORD_PRIC")]
    pub fm_lqd_lmt_ord_pric: &'a str,
    #[serde(rename = "FM_LQD_STOP_ORD_PRIC")]
    pub fm_lqd_stop_ord_pric: &'a str,
    #[serde(rename = "CCLD_CNDT_CD")]
    pub ccld_cndt_cd: &'a str,
    #[serde(rename = "CPLX_ORD_DVSN_CD")]
    pub cplx_ord_dvsn_cd: &'a str,
    #[serde(rename = "ECIS_RSVN_ORD_YN")]
    pub ecis_rsvn_ord_yn: &'a str,
    #[serde(rename = "FM_HDGE_ORD_SCRN_YN")]
    pub fm_hdge_ord_scrn_yn: &'a str,
}

#[derive(Debug, Clone, Serialize)]
pub struct OverseasFutureOrderRevisionRequest<'a> {
    #[serde(rename = "CANO")]
    pub cano: &'a str,
    #[serde(skip)]
    pub ord_dv: &'a str,
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: &'a str,
    #[serde(rename = "ORGN_ORD_DT")]
    pub orgn_ord_dt: &'a str,
    #[serde(rename = "ORGN_ODNO")]
    pub orgn_odno: &'a str,
    #[serde(rename = "FM_LIMIT_ORD_PRIC")]
    pub fm_limit_ord_pric: &'a str,
    #[serde(rename = "FM_STOP_ORD_PRIC")]
    pub fm_stop_ord_pric: &'a str,
    #[serde(rename = "FM_LQD_LMT_ORD_PRIC")]
    pub fm_lqd_lmt_ord_pric: &'a str,
    #[serde(rename = "FM_LQD_STOP_ORD_PRIC")]
    pub fm_lqd_stop_ord_pric: &'a str,
    #[serde(rename = "FM_HDGE_ORD_SCRN_YN")]
    pub fm_hdge_ord_scrn_yn: &'a str,
    #[serde(rename = "FM_MKPR_CVSN_YN")]
    pub fm_mkpr_cvsn_yn: &'a str,
}

#[async_trait]
pub trait DomesticFutureOptionTrading {
    async fn place_future_option_order(
        &self,
        session: FutureOptionSession,
        request: DomesticFutureOrderRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>>;

    async fn revise_or_cancel_future_option_order(
        &self,
        session: FutureOptionSession,
        request: DomesticFutureOrderRevisionRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>>;

    async fn inquire_future_option_balance(
        &self,
        request: DomesticFutureBalanceRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>>;

    async fn inquire_future_option_ccld(
        &self,
        request: DomesticFutureCcldRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>>;

    async fn inquire_future_option_possible_order(
        &self,
        request: DomesticFuturePossibleOrderRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>>;

    async fn get_future_option_price(
        &self,
        request: DomesticFuturePriceRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>>;

    async fn get_future_option_asking_price(
        &self,
        request: DomesticFuturePriceRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>>;

    async fn get_future_option_time_chart(
        &self,
        request: DomesticFutureTimeChartRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>>;

    async fn get_domestic_future_option_raw(
        &self,
        endpoint: DomesticFutureOptionGetEndpoint,
        query: &[(&str, &str)],
        continuation: Option<&str>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>>;
}

#[async_trait]
pub trait OverseasFutureOptionTrading {
    async fn place_overseas_future_option_order(
        &self,
        request: OverseasFutureOrderRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>>;

    async fn revise_or_cancel_overseas_future_option_order(
        &self,
        request: OverseasFutureOrderRevisionRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>>;

    async fn get_overseas_future_option_raw(
        &self,
        endpoint: OverseasFutureOptionGetEndpoint,
        query: &[(&str, &str)],
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>>;
}

fn domestic_order_endpoint(session: FutureOptionSession, practice: bool) -> ApiEndpoint {
    match (session, practice) {
        (FutureOptionSession::Day, false) => ApiEndpoint::real_only(
            "/uapi/domestic-futureoption/v1/trading/order",
            TrId::new("TTTO1101U", None),
        ),
        (FutureOptionSession::Night, false) => ApiEndpoint::real_only(
            "/uapi/domestic-futureoption/v1/trading/order",
            TrId::new("STTN1101U", None),
        ),
        (FutureOptionSession::Day, true) => ApiEndpoint::new(
            "/uapi/domestic-futureoption/v1/trading/order",
            TrId::new("TTTO1101U", Some("VTTO1101U")),
        ),
        (FutureOptionSession::Night, true) => ApiEndpoint::real_only(
            "/uapi/domestic-futureoption/v1/trading/order",
            TrId::new("STTN1101U", None),
        ),
    }
}

fn domestic_revise_cancel_endpoint(session: FutureOptionSession, practice: bool) -> ApiEndpoint {
    match (session, practice) {
        (FutureOptionSession::Day, false) => ApiEndpoint::real_only(
            "/uapi/domestic-futureoption/v1/trading/order-rvsecncl",
            TrId::new("TTTO1103U", None),
        ),
        (FutureOptionSession::Night, false) => ApiEndpoint::real_only(
            "/uapi/domestic-futureoption/v1/trading/order-rvsecncl",
            TrId::new("TTTN1103U", None),
        ),
        (FutureOptionSession::Day, true) => ApiEndpoint::new(
            "/uapi/domestic-futureoption/v1/trading/order-rvsecncl",
            TrId::new("TTTO1103U", Some("VTTO1103U")),
        ),
        (FutureOptionSession::Night, true) => ApiEndpoint::real_only(
            "/uapi/domestic-futureoption/v1/trading/order-rvsecncl",
            TrId::new("TTTN1103U", None),
        ),
    }
}

async fn post_with_hashkey<T: Serialize>(
    provider: &KISProvider,
    endpoint: ApiEndpoint,
    body: &T,
) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>> {
    let hashkey = create_hashkey(&provider.oauth, provider.practice, body).await?;
    let header = ApiHeader::personal().with_hashkey(Some(hashkey.as_str()));
    let response =
        call_post_api::<RawApiBody, _>(&provider.oauth, &header, provider.practice, endpoint, body)
            .await?;
    response.body.ensure_success()?;
    Ok(response)
}

#[async_trait]
impl DomesticFutureOptionTrading for KISProvider {
    async fn place_future_option_order(
        &self,
        session: FutureOptionSession,
        request: DomesticFutureOrderRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>> {
        post_with_hashkey(
            self,
            domestic_order_endpoint(session, self.practice),
            &request,
        )
        .await
    }

    async fn revise_or_cancel_future_option_order(
        &self,
        session: FutureOptionSession,
        request: DomesticFutureOrderRevisionRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>> {
        post_with_hashkey(
            self,
            domestic_revise_cancel_endpoint(session, self.practice),
            &request,
        )
        .await
    }

    async fn inquire_future_option_balance(
        &self,
        request: DomesticFutureBalanceRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>> {
        let (fk, nk) = continuation_pair(request.continuation);
        self.get_domestic_future_option_raw(
            DomesticFutureOptionGetEndpoint::Balance,
            &[
                ("CANO", &request.account.cano),
                ("ACNT_PRDT_CD", &request.account.acnt_prdt_cd),
                ("MGNA_DVSN", request.mgna_dvsn),
                ("EXCC_STAT_CD", request.excc_stat_cd),
                ("CTX_AREA_FK200", fk),
                ("CTX_AREA_NK200", nk),
            ],
            request.continuation.map(|_| "N"),
        )
        .await
    }

    async fn inquire_future_option_ccld(
        &self,
        request: DomesticFutureCcldRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>> {
        let (fk, nk) = continuation_pair(request.continuation);
        self.get_domestic_future_option_raw(
            DomesticFutureOptionGetEndpoint::Ccld,
            &[
                ("CANO", &request.account.cano),
                ("ACNT_PRDT_CD", &request.account.acnt_prdt_cd),
                ("STRT_ORD_DT", request.start_order_date),
                ("END_ORD_DT", request.end_order_date),
                ("SLL_BUY_DVSN_CD", request.sll_buy_dvsn_cd),
                ("CCLD_NCCS_DVSN", request.ccld_nccs_dvsn),
                ("SORT_SQN", request.sort_sqn),
                ("PDNO", request.pdno),
                ("STRT_ODNO", request.start_odno),
                ("MKET_ID_CD", request.market_id_code),
                ("CTX_AREA_FK200", fk),
                ("CTX_AREA_NK200", nk),
            ],
            request.continuation.map(|_| "N"),
        )
        .await
    }

    async fn inquire_future_option_possible_order(
        &self,
        request: DomesticFuturePossibleOrderRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>> {
        self.get_domestic_future_option_raw(
            DomesticFutureOptionGetEndpoint::PossibleOrder,
            &[
                ("CANO", &request.account.cano),
                ("ACNT_PRDT_CD", &request.account.acnt_prdt_cd),
                ("PDNO", request.pdno),
                ("SLL_BUY_DVSN_CD", request.sll_buy_dvsn_cd),
                ("UNIT_PRICE", request.unit_price),
                ("ORD_DVSN_CD", request.ord_dvsn_cd),
            ],
            None,
        )
        .await
    }

    async fn get_future_option_price(
        &self,
        request: DomesticFuturePriceRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>> {
        future_option_price_query(self, DomesticFutureOptionGetEndpoint::Price, request).await
    }

    async fn get_future_option_asking_price(
        &self,
        request: DomesticFuturePriceRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>> {
        future_option_price_query(self, DomesticFutureOptionGetEndpoint::AskingPrice, request).await
    }

    async fn get_future_option_time_chart(
        &self,
        request: DomesticFutureTimeChartRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>> {
        self.get_domestic_future_option_raw(
            DomesticFutureOptionGetEndpoint::TimeChartPrice,
            &[
                ("FID_COND_MRKT_DIV_CODE", request.market_div_code),
                ("FID_INPUT_ISCD", request.input_iscd),
                ("FID_HOUR_CLS_CODE", request.hour_cls_code),
                ("FID_PW_DATA_INCU_YN", request.include_past_data),
                ("FID_FAKE_TICK_INCU_YN", request.include_fake_tick),
                ("FID_INPUT_DATE_1", request.input_date),
                ("FID_INPUT_HOUR_1", request.input_hour),
            ],
            None,
        )
        .await
    }

    async fn get_domestic_future_option_raw(
        &self,
        endpoint: DomesticFutureOptionGetEndpoint,
        query: &[(&str, &str)],
        continuation: Option<&str>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>> {
        let header = self.header.clone().with_tr_cont(continuation);
        let response = call_get_api::<RawApiBody>(
            &self.oauth,
            &header,
            self.practice,
            endpoint.endpoint(),
            query,
        )
        .await?;
        response.body.ensure_success()?;
        Ok(response)
    }
}

#[async_trait]
impl OverseasFutureOptionTrading for KISProvider {
    async fn place_overseas_future_option_order(
        &self,
        request: OverseasFutureOrderRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>> {
        let endpoint = ApiEndpoint::real_only(
            "/uapi/overseas-futureoption/v1/trading/order",
            TrId::new("OTFM3001U", None),
        );
        post_with_hashkey(self, endpoint, &request).await
    }

    async fn revise_or_cancel_overseas_future_option_order(
        &self,
        request: OverseasFutureOrderRevisionRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>> {
        let tr_id = match request.ord_dv {
            "0" => "OTFM3002U",
            "1" => "OTFM3003U",
            _ => return Err("ord_dv는 0(정정) 또는 1(취소)이어야 합니다".into()),
        };
        let endpoint = ApiEndpoint::real_only(
            "/uapi/overseas-futureoption/v1/trading/order-rvsecncl",
            TrId::new(tr_id, None),
        );
        post_with_hashkey(self, endpoint, &request).await
    }

    async fn get_overseas_future_option_raw(
        &self,
        endpoint: OverseasFutureOptionGetEndpoint,
        query: &[(&str, &str)],
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>> {
        let response = call_get_api::<RawApiBody>(
            &self.oauth,
            &self.header,
            self.practice,
            endpoint.endpoint(),
            query,
        )
        .await?;
        response.body.ensure_success()?;
        Ok(response)
    }
}

async fn future_option_price_query(
    provider: &KISProvider,
    endpoint: DomesticFutureOptionGetEndpoint,
    request: DomesticFuturePriceRequest<'_>,
) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>> {
    provider
        .get_domestic_future_option_raw(
            endpoint,
            &[
                ("FID_COND_MRKT_DIV_CODE", request.market_div_code),
                ("FID_INPUT_ISCD", request.input_iscd),
            ],
            None,
        )
        .await
}

fn continuation_pair(continuation: Option<&ContinuationKey>) -> (&str, &str) {
    continuation
        .map(|key| (key.fk.as_str(), key.nk.as_str()))
        .unwrap_or(("", ""))
}
