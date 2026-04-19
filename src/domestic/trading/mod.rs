use crate::provider::KISProvider;
use crate::types::{AccountInfo, ContinuationKey};
use crate::utils::{
    ApiEndpoint, ApiHeader, ApiResponse, RawApiBody, TrId, call_get_api, call_post_api,
    create_hashkey,
};
use async_trait::async_trait;
use serde::Serialize;
use std::error::Error;

const ORDER_CASH_ENDPOINT: ApiEndpoint = ApiEndpoint::new(
    "/uapi/domestic-stock/v1/trading/order-cash",
    TrId::new("TTTC0012U", Some("VTTC0012U")),
);
const ORDER_CASH_SELL_ENDPOINT: ApiEndpoint = ApiEndpoint::new(
    "/uapi/domestic-stock/v1/trading/order-cash",
    TrId::new("TTTC0011U", Some("VTTC0011U")),
);
const ORDER_REVISE_CANCEL_ENDPOINT: ApiEndpoint = ApiEndpoint::new(
    "/uapi/domestic-stock/v1/trading/order-rvsecncl",
    TrId::new("TTTC0013U", Some("VTTC0013U")),
);
const BALANCE_ENDPOINT: ApiEndpoint = ApiEndpoint::new(
    "/uapi/domestic-stock/v1/trading/inquire-balance",
    TrId::new("TTTC8434R", Some("VTTC8434R")),
);
const POSSIBLE_ORDER_ENDPOINT: ApiEndpoint = ApiEndpoint::new(
    "/uapi/domestic-stock/v1/trading/inquire-psbl-order",
    TrId::new("TTTC8908R", Some("VTTC8908R")),
);
const DAILY_CCLD_RECENT_ENDPOINT: ApiEndpoint = ApiEndpoint::new(
    "/uapi/domestic-stock/v1/trading/inquire-daily-ccld",
    TrId::new("TTTC0081R", Some("VTTC0081R")),
);
const DAILY_CCLD_OLD_ENDPOINT: ApiEndpoint = ApiEndpoint::new(
    "/uapi/domestic-stock/v1/trading/inquire-daily-ccld",
    TrId::new("CTSC9215R", Some("VTSC9215R")),
);
const PSBL_REVISE_CANCEL_ENDPOINT: ApiEndpoint = ApiEndpoint::real_only(
    "/uapi/domestic-stock/v1/trading/inquire-psbl-rvsecncl",
    TrId::new("TTTC0084R", None),
);
const BALANCE_REALIZED_PL_ENDPOINT: ApiEndpoint = ApiEndpoint::real_only(
    "/uapi/domestic-stock/v1/trading/inquire-balance-rlz-pl",
    TrId::new("TTTC8494R", None),
);
const PERIOD_TRADE_PROFIT_ENDPOINT: ApiEndpoint = ApiEndpoint::real_only(
    "/uapi/domestic-stock/v1/trading/inquire-period-trade-profit",
    TrId::new("TTTC8715R", None),
);
const PERIOD_PROFIT_ENDPOINT: ApiEndpoint = ApiEndpoint::real_only(
    "/uapi/domestic-stock/v1/trading/inquire-period-profit",
    TrId::new("TTTC8708R", None),
);
const RESERVE_ORDER_ENDPOINT: ApiEndpoint = ApiEndpoint::real_only(
    "/uapi/domestic-stock/v1/trading/order-resv",
    TrId::new("CTSC0008U", None),
);
const RESERVE_ORDER_LIST_ENDPOINT: ApiEndpoint = ApiEndpoint::real_only(
    "/uapi/domestic-stock/v1/trading/order-resv-ccnl",
    TrId::new("CTSC0004R", None),
);
const RESERVE_ORDER_CANCEL_ENDPOINT: ApiEndpoint = ApiEndpoint::real_only(
    "/uapi/domestic-stock/v1/trading/order-resv-rvsecncl",
    TrId::new("CTSC0009U", None),
);
const RESERVE_ORDER_REVISE_ENDPOINT: ApiEndpoint = ApiEndpoint::real_only(
    "/uapi/domestic-stock/v1/trading/order-resv-rvsecncl",
    TrId::new("CTSC0013U", None),
);
const PENSION_BALANCE_ENDPOINT: ApiEndpoint = ApiEndpoint::real_only(
    "/uapi/domestic-stock/v1/trading/pension/inquire-balance",
    TrId::new("TTTC2208R", None),
);
const PENSION_DEPOSIT_ENDPOINT: ApiEndpoint = ApiEndpoint::real_only(
    "/uapi/domestic-stock/v1/trading/pension/inquire-deposit",
    TrId::new("TTTC0506R", None),
);
const PENSION_POSSIBLE_ORDER_ENDPOINT: ApiEndpoint = ApiEndpoint::real_only(
    "/uapi/domestic-stock/v1/trading/pension/inquire-psbl-order",
    TrId::new("TTTC0503R", None),
);
const PENSION_DAILY_CCLD_ENDPOINT: ApiEndpoint = ApiEndpoint::real_only(
    "/uapi/domestic-stock/v1/trading/pension/inquire-daily-ccld",
    TrId::new("TTTC2210R", None),
);
const PENSION_PRESENT_BALANCE_ENDPOINT: ApiEndpoint = ApiEndpoint::real_only(
    "/uapi/domestic-stock/v1/trading/pension/inquire-present-balance",
    TrId::new("TTTC2202R", None),
);

