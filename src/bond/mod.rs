use crate::provider::KISProvider;
use crate::types::{AccountInfo, ContinuationKey};
use crate::utils::{ApiEndpoint, ApiResponse, RawApiBody, TrId, call_get_api};
use async_trait::async_trait;
use std::error::Error;

const BOND_BALANCE_ENDPOINT: ApiEndpoint = ApiEndpoint::real_only(
    "/uapi/domestic-bond/v1/trading/inquire-balance",
    TrId::new("CTSC8407R", None),
);
const BOND_PRICE_ENDPOINT: ApiEndpoint = ApiEndpoint::real_only(
    "/uapi/domestic-bond/v1/quotations/inquire-price",
    TrId::new("FHKBJ773400C0", None),
);

#[derive(Debug, Clone)]
pub struct BondBalanceRequest<'a> {
    pub account: &'a AccountInfo,
    pub inqr_cndt: &'a str,
    pub pdno: &'a str,
    pub buy_dt: &'a str,
    pub continuation: Option<&'a ContinuationKey>,
}

#[async_trait]
pub trait DomesticBondApi {
    async fn inquire_bond_balance(
        &self,
        request: BondBalanceRequest<'_>,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>>;
    async fn get_bond_price(
        &self,
        bond_code: &str,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>>;
}

#[async_trait]
impl DomesticBondApi for KISProvider {
    async fn inquire_bond_balance(
        &self,
        request: BondBalanceRequest<'_>,
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
            BOND_BALANCE_ENDPOINT,
            &[
                ("CANO", &request.account.cano),
                ("ACNT_PRDT_CD", &request.account.acnt_prdt_cd),
                ("INQR_CNDT", request.inqr_cndt),
                ("PDNO", request.pdno),
                ("BUY_DT", request.buy_dt),
                ("CTX_AREA_FK200", &continuation.fk),
                ("CTX_AREA_NK200", &continuation.nk),
            ],
        )
        .await?;
        response.body.ensure_success()?;
        Ok(response)
    }

    async fn get_bond_price(
        &self,
        bond_code: &str,
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>> {
        let response = call_get_api::<RawApiBody>(
            &self.oauth,
            &self.header,
            self.practice,
            BOND_PRICE_ENDPOINT,
            &[("FID_COND_MRKT_DIV_CODE", "B"), ("FID_INPUT_ISCD", bond_code)],
        )
        .await?;
        response.body.ensure_success()?;
        Ok(response)
    }
}
