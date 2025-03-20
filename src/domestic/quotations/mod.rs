use crate::oauth::Oauth;
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue};
use serde::{self, Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Custtype {
    B, //법인
    P, //개인
}
/*Header
주식 현재가
GET
*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiHeader<'a> {
    /// OAuth 토큰이 필요한 API의 경우 발급한 Access token
    /// 일반고객: 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차 준용
    /// 법인: 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차 준용

    /// [법인 필수] 제휴사 회원 관리를 위한 고객식별키
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personalseckey: Option<&'a str>,

    /// 거래ID (예: 'FHKST01010100')

    /// 연속 거래 여부
    /// - 공백: 초기 조회
    /// - N: 다음 데이터 조회 (output header의 tr_cont가 M일 경우)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tr_cont: Option<&'a str>,

    /// 고객 타입
    /// - B: 법인
    /// - P: 개인
    pub custtype: Custtype,

    /// [법인 필수] 일련번호 (001)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seq_no: Option<&'a str>,

    /// 법인고객 혹은 개인고객의 Mac address 값
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<&'a str>,

    /// [법인 필수] 제휴사APP을 사용하는 경우 사용자(회원) 핸드폰번호
    /// ex) 01011112222 (하이픈 등 구분값 제거)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<&'a str>,

    /// [법인 필수] 사용자(회원)의 IP Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_addr: Option<&'a str>,

    /// [POST API 대상] Client가 요청하는 Request Body를 hashkey api로 생성한 Hash값
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hashkey: Option<&'a str>,

    /// [법인 필수] 거래고유번호로 사용하므로 거래별로 UNIQUE해야 함
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gt_uid: Option<&'a str>,
}
impl<'a> ApiHeader<'a> {
    pub fn new(
        custtype: Custtype,
        personalseckey: Option<&'a str>,
        seq_no: Option<&'a str>,
        phone_number: Option<&'a str>,
        ip_addr: Option<&'a str>,
        gt_uid: Option<&'a str>,
    ) -> Result<Self, &'static str> {
        // 법인인 경우 필수 필드 검증
        if custtype == Custtype::B {
            if personalseckey.is_none() {
                return Err("법인 사용자는 personalseckey가 필요합니다");
            }
            if seq_no.is_none() {
                return Err("법인 사용자는 seq_no가 필요합니다");
            }
            if phone_number.is_none() {
                return Err("법인 사용자는 phone_number가 필요합니다");
            }
            if ip_addr.is_none() {
                return Err("법인 사용자는 ip_addr이 필요합니다");
            }
            if gt_uid.is_none() {
                return Err("법인 사용자는 gt_uid가 필요합니다");
            }
        }

        Ok(Self {
            personalseckey,
            tr_cont: None,
            custtype,
            seq_no,
            mac_address: None,
            phone_number,
            ip_addr,
            hashkey: None,
            gt_uid,
        })
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryParam<'a> {
    /// 조건 시장 분류 코드
    /// - J: KRX
    /// - NX: NXT
    /// - UN: 통합
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub market_division_code: &'a str,

    /// 입력 종목코드 (예: 005930 삼성전자)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub stock_code: &'a str,
}
impl<'a> QueryParam<'a> {
    pub fn new(market_division_code: &'a str, stock_code: &'a str) -> Self {
        Self {
            market_division_code,
            stock_code,
        }
    }
}
pub async fn get_stock_price(
    oauth: Oauth,
    header: ApiHeader<'_>,
    query: QueryParam<'_>,
) -> Result<serde_json::Value, Box<dyn Error>> {
    let client = reqwest::Client::new();

    let url =
        "https://openapi.koreainvestment.com:9443/uapi/domestic-stock/v1/quotations/inquire-price";

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(
        "authorization",
        HeaderValue::from_str(&format!("Bearer {}", oauth.token)).unwrap(),
    );
    headers.insert("appkey", HeaderValue::from_str(&oauth.app_key).unwrap());
    if let Some(personalseckey) = header.personalseckey {
        headers.insert(
            "personalseckey",
            HeaderValue::from_str(personalseckey).unwrap(),
        );
    }
    headers.insert(
        "appsecret",
        HeaderValue::from_str(&oauth.app_secret).unwrap(),
    );
    headers.insert("tr_id", HeaderValue::from_static("FHKST01010100")); // 주식 현재가 시세 조회

    let response = client
        .get(url)
        .headers(headers)
        .query(&[
            ("FID_COND_MRKT_DIV_CODE", &query.market_division_code), // 주식 시장 구분 코드 (J:주식)
            ("FID_INPUT_ISCD", &query.stock_code),                   // 삼성전자 종목코드
        ])
        .send()
        .await
        .unwrap();

    let response_json: serde_json::Value = response.json().await.unwrap();
    println!("{}", response_json);
    Ok(response_json)
}
