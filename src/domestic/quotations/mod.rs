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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockPriceOutput {
    /// 종목 상태 구분 코드
    pub iscd_stat_cls_code: String,

    /// 증거금 비율
    pub marg_rate: String,

    /// 대표 시장 한글 명
    pub rprs_mrkt_kor_name: String,

    /// 신 고가 저가 구분 코드
    // pub new_hgpr_lwpr_cls_code: String,

    /// 업종 한글 종목명
    pub bstp_kor_isnm: String,

    /// 임시 정지 여부
    pub temp_stop_yn: String,

    /// 시가 범위 연장 여부
    pub oprc_rang_cont_yn: String,

    /// 종가 범위 연장 여부
    pub clpr_rang_cont_yn: String,

    /// 신용 가능 여부
    pub crdt_able_yn: String,

    /// 보증금 비율 구분 코드
    pub grmn_rate_cls_code: String,

    /// ELW 발행 여부
    pub elw_pblc_yn: String,

    /// 주식 현재가
    pub stck_prpr: String,

    /// 전일 대비
    pub prdy_vrss: String,

    /// 전일 대비 부호
    pub prdy_vrss_sign: String,

    /// 전일 대비율
    pub prdy_ctrt: String,

    /// 누적 거래 대금
    pub acml_tr_pbmn: String,

    /// 누적 거래량
    pub acml_vol: String,

    /// 전일 대비 거래량 비율
    pub prdy_vrss_vol_rate: String,

    /// 주식 시가
    pub stck_oprc: String,

    /// 주식 최고가
    pub stck_hgpr: String,

    /// 주식 최저가
    pub stck_lwpr: String,

    /// 주식 상한가
    pub stck_mxpr: String,

    /// 주식 하한가
    pub stck_llam: String,

    /// 주식 기준가
    pub stck_sdpr: String,

    /// 가중 평균 주식 가격
    pub wghn_avrg_stck_prc: String,

    /// HTS 외국인 소진율
    pub hts_frgn_ehrt: String,

    /// 외국인 순매수 수량
    pub frgn_ntby_qty: String,

    /// 프로그램매매 순매수 수량
    pub pgtr_ntby_qty: String,

    /// 피벗 2차 디저항 가격
    pub pvt_scnd_dmrs_prc: String,

    /// 피벗 1차 디저항 가격
    pub pvt_frst_dmrs_prc: String,

    /// 피벗 포인트 값
    pub pvt_pont_val: String,

    /// 피벗 1차 디지지 가격
    pub pvt_frst_dmsp_prc: String,

    /// 피벗 2차 디지지 가격
    pub pvt_scnd_dmsp_prc: String,

    /// 디저항 값
    pub dmrs_val: String,

    /// 디지지 값
    pub dmsp_val: String,

    /// 자본금
    pub cpfn: String,

    /// 제한 폭 가격
    pub rstc_wdth_prc: String,

    /// 주식 액면가
    pub stck_fcam: String,

    /// 주식 대용가
    pub stck_sspr: String,

    /// 호가단위
    pub aspr_unit: String,

    /// HTS 매매 수량 단위 값
    pub hts_deal_qty_unit_val: String,

    /// 상장 주수
    pub lstn_stcn: String,

    /// HTS 시가총액
    pub hts_avls: String,

    /// PER
    pub per: String,

    /// PBR
    pub pbr: String,

    /// 결산 월
    pub stac_month: String,

    /// 거래량 회전율
    pub vol_tnrt: String,

    /// EPS
    pub eps: String,

    /// BPS
    pub bps: String,

    /// 250일 최고가
    pub d250_hgpr: String,

    /// 250일 최고가 일자
    pub d250_hgpr_date: String,

    /// 250일 최고가 대비 현재가 비율
    pub d250_hgpr_vrss_prpr_rate: String,

    /// 250일 최저가
    pub d250_lwpr: String,

    /// 250일 최저가 일자
    pub d250_lwpr_date: String,

    /// 250일 최저가 대비 현재가 비율
    pub d250_lwpr_vrss_prpr_rate: String,

    /// 주식 연중 최고가
    pub stck_dryy_hgpr: String,

    /// 연중 최고가 대비 현재가 비율
    pub dryy_hgpr_vrss_prpr_rate: String,

    /// 연중 최고가 일자
    pub dryy_hgpr_date: String,

    /// 주식 연중 최저가
    pub stck_dryy_lwpr: String,

    /// 연중 최저가 대비 현재가 비율
    pub dryy_lwpr_vrss_prpr_rate: String,

    /// 연중 최저가 일자
    pub dryy_lwpr_date: String,

    /// 52주일 최고가
    pub w52_hgpr: String,

    /// 52주일 최고가 대비 현재가 대비
    pub w52_hgpr_vrss_prpr_ctrt: String,

    /// 52주일 최고가 일자
    pub w52_hgpr_date: String,

    /// 52주일 최저가
    pub w52_lwpr: String,

    /// 52주일 최저가 대비 현재가 대비
    pub w52_lwpr_vrss_prpr_ctrt: String,

    /// 52주일 최저가 일자
    pub w52_lwpr_date: String,

    /// 전체 융자 잔고 비율
    pub whol_loan_rmnd_rate: String,

    /// 공매도가능여부
    pub ssts_yn: String,

    /// 주식 단축 종목코드
    pub stck_shrn_iscd: String,

    /// 액면가 통화명
    pub fcam_cnnm: String,

    /// 자본금 통화명
    pub cpfn_cnnm: String,

    /// 접근도
    // pub apprch_rate: String,

    /// 외국인 보유 수량
    pub frgn_hldn_qty: String,

    /// VI적용구분코드
    pub vi_cls_code: String,

    /// 시간외단일가VI적용구분코드
    pub ovtm_vi_cls_code: String,

    /// 최종 공매도 체결 수량
    pub last_ssts_cntg_qty: String,

    /// 투자유의여부
    pub invt_caful_yn: String,

    /// 시장경고코드
    pub mrkt_warn_cls_code: String,

    /// 단기과열여부
    pub short_over_yn: String,

    /// 정리매매여부
    pub sltr_yn: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockPriceResponse {
    /// 성공 실패 여부
    pub rt_cd: String,
    /// 응답코드
    pub msg_cd: String,
    /// 응답메세지
    pub msg1: String,
    /// 응답상세
    pub output: StockPriceOutput,
}
/*주식 현제가 시세 */
pub async fn get_inquire_price(
    oauth: Oauth,
    header: ApiHeader<'_>,
    query: QueryParam<'_>,
) -> Result<StockPriceOutput, Box<dyn Error>> {
    let client = reqwest::Client::new();

    let url =
        "https://openapi.koreainvestment.com:9443/uapi/domestic-stock/v1/quotations/inquire-price";

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(
        "authorization",
        HeaderValue::from_str(&format!("Bearer {}", oauth.token))?,
    );
    headers.insert("appkey", HeaderValue::from_str(&oauth.app_key)?);
    if let Some(personalseckey) = header.personalseckey {
        headers.insert("personalseckey", HeaderValue::from_str(personalseckey)?);
    }
    headers.insert("appsecret", HeaderValue::from_str(&oauth.app_secret)?);
    headers.insert("tr_id", HeaderValue::from_static("FHKST01010100")); // 주식 현재가 시세 조회

    let response = client
        .get(url)
        .headers(headers)
        .query(&[
            ("FID_COND_MRKT_DIV_CODE", &query.market_division_code), // 주식 시장 구분 코드 (J:주식)
            ("FID_INPUT_ISCD", &query.stock_code),                   // 종목코드
        ])
        .send()
        .await?;

    // 응답 상태 확인
    let status = response.status();
    if !status.is_success() {
        let error_text = response.text().await?;
        return Err(format!("API 요청 실패 ({}): {}", status, error_text).into());
    }

    // 응답을 StockPriceResponse 구조체로 파싱
    let response_data: StockPriceResponse = response.json().await?;

    // 응답 코드 확인
    if response_data.rt_cd != "0" {
        return Err(format!(
            "API 응답 오류: {} (코드: {})",
            response_data.msg1, response_data.msg_cd
        )
        .into());
    }

    // 성공 시 output1 데이터 반환
    Ok(response_data.output)
}
