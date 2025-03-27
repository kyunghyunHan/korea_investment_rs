/// 해외주식 실시간시세 관련 정보 유형
#[derive(Debug, Clone, PartialEq)]
pub enum OverseasRealtimeInfoType {
    /// 해외주식 실시간지연체결가[실시간-007]
    DelayedTradePrice,
    /// 해외주식 실시간지연호가(아시아)[실시간-008]
    DelayedQuoteAsia,
    /// 해외주식 실시간체결통보[실시간-009]
    TradeNotification,
    /// 해외주식 실시간호가(미국)[실시간-021]
    QuoteUSA,
}
impl OverseasRealtimeInfoType {
    /// 메뉴 이름 반환
    pub fn get_name(&self) -> &'static str {
        match self {
            Self::DelayedTradePrice => "해외주식 실시간지연체결가[실시간-007]",
            Self::DelayedQuoteAsia => "해외주식 실시간지연호가(아시아)[실시간-008]",
            Self::TradeNotification => "해외주식 실시간체결통보[실시간-009]",
            Self::QuoteUSA => "해외주식 실시간호가(미국)[실시간-021]",
        }
    }

    /// TR 코드 반환 (필요한 경우)
    pub fn get_tr_code(&self) -> &'static str {
        match self {
            Self::DelayedTradePrice => "HDFSCNT0",
            Self::DelayedQuoteAsia => "HDFSASP1",
            Self::TradeNotification => "H0GSCNI0",
            Self::QuoteUSA => "HDFSASP0",
        }
    }
}
