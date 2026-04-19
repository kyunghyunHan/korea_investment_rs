#[cfg(feature = "ex")]
use dotenv::dotenv;
use korea_investment_rs::{
    overseas::{OverseasPresentBalanceRequest, OverseasTrading},
    provider::KISProvider,
    types::{AccountInfo, MarketType},
};

#[tokio::main]
async fn main() {
    #[cfg(feature = "ex")]
    dotenv().ok();

    let provider = KISProvider::new(MarketType::Overseas, true)
        .await
        .expect("Provider 초기화 실패");
    let account = AccountInfo::from_env().expect("계좌 환경변수 로드 실패");

    let response = provider
        .inquire_overseas_present_balance(OverseasPresentBalanceRequest {
            account: &account,
            wcrc_frcr_dvsn_cd: "01",
            natn_cd: "840",
            tr_mket_cd: "00",
            inqr_dvsn_cd: "00",
        })
        .await
        .expect("해외 체결기준현재잔고 조회 실패");

    println!("body = {:#?}", response.body);
}
