use crate::provider::KISProvider;
use crate::types::AccountInfo;
use crate::utils::{ApiEndpoint, ApiResponse, RawApiBody, TrId, call_get_api};
use async_trait::async_trait;
use std::error::Error;

const DOMESTIC_FUTURES_PSBL_ORDER_ENDPOINT: ApiEndpoint = ApiEndpoint::new(
    "/uapi/domestic-futureoption/v1/trading/inquire-psbl-order",
    TrId::new("TTTO5105R", Some("VTTO5105R")),
);

#[derive(Debug, Clone)]
pub struct DomesticFuturePossibleOrderRequest<'a> {
    pub account: &'a AccountInfo,
    pub pdno: &'a str,
    pub sll_buy_dvsn_cd: &'a str,
    pub unit_price: &'a str,
    pub ord_dvsn_cd: &'a str,
}

#[async_trait]
pub trait DomesticFutureOptionTrading {
    async fn inquire_future_option_possible_order(
        &self,
        request: DomesticFuturePossibleOrderRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>>;
}

#[async_trait]
impl DomesticFutureOptionTrading for KISProvider {
    async fn inquire_future_option_possible_order(
        &self,
        request: DomesticFuturePossibleOrderRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>> {
        let response = call_get_api::<RawApiBody>(
            &self.oauth,
            &self.header,
            self.practice,
            DOMESTIC_FUTURES_PSBL_ORDER_ENDPOINT,
            &[
                ("CANO", &request.account.cano),
                ("ACNT_PRDT_CD", &request.account.acnt_prdt_cd),
                ("PDNO", request.pdno),
                ("SLL_BUY_DVSN_CD", request.sll_buy_dvsn_cd),
                ("UNIT_PRICE", request.unit_price),
                ("ORD_DVSN_CD", request.ord_dvsn_cd),
            ],
        )
        .await?;
        response.body.ensure_success()?;
        Ok(response)
    }
}