#[derive(Debug, Clone, Serialize)]
pub struct DomesticCashOrderRequest<'a> {
    #[serde(rename = "CANO")]
    pub cano: &'a str,
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: &'a str,
    #[serde(rename = "PDNO")]
    pub pdno: &'a str,
    #[serde(rename = "ORD_DVSN")]
    pub ord_dvsn: &'a str,
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: &'a str,
    #[serde(rename = "ORD_UNPR")]
    pub ord_unpr: &'a str,
    #[serde(rename = "SLL_TYPE", skip_serializing_if = "Option::is_none")]
    pub sll_type: Option<&'a str>,
    #[serde(rename = "CNDT_PRIC", skip_serializing_if = "Option::is_none")]
    pub cndt_pric: Option<&'a str>,
    #[serde(rename = "EXCG_ID_DVSN_CD", skip_serializing_if = "Option::is_none")]
    pub excg_id_dvsn_cd: Option<&'a str>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DomesticOrderRevisionRequest<'a> {
    #[serde(rename = "CANO")]
    pub cano: &'a str,
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: &'a str,
    #[serde(rename = "KRX_FWDG_ORD_ORGNO")]
    pub krx_fwdg_ord_orgno: &'a str,
    #[serde(rename = "ORGN_ODNO")]
    pub orgn_odno: &'a str,
    #[serde(rename = "ORD_DVSN")]
    pub ord_dvsn: &'a str,
    #[serde(rename = "RVSE_CNCL_DVSN_CD")]
    pub rvse_cncl_dvsn_cd: &'a str,
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: &'a str,
    #[serde(rename = "ORD_UNPR")]
    pub ord_unpr: &'a str,
    #[serde(rename = "QTY_ALL_ORD_YN")]
    pub qty_all_ord_yn: &'a str,
    #[serde(rename = "CNDT_PRIC", skip_serializing_if = "Option::is_none")]
    pub cndt_pric: Option<&'a str>,
    #[serde(rename = "EXCG_ID_DVSN_CD", skip_serializing_if = "Option::is_none")]
    pub excg_id_dvsn_cd: Option<&'a str>,
}

#[derive(Debug, Clone)]
pub struct DomesticBalanceRequest<'a> {
    pub account: &'a AccountInfo,
    pub afhr_flpr_yn: &'a str,
    pub ofl_yn: &'a str,
    pub inqr_dvsn: &'a str,
    pub unpr_dvsn: &'a str,
    pub fund_sttl_icld_yn: &'a str,
    pub fncg_amt_auto_rdpt_yn: &'a str,
    pub prcs_dvsn: &'a str,
    pub continuation: Option<&'a ContinuationKey>,
}

