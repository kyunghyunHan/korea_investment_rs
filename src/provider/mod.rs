use crate::domestic;
use crate::oauth::Oauth;
use crate::types::{CustType, MarketType};
use crate::utils::{ApiHeader, call_api};
use std::error::Error;
/// 국내 / 해외 시장 구분

#[derive(Debug)]
/// 공통 Provider
pub struct KISProvider {
    pub oauth: Oauth,
    pub header: ApiHeader<'static>, // 해외도 같은 ApiHeader 씀
    pub practice: bool,
    pub market: MarketType,
}

impl KISProvider {
    /// Provider 생성 (국내 or 해외)
    pub async fn new(market: MarketType, practice: bool) -> Result<Self, Box<dyn Error>> {
        let oauth = Oauth::from_env_with_cache(CustType::P, practice).await?;
        let header = ApiHeader::personal();
        Ok(Self {
            oauth,
            header,
            practice,
            market,
        })
    }
    
}
