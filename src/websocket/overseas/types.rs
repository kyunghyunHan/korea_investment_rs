/// 해외주식 실시간시세 관련 정보 유형
#[derive(Debug, Clone, PartialEq)]
pub enum OverseasRealtimeInfoType {
    /// 해외주식 실시간시세
    RealTimeQuote,
    /// 해외주식 실시간지수체결가
    IndexPrice,
    /// 해외주식 실시간지연호가(아시아)
    DelayedQuoteAsia,
    /// 해외주식 실시간체결통보
    TradeNotification,
    /// 해외주식 실시간호가(미국)
    QuoteUSA,
}
impl OverseasRealtimeInfoType {
    /// 메뉴 이름 반환
    pub fn get_name(&self) -> &'static str {
        match self {
            Self::RealTimeQuote => "해외주식 실시간시세",
            Self::IndexPrice => "해외주식 실시간지수체결가",
            Self::DelayedQuoteAsia => "해외주식 실시간지연호가(아시아)",
            Self::TradeNotification => "해외주식 실시간체결통보",
            Self::QuoteUSA => "해외주식 실시간호가(미국)",
        }
    }

    /// TR 코드 반환 (필요한 경우)
    pub fn get_tr_code(&self) -> &'static str {
        match self {
            Self::RealTimeQuote => "HDFSCNT0",
            Self::IndexPrice => "HDFSCNI0",
            Self::DelayedQuoteAsia => "HDFSADAQ",
            Self::TradeNotification => "HDFSCNST",
            Self::QuoteUSA => "HDFSUSAQ",
        }
    }
}