impl<'a> DomesticBalanceRequest<'a> {
    pub fn new(account: &'a AccountInfo) -> Self {
        Self {
            account,
            afhr_flpr_yn: "N",
            ofl_yn: "",
            inqr_dvsn: "01",
            unpr_dvsn: "01",
            fund_sttl_icld_yn: "N",
            fncg_amt_auto_rdpt_yn: "N",
            prcs_dvsn: "00",
            continuation: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DomesticPossibleOrderRequest<'a> {
    pub account: &'a AccountInfo,
    pub pdno: &'a str,
    pub ord_unpr: &'a str,
    pub ord_dvsn: &'a str,
    pub cma_evlu_amt_icld_yn: &'a str,
    pub ovrs_icld_yn: &'a str,
}

#[derive(Debug, Clone)]
pub struct DomesticDailyCcldRequest<'a> {
    pub account: &'a AccountInfo,
    pub inqr_strt_dt: &'a str,
    pub inqr_end_dt: &'a str,
    pub sll_buy_dvsn_cd: &'a str,
    pub pdno: &'a str,
    pub ord_gno_brno: &'a str,
    pub odno: &'a str,
    pub ccld_dvsn: &'a str,
    pub inqr_dvsn: &'a str,
    pub inqr_dvsn_1: &'a str,
    pub inqr_dvsn_3: &'a str,
    pub excg_id_dvsn_cd: &'a str,
    pub is_recent: bool,
}

#[derive(Debug, Clone)]
pub struct DomesticReviseCancelPossibleRequest<'a> {
    pub account: &'a AccountInfo,
    pub inqr_dvsn_1: &'a str,
    pub inqr_dvsn_2: &'a str,
    pub continuation: Option<&'a ContinuationKey>,
}

#[derive(Debug, Clone)]
pub struct DomesticRealizedProfitBalanceRequest<'a> {
    pub account: &'a AccountInfo,
    pub afhr_flpr_yn: &'a str,
    pub ofl_yn: &'a str,
    pub inqr_dvsn: &'a str,
    pub unpr_dvsn: &'a str,
    pub fund_sttl_icld_yn: &'a str,
    pub fncg_amt_auto_rdpt_yn: &'a str,
    pub prcs_dvsn: &'a str,
    pub cost_icld_yn: &'a str,
    pub continuation: Option<&'a ContinuationKey>,
}

#[derive(Debug, Clone)]
pub struct DomesticPeriodTradeProfitRequest<'a> {
    pub account: &'a AccountInfo,
    pub sort_dvsn: &'a str,
    pub pdno: &'a str,
    pub inqr_strt_dt: &'a str,
    pub inqr_end_dt: &'a str,
    pub cblc_dvsn: &'a str,
    pub continuation: Option<&'a ContinuationKey>,
}

