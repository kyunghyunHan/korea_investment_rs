use crate::provider::KISProvider;
use crate::utils::{ApiEndpoint, ApiResponse, RawApiBody, TrId, call_get_api};
use async_trait::async_trait;
use std::error::Error;

#[derive(Debug, Clone, Copy)]
pub enum DomesticAnalysisEndpoint {
    ProgramTradeToday,
    DailyCreditBalance,
    InvestorDailyByMarket,
    DailyShortSale,
    InvestorTradeByStockDaily,
    CaptureUpLowPrice,
    ProgramTradeDaily,
    DailyLoanTransaction,
    PriceBarTradeRatio,
    ForeignInstitutionTotal,
    MemberDaily,
    ProgramTradeByStockDaily,
    InvestorTrendEstimate,
    DailyTradeVolume,
    TradeProportionByAmount,
    InvestorProgramTradeToday,
    MarketFunds,
    ExpectedPriceTrend,
    ForeignMemberTradeTrend,
    InvestorTimeByMarket,
    ProgramTradeByStockRealtime,
    ConditionSearchTitles,
    ConditionSearchResult,
    InterestGroupList,
    InterestStocksByGroup,
    StockSearchInfo,
    ProductSearchInfo,
    FinancialRatio,
    ProfitRatio,
    StabilityRatio,
    GrowthRatio,
    BalanceSheet,
    IncomeStatement,
    EstimatePerformance,
    InvestOpinion,
    InvestOpinionBySec,
}

impl DomesticAnalysisEndpoint {
    fn endpoint(self) -> ApiEndpoint {
        match self {
            Self::ProgramTradeToday => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/quotations/comp-program-trade-today",
                TrId::new("FHPPG04600101", None),
            ),
            Self::DailyCreditBalance => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/quotations/daily-credit-balance",
                TrId::new("FHPST04760000", None),
            ),
            Self::InvestorDailyByMarket => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/quotations/inquire-investor-daily-by-market",
                TrId::new("FHPTJ04040000", None),
            ),
            Self::DailyShortSale => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/quotations/daily-short-sale",
                TrId::new("FHPST04830000", None),
            ),
            Self::InvestorTradeByStockDaily => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/quotations/investor-trade-by-stock-daily",
                TrId::new("FHPTJ04160001", None),
            ),
            Self::CaptureUpLowPrice => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/quotations/capture-uplowprice",
                TrId::new("FHKST130000C0", None),
            ),
            Self::ProgramTradeDaily => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/quotations/comp-program-trade-daily",
                TrId::new("FHPPG04600001", None),
            ),
            Self::DailyLoanTransaction => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/quotations/daily-loan-trans",
                TrId::new("HHPST074500C0", None),
            ),
            Self::PriceBarTradeRatio => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/quotations/pbar-tratio",
                TrId::new("FHPST01130000", None),
            ),
            Self::ForeignInstitutionTotal => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/quotations/foreign-institution-total",
                TrId::new("FHPTJ04400000", None),
            ),
            Self::MemberDaily => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/quotations/inquire-member-daily",
                TrId::new("FHPST04540000", None),
            ),
            Self::ProgramTradeByStockDaily => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/quotations/program-trade-by-stock-daily",
                TrId::new("FHPPG04650201", None),
            ),
            Self::InvestorTrendEstimate => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/quotations/investor-trend-estimate",
                TrId::new("HHPTJ04160200", None),
            ),
            Self::DailyTradeVolume => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/quotations/inquire-daily-trade-volume",
                TrId::new("FHKST03010800", None),
            ),
            Self::TradeProportionByAmount => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/quotations/tradprt-byamt",
                TrId::new("FHKST111900C0", None),
            ),
            Self::InvestorProgramTradeToday => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/quotations/investor-program-trade-today",
                TrId::new("HHPPG046600C1", None),
            ),
            Self::MarketFunds => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/quotations/mktfunds",
                TrId::new("FHKST649100C0", None),
            ),
            Self::ExpectedPriceTrend => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/quotations/exp-price-trend",
                TrId::new("FHPST01810000", None),
            ),
            Self::ForeignMemberTradeTrend => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/quotations/frgnmem-trade-trend",
                TrId::new("FHPST04320000", None),
            ),
            Self::InvestorTimeByMarket => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/quotations/inquire-investor-time-by-market",
                TrId::new("FHPTJ04030000", None),
            ),
            Self::ProgramTradeByStockRealtime => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/quotations/program-trade-by-stock",
                TrId::new("FHPPG04650101", None),
            ),
            Self::ConditionSearchTitles => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/quotations/psearch-title",
                TrId::new("HHKST03900300", None),
            ),
            Self::ConditionSearchResult => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/quotations/psearch-result",
                TrId::new("HHKST03900400", None),
            ),
            Self::InterestGroupList => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/quotations/intstock-grouplist",
                TrId::new("HHKCM113004C7", None),
            ),
            Self::InterestStocksByGroup => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/quotations/intstock-stocklist-by-group",
                TrId::new("HHKCM113004C6", None),
            ),
            Self::StockSearchInfo => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/quotations/search-stock-info",
                TrId::new("CTPF1002R", None),
            ),
            Self::ProductSearchInfo => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/quotations/search-info",
                TrId::new("CTPF1604R", None),
            ),
            Self::FinancialRatio => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/finance/financial-ratio",
                TrId::new("FHKST66430300", None),
            ),
            Self::ProfitRatio => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/finance/profit-ratio",
                TrId::new("FHKST66430400", None),
            ),
            Self::StabilityRatio => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/finance/stability-ratio",
                TrId::new("FHKST66430600", None),
            ),
            Self::GrowthRatio => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/finance/growth-ratio",
                TrId::new("FHKST66430800", None),
            ),
            Self::BalanceSheet => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/finance/balance-sheet",
                TrId::new("FHKST66430100", None),
            ),
            Self::IncomeStatement => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/finance/income-statement",
                TrId::new("FHKST66430200", None),
            ),
            Self::EstimatePerformance => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/quotations/estimate-perform",
                TrId::new("HHKST668300C0", None),
            ),
            Self::InvestOpinion => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/quotations/invest-opinion",
                TrId::new("FHKST663300C0", None),
            ),
            Self::InvestOpinionBySec => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/quotations/invest-opbysec",
                TrId::new("FHKST663400C0", None),
            ),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DomesticRankingEndpoint {
    ExpectedUpDown,
    QuoteBalance,
    CreditBalance,
    OvertimeVolume,
    DividendRate,
    AfterHourBalance,
    ShortSale,
    Disparity,
    HtsTopView,
    VolumeRank,
    ProfitAssetIndex,
    NearNewHighLow,
    PreferDisparateRatio,
    BulkTransactionCount,
    FinanceRatio,
    MarketCap,
    TradedByCompany,
    Fluctuation,
    MarketValue,
    TopInterestStock,
    VolumePower,
    OvertimeFluctuation,
}

