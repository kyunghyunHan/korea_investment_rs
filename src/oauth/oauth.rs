use crate::types::CustType;
#[cfg(feature = "ex")]
use dotenv::dotenv;
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue};
use serde::{self, Deserialize, Serialize};
use serde_json::json;
use std::env;

use std::error::Error;
#[derive(Deserialize, Debug)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: i32,
    access_token_token_expired: String, // 추가
}

#[derive(Debug)]
pub enum OauthType {
    PRACTICE,
    IMITATION,
}

#[derive(Debug, Serialize)]
pub struct Oauth {
    pub app_key: String,
    pub app_secret: String,
    pub token: String,
    pub cust_type: CustType,
    /// OAuth 토큰이 필요한 API의 경우 발급한 Access token
    /// 일반고객: 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차 준용
    /// 법인: 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차 준용

    /// [법인 필수] 제휴사 회원 관리를 위한 고객식별키
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personalseckey: Option<String>,

    /// 거래ID (예: 'FHKST01010100')

    /// 연속 거래 여부
    /// - 공백: 초기 조회
    /// - N: 다음 데이터 조회 (output header의 tr_cont가 M일 경우)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tr_cont: Option<String>,

    /// 고객 타입
    /// - B: 법인
    /// - P: 개인

    /// [법인 필수] 일련번호 (001)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seq_no: Option<String>,

    /// 법인고객 혹은 개인고객의 Mac address 값
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,

    /// [법인 필수] 제휴사APP을 사용하는 경우 사용자(회원) 핸드폰번호
    /// ex) 01011112222 (하이픈 등 구분값 제거)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,

    /// [법인 필수] 사용자(회원)의 IP Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_addr: Option<String>,

    /// [POST API 대상] Client가 요청하는 Request Body를 hashkey api로 생성한 Hash값
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hashkey: Option<String>,

    /// [법인 필수] 거래고유번호로 사용하므로 거래별로 UNIQUE해야 함
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gt_uid: Option<String>,
}

impl Oauth {
    /// 토큰 발급 (async)
    pub async fn request_token(
        app_key: String,
        app_secret: String,
        cust_type: CustType,
        personalseckey: Option<String>,
        seq_no: Option<String>,
        phone_number: Option<String>,
        ip_addr: Option<String>,
        gt_uid: Option<String>,
        practice: bool, // 모의/실전 선택
    ) -> Result<Self, Box<dyn Error>> {
        let client = reqwest::Client::new();

        let domain = if practice {
            "https://openapivts.koreainvestment.com:29443"
        } else {
            "https://openapi.koreainvestment.com:9443"
        };
        let url = format!("{}/oauth2/tokenP", domain);

        let body = json!({
            "grant_type": "client_credentials",
            "appkey": app_key,
            "appsecret": app_secret
        });

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json; charset=UTF-8"));

        let response = client.post(&url).headers(headers).json(&body).send().await?;

        let token_response: TokenResponse = response.json().await?;

        Ok(Self {
            app_key,
            app_secret,
            token: token_response.access_token,
            cust_type,
            personalseckey,
            tr_cont: None,
            seq_no,
            mac_address: None,
            phone_number,
            ip_addr,
            hashkey: None,
            gt_uid,
        })
    }

    /// 환경 변수에서 발급
    pub async fn from_env(cust_type: CustType, practice: bool) -> Result<Self, Box<dyn Error>> {
        #[cfg(feature = "ex")]
        dotenv().ok();

        let app_key = env::var("PUB_KEY").expect("PUB_KEY not set in .env file");
        let app_secret = env::var("SCREST_KEY").expect("SCREST_KEY not set in .env file");

        Self::request_token(
            app_key,
            app_secret,
            cust_type,
            None,
            None,
            None,
            None,
            None,
            practice,
        ).await
    }
}