#[derive(Debug, Clone)]
pub struct DomesticPeriodProfitRequest<'a> {
    pub account: &'a AccountInfo,
    pub inqr_strt_dt: &'a str,
    pub inqr_end_dt: &'a str,
    pub pdno: &'a str,
    pub sort_dvsn: &'a str,
    pub inqr_dvsn: &'a str,
    pub cblc_dvsn: &'a str,
    pub continuation: Option<&'a ContinuationKey>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DomesticReserveOrderRequest<'a> {
    #[serde(rename = "CANO")]
    pub cano: &'a str,
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: &'a str,
    #[serde(rename = "PDNO")]
    pub pdno: &'a str,
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: &'a str,
    #[serde(rename = "ORD_UNPR")]
    pub ord_unpr: &'a str,
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: &'a str,
    #[serde(rename = "ORD_DVSN_CD")]
    pub ord_dvsn_cd: &'a str,
    #[serde(rename = "ORD_OBJT_CBLC_DVSN_CD")]
    pub ord_objt_cblc_dvsn_cd: &'a str,
    #[serde(rename = "LOAN_DT", skip_serializing_if = "Option::is_none")]
    pub loan_dt: Option<&'a str>,
    #[serde(rename = "RSVN_ORD_END_DT", skip_serializing_if = "Option::is_none")]
    pub rsvn_ord_end_dt: Option<&'a str>,
    #[serde(rename = "LDNG_DT", skip_serializing_if = "Option::is_none")]
    pub ldng_dt: Option<&'a str>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DomesticReserveOrderRevisionRequest<'a> {
    #[serde(rename = "CANO")]
    pub cano: &'a str,
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: &'a str,
    #[serde(rename = "PDNO", skip_serializing_if = "Option::is_none")]
    pub pdno: Option<&'a str>,
    #[serde(rename = "ORD_QTY", skip_serializing_if = "Option::is_none")]
    pub ord_qty: Option<&'a str>,
    #[serde(rename = "ORD_UNPR", skip_serializing_if = "Option::is_none")]
    pub ord_unpr: Option<&'a str>,
    #[serde(rename = "SLL_BUY_DVSN_CD", skip_serializing_if = "Option::is_none")]
    pub sll_buy_dvsn_cd: Option<&'a str>,
    #[serde(rename = "ORD_DVSN_CD", skip_serializing_if = "Option::is_none")]
    pub ord_dvsn_cd: Option<&'a str>,
    #[serde(rename = "ORD_OBJT_CBLC_DVSN_CD", skip_serializing_if = "Option::is_none")]
    pub ord_objt_cblc_dvsn_cd: Option<&'a str>,
    #[serde(rename = "LOAN_DT", skip_serializing_if = "Option::is_none")]
    pub loan_dt: Option<&'a str>,
    #[serde(rename = "RSVN_ORD_END_DT", skip_serializing_if = "Option::is_none")]
    pub rsvn_ord_end_dt: Option<&'a str>,
    #[serde(rename = "CTAL_TLNO", skip_serializing_if = "Option::is_none")]
    pub ctal_tlno: Option<&'a str>,
    #[serde(rename = "RSVN_ORD_SEQ")]
    pub rsvn_ord_seq: &'a str,
    #[serde(rename = "RSVN_ORD_ORGNO", skip_serializing_if = "Option::is_none")]
    pub rsvn_ord_orgno: Option<&'a str>,
    #[serde(rename = "RSVN_ORD_ORD_DT", skip_serializing_if = "Option::is_none")]
    pub rsvn_ord_ord_dt: Option<&'a str>,
}

#[derive(Debug, Clone)]
pub struct DomesticReserveOrderListRequest<'a> {
    pub account: &'a AccountInfo,
    pub rsvn_ord_end_dt: &'a str,
    pub rsvn_ord_seq: &'a str,
    pub tmnl_mdia_kind_cd: &'a str,
    pub prcs_dvsn_cd: &'a str,
    pub cncl_yn: &'a str,
    pub pdno: &'a str,
    pub sll_buy_dvsn_cd: &'a str,
    pub continuation: Option<&'a ContinuationKey>,
}

#[derive(Debug, Clone)]
pub struct PensionBalanceRequest<'a> {
    pub account: &'a AccountInfo,
    pub acca_dvsn_cd: &'a str,
    pub inqr_dvsn: &'a str,
    pub continuation: Option<&'a ContinuationKey>,
}

#[derive(Debug, Clone)]
pub struct PensionDepositRequest<'a> {
    pub account: &'a AccountInfo,
    pub acca_dvsn_cd: &'a str,
}

#[derive(Debug, Clone)]
pub struct PensionPossibleOrderRequest<'a> {
    pub account: &'a AccountInfo,
    pub pdno: &'a str,
    pub acca_dvsn_cd: &'a str,
    pub cma_evlu_amt_icld_yn: &'a str,
    pub ord_dvsn: &'a str,
    pub ord_unpr: &'a str,
}