impl DomesticRankingEndpoint {
    fn endpoint(self) -> ApiEndpoint {
        match self {
            Self::ExpectedUpDown => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/ranking/exp-trans-updown",
                TrId::new("FHPST01820000", None),
            ),
            Self::QuoteBalance => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/ranking/quote-balance",
                TrId::new("FHPST01720000", None),
            ),
            Self::CreditBalance => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/ranking/credit-balance",
                TrId::new("FHKST17010000", None),
            ),
            Self::OvertimeVolume => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/ranking/overtime-volume",
                TrId::new("FHPST02350000", None),
            ),
            Self::DividendRate => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/ranking/dividend-rate",
                TrId::new("HHKDB13470100", None),
            ),
            Self::AfterHourBalance => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/ranking/after-hour-balance",
                TrId::new("FHPST01760000", None),
            ),
            Self::ShortSale => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/ranking/short-sale",
                TrId::new("FHPST04820000", None),
            ),
            Self::Disparity => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/ranking/disparity",
                TrId::new("FHPST01780000", None),
            ),
            Self::HtsTopView => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/ranking/hts-top-view",
                TrId::new("HHMCM000100C0", None),
            ),
            Self::VolumeRank => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/quotations/volume-rank",
                TrId::new("FHPST01710000", None),
            ),
            Self::ProfitAssetIndex => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/ranking/profit-asset-index",
                TrId::new("FHPST01730000", None),
            ),
            Self::NearNewHighLow => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/ranking/near-new-highlow",
                TrId::new("FHPST01870000", None),
            ),
            Self::PreferDisparateRatio => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/ranking/prefer-disparate-ratio",
                TrId::new("FHPST01770000", None),
            ),
            Self::BulkTransactionCount => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/ranking/bulk-trans-num",
                TrId::new("FHKST190900C0", None),
            ),
            Self::FinanceRatio => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/ranking/finance-ratio",
                TrId::new("FHPST01750000", None),
            ),
            Self::MarketCap => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/ranking/market-cap",
                TrId::new("FHPST01740000", None),
            ),
            Self::TradedByCompany => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/ranking/traded-by-company",
                TrId::new("FHPST01860000", None),
            ),
            Self::Fluctuation => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/ranking/fluctuation",
                TrId::new("FHPST01700000", None),
            ),
            Self::MarketValue => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/ranking/market-value",
                TrId::new("FHPST01790000", None),
            ),
            Self::TopInterestStock => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/ranking/top-interest-stock",
                TrId::new("FHPST01800000", None),
            ),
            Self::VolumePower => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/ranking/volume-power",
                TrId::new("FHPST01680000", None),
            ),
            Self::OvertimeFluctuation => ApiEndpoint::real_only(
                "/uapi/domestic-stock/v1/ranking/overtime-fluctuation",
                TrId::new("FHPST02340000", None),
            ),
        }
    }
}

#[async_trait]
pub trait DomesticAnalysis {
    async fn get_analysis_raw(
        &self,
        endpoint: DomesticAnalysisEndpoint,
        query: &[(&str, &str)],
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>>;
    async fn get_ranking_raw(
        &self,
        endpoint: DomesticRankingEndpoint,
        query: &[(&str, &str)],
    ) -> Result<ApiResponse<RawApiBody>, Box<dyn Error>>;
}

#[async_trait]
impl DomesticAnalysis for KISProvider {
    async fn get_analysis_raw(
        &self,
        endpoint: DomesticAnalysisEndpoint,
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

    async fn get_ranking_raw(
        &self,
        endpoint: DomesticRankingEndpoint,
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