#[derive(Debug, Clone)]
pub struct PensionDailyCcldRequest<'a> {
    pub account: &'a AccountInfo,
    pub user_dvsn_cd: &'a str,
    pub sll_buy_dvsn_cd: &'a str,
    pub ccld_nccs_dvsn: &'a str,
    pub inqr_dvsn_3: &'a str,
    pub continuation: Option<&'a ContinuationKey>,
}

#[derive(Debug, Clone)]
pub struct PensionPresentBalanceRequest<'a> {
    pub account: &'a AccountInfo,
    pub user_dvsn_cd: &'a str,
    pub prcs_dvsn_cd: &'a str,
    pub continuation: Option<&'a ContinuationKey>,
}

#[async_trait]
pub trait DomesticTrading {
    async fn create_hashkey<T: Serialize + Send + Sync>(
        &self,
        body: &T,
    ) -> Result<String, Box<dyn Error>>;
    async fn place_cash_buy_order(
        &self,
        request: DomesticCashOrderRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>>;
    async fn place_cash_sell_order(
        &self,
        request: DomesticCashOrderRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>>;
    async fn revise_or_cancel_order(
        &self,
        request: DomesticOrderRevisionRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>>;
    async fn inquire_balance(
        &self,
        request: DomesticBalanceRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>>;
    async fn inquire_possible_order(
        &self,
        request: DomesticPossibleOrderRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>>;
    async fn inquire_daily_ccld(
        &self,
        request: DomesticDailyCcldRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>>;
    async fn inquire_psbl_rvsecncl(
        &self,
        request: DomesticReviseCancelPossibleRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>>;
    async fn inquire_balance_realized_pl(
        &self,
        request: DomesticRealizedProfitBalanceRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>>;
    async fn inquire_period_trade_profit(
        &self,
        request: DomesticPeriodTradeProfitRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>>;
    async fn inquire_period_profit(
        &self,
        request: DomesticPeriodProfitRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>>;
    async fn place_reserve_order(
        &self,
        request: DomesticReserveOrderRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>>;
    async fn revise_reserve_order(
        &self,
        request: DomesticReserveOrderRevisionRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>>;
    async fn cancel_reserve_order(
        &self,
        request: DomesticReserveOrderRevisionRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>>;
    async fn inquire_reserve_orders(
        &self,
        request: DomesticReserveOrderListRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>>;
    async fn inquire_pension_balance(
        &self,
        request: PensionBalanceRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>>;
    async fn inquire_pension_deposit(
        &self,
        request: PensionDepositRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>>;
    async fn inquire_pension_possible_order(
        &self,
        request: PensionPossibleOrderRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>>;
    async fn inquire_pension_daily_ccld(
        &self,
        request: PensionDailyCcldRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>>;
    async fn inquire_pension_present_balance(
        &self,
        request: PensionPresentBalanceRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>>;
}

async fn post_with_hashkey<T: Serialize>(
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
impl DomesticTrading for KISProvider {
    async fn create_hashkey<T: Serialize + Send + Sync>(
        &self,
        body: &T,
    ) -> Result<String, Box<dyn Error>> {
        create_hashkey(&self.oauth, self.practice, body).await
    }

    async fn place_cash_buy_order(
        &self,
        request: DomesticCashOrderRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>> {
        post_with_hashkey(self, ORDER_CASH_ENDPOINT, &request).await
    }

    async fn place_cash_sell_order(
        &self,
        request: DomesticCashOrderRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>> {
        post_with_hashkey(self, ORDER_CASH_SELL_ENDPOINT, &request).await
    }

    async fn revise_or_cancel_order(
        &self,
        request: DomesticOrderRevisionRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>> {
        post_with_hashkey(self, ORDER_REVISE_CANCEL_ENDPOINT, &request).await
    }

    async fn inquire_balance(
        &self,
        request: DomesticBalanceRequest<'_>,
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
            BALANCE_ENDPOINT,
            &[
                ("CANO", &request.account.cano),
                ("ACNT_PRDT_CD", &request.account.acnt_prdt_cd),
                ("AFHR_FLPR_YN", request.afhr_flpr_yn),
                ("OFL_YN", request.ofl_yn),
                ("INQR_DVSN", request.inqr_dvsn),
                ("UNPR_DVSN", request.unpr_dvsn),
                ("FUND_STTL_ICLD_YN", request.fund_sttl_icld_yn),
                ("FNCG_AMT_AUTO_RDPT_YN", request.fncg_amt_auto_rdpt_yn),
                ("PRCS_DVSN", request.prcs_dvsn),
                ("CTX_AREA_FK100", &continuation.fk),
                ("CTX_AREA_NK100", &continuation.nk),
            ],
        )
        .await?;
        response.body.ensure_success()?;
        Ok(response)
    }

    async fn inquire_possible_order(
        &self,
        request: DomesticPossibleOrderRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>> {
        let response = call_get_api::<RawApiBody>(
            &self.oauth,
            &self.header,
            self.practice,
            POSSIBLE_ORDER_ENDPOINT,
            &[
                ("CANO", &request.account.cano),
                ("ACNT_PRDT_CD", &request.account.acnt_prdt_cd),
                ("PDNO", request.pdno),
                ("ORD_UNPR", request.ord_unpr),
                ("ORD_DVSN", request.ord_dvsn),
                ("CMA_EVLU_AMT_ICLD_YN", request.cma_evlu_amt_icld_yn),
                ("OVRS_ICLD_YN", request.ovrs_icld_yn),
            ],
        )
        .await?;
        response.body.ensure_success()?;
        Ok(response)
    }

    async fn inquire_daily_ccld(
        &self,
        request: DomesticDailyCcldRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>> {
        let endpoint = if request.is_recent {
            DAILY_CCLD_RECENT_ENDPOINT
        } else {
            DAILY_CCLD_OLD_ENDPOINT
        };
        let response = call_get_api::<RawApiBody>(
            &self.oauth,
            &self.header,
            self.practice,
            endpoint,
            &[
                ("CANO", &request.account.cano),
                ("ACNT_PRDT_CD", &request.account.acnt_prdt_cd),
                ("INQR_STRT_DT", request.inqr_strt_dt),
                ("INQR_END_DT", request.inqr_end_dt),
                ("SLL_BUY_DVSN_CD", request.sll_buy_dvsn_cd),
                ("PDNO", request.pdno),
                ("ORD_GNO_BRNO", request.ord_gno_brno),
                ("ODNO", request.odno),
                ("CCLD_DVSN", request.ccld_dvsn),
                ("INQR_DVSN", request.inqr_dvsn),
                ("INQR_DVSN_1", request.inqr_dvsn_1),
                ("INQR_DVSN_3", request.inqr_dvsn_3),
                ("EXCG_ID_DVSN_CD", request.excg_id_dvsn_cd),
            ],
        )
        .await?;
        response.body.ensure_success()?;
        Ok(response)
    }

    async fn inquire_psbl_rvsecncl(
        &self,
        request: DomesticReviseCancelPossibleRequest<'_>,
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
            PSBL_REVISE_CANCEL_ENDPOINT,
            &[
                ("CANO", &request.account.cano),
                ("ACNT_PRDT_CD", &request.account.acnt_prdt_cd),
                ("CTX_AREA_FK100", &continuation.fk),
                ("CTX_AREA_NK100", &continuation.nk),
                ("INQR_DVSN_1", request.inqr_dvsn_1),
                ("INQR_DVSN_2", request.inqr_dvsn_2),
            ],
        )
        .await?;
        response.body.ensure_success()?;
        Ok(response)
    }

    async fn inquire_balance_realized_pl(
        &self,
        request: DomesticRealizedProfitBalanceRequest<'_>,
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
            BALANCE_REALIZED_PL_ENDPOINT,
            &[
                ("CANO", &request.account.cano),
                ("ACNT_PRDT_CD", &request.account.acnt_prdt_cd),
                ("AFHR_FLPR_YN", request.afhr_flpr_yn),
                ("OFL_YN", request.ofl_yn),
                ("INQR_DVSN", request.inqr_dvsn),
                ("UNPR_DVSN", request.unpr_dvsn),
                ("FUND_STTL_ICLD_YN", request.fund_sttl_icld_yn),
                ("FNCG_AMT_AUTO_RDPT_YN", request.fncg_amt_auto_rdpt_yn),
                ("PRCS_DVSN", request.prcs_dvsn),
                ("COST_ICLD_YN", request.cost_icld_yn),
                ("CTX_AREA_FK100", &continuation.fk),
                ("CTX_AREA_NK100", &continuation.nk),
            ],
        )
        .await?;
        response.body.ensure_success()?;
        Ok(response)
    }

    async fn inquire_period_trade_profit(
        &self,
        request: DomesticPeriodTradeProfitRequest<'_>,
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
            PERIOD_TRADE_PROFIT_ENDPOINT,
            &[
                ("CANO", &request.account.cano),
                ("SORT_DVSN", request.sort_dvsn),
                ("ACNT_PRDT_CD", &request.account.acnt_prdt_cd),
                ("PDNO", request.pdno),
                ("INQR_STRT_DT", request.inqr_strt_dt),
                ("INQR_END_DT", request.inqr_end_dt),
                ("CTX_AREA_NK100", &continuation.nk),
                ("CBLC_DVSN", request.cblc_dvsn),
                ("CTX_AREA_FK100", &continuation.fk),
            ],
        )
        .await?;
        response.body.ensure_success()?;
        Ok(response)
    }

    async fn inquire_period_profit(
        &self,
        request: DomesticPeriodProfitRequest<'_>,
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
            PERIOD_PROFIT_ENDPOINT,
            &[
                ("CANO", &request.account.cano),
                ("ACNT_PRDT_CD", &request.account.acnt_prdt_cd),
                ("INQR_STRT_DT", request.inqr_strt_dt),
                ("PDNO", request.pdno),
                ("CTX_AREA_NK100", &continuation.nk),
                ("INQR_END_DT", request.inqr_end_dt),
                ("SORT_DVSN", request.sort_dvsn),
                ("INQR_DVSN", request.inqr_dvsn),
                ("CBLC_DVSN", request.cblc_dvsn),
                ("CTX_AREA_FK100", &continuation.fk),
            ],
        )
        .await?;
        response.body.ensure_success()?;
        Ok(response)
    }

    async fn place_reserve_order(
        &self,
        request: DomesticReserveOrderRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>> {
        post_with_hashkey(self, RESERVE_ORDER_ENDPOINT, &request).await
    }

    async fn revise_reserve_order(
        &self,
        request: DomesticReserveOrderRevisionRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>> {
        post_with_hashkey(self, RESERVE_ORDER_REVISE_ENDPOINT, &request).await
    }

    async fn cancel_reserve_order(
        &self,
        request: DomesticReserveOrderRevisionRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>> {
        post_with_hashkey(self, RESERVE_ORDER_CANCEL_ENDPOINT, &request).await
    }

    async fn inquire_reserve_orders(
        &self,
        request: DomesticReserveOrderListRequest<'_>,
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
            RESERVE_ORDER_LIST_ENDPOINT,
            &[
                ("RSVN_ORD_END_DT", request.rsvn_ord_end_dt),
                ("RSVN_ORD_SEQ", request.rsvn_ord_seq),
                ("TMNL_MDIA_KIND_CD", request.tmnl_mdia_kind_cd),
                ("CANO", &request.account.cano),
                ("ACNT_PRDT_CD", &request.account.acnt_prdt_cd),
                ("PRCS_DVSN_CD", request.prcs_dvsn_cd),
                ("CNCL_YN", request.cncl_yn),
                ("PDNO", request.pdno),
                ("SLL_BUY_DVSN_CD", request.sll_buy_dvsn_cd),
                ("CTX_AREA_FK200", &continuation.fk),
                ("CTX_AREA_NK200", &continuation.nk),
            ],
        )
        .await?;
        response.body.ensure_success()?;
        Ok(response)
    }

    async fn inquire_pension_balance(
        &self,
        request: PensionBalanceRequest<'_>,
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
            PENSION_BALANCE_ENDPOINT,
            &[
                ("CANO", &request.account.cano),
                ("ACNT_PRDT_CD", &request.account.acnt_prdt_cd),
                ("ACCA_DVSN_CD", request.acca_dvsn_cd),
                ("INQR_DVSN", request.inqr_dvsn),
                ("CTX_AREA_FK100", &continuation.fk),
                ("CTX_AREA_NK100", &continuation.nk),
            ],
        )
        .await?;
        response.body.ensure_success()?;
        Ok(response)
    }

    async fn inquire_pension_deposit(
        &self,
        request: PensionDepositRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>> {
        let response = call_get_api::<RawApiBody>(
            &self.oauth,
            &self.header,
            self.practice,
            PENSION_DEPOSIT_ENDPOINT,
            &[
                ("CANO", &request.account.cano),
                ("ACNT_PRDT_CD", &request.account.acnt_prdt_cd),
                ("ACCA_DVSN_CD", request.acca_dvsn_cd),
            ],
        )
        .await?;
        response.body.ensure_success()?;
        Ok(response)
    }

    async fn inquire_pension_possible_order(
        &self,
        request: PensionPossibleOrderRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>> {
        let response = call_get_api::<RawApiBody>(
            &self.oauth,
            &self.header,
            self.practice,
            PENSION_POSSIBLE_ORDER_ENDPOINT,
            &[
                ("CANO", &request.account.cano),
                ("ACNT_PRDT_CD", &request.account.acnt_prdt_cd),
                ("PDNO", request.pdno),
                ("ACCA_DVSN_CD", request.acca_dvsn_cd),
                ("CMA_EVLU_AMT_ICLD_YN", request.cma_evlu_amt_icld_yn),
                ("ORD_DVSN", request.ord_dvsn),
                ("ORD_UNPR", request.ord_unpr),
            ],
        )
        .await?;
        response.body.ensure_success()?;
        Ok(response)
    }

    async fn inquire_pension_daily_ccld(
        &self,
        request: PensionDailyCcldRequest<'_>,
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
            PENSION_DAILY_CCLD_ENDPOINT,
            &[
                ("CANO", &request.account.cano),
                ("ACNT_PRDT_CD", &request.account.acnt_prdt_cd),
                ("USER_DVSN_CD", request.user_dvsn_cd),
                ("SLL_BUY_DVSN_CD", request.sll_buy_dvsn_cd),
                ("CCLD_NCCS_DVSN", request.ccld_nccs_dvsn),
                ("INQR_DVSN_3", request.inqr_dvsn_3),
                ("CTX_AREA_FK100", &continuation.fk),
                ("CTX_AREA_NK100", &continuation.nk),
            ],
        )
        .await?;
        response.body.ensure_success()?;
        Ok(response)
    }

    async fn inquire_pension_present_balance(
        &self,
        request: PensionPresentBalanceRequest<'_>,
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
            PENSION_PRESENT_BALANCE_ENDPOINT,
            &[
                ("CANO", &request.account.cano),
                ("ACNT_PRDT_CD", &request.account.acnt_prdt_cd),
                ("USER_DVSN_CD", request.user_dvsn_cd),
                ("CTX_AREA_FK100", &continuation.fk),
                ("CTX_AREA_NK100", &continuation.nk),
                ("PRCS_DVSN_CD", request.prcs_dvsn_cd),
            ],
        )
        .await?;
        response.body.ensure_success()?;
        Ok(response)
    }
}
