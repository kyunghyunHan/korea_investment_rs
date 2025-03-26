// use crate::websocket::oauth::ApproveOauth;
use crate::types::CustType;
#[cfg(feature = "ex")]
use dotenv::dotenv;
use futures_util::{SinkExt, stream::StreamExt};
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use std::error::Error;
use std::fmt;
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
#[derive(Deserialize, Debug)]
struct TokenResponse {
    approval_key: String,
}
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverseasOrderNotificationData {
    /// 고객 ID
    pub cust_id: String,
    /// 계좌번호
    pub acnt_no: String,
    /// 주문번호
    pub oder_no: String,
    /// 원주문번호
    pub ooder_no: String,
    /// 매도매수구분 (01:매도 02:매수 03:전매도 04:환매수)
    pub seln_byov_cls: String,
    /// 정정구분 (0:정상 1:정정 2:취소)
    pub rctf_cls: String,
    /// 주문종류2 (1:시장가 2:지정자 6:단주시장가 7:단주지정가 A:MOO B:LOO C:MOC D:LOC)
    pub oder_kind2: String,
    /// 주식 단축 종목코드
    pub stck_shrn_iscd: String,
    /// 체결수량 (주문통보의 경우 주문수량, 체결통보인 경우 체결수량)
    pub cntg_qty: String,
    /// 체결단가 (주문통보 시 주문단가, 체결통보 시 체결단가)
    pub cntg_unpr: String,
    /// 주식 체결 시간
    pub stck_cntg_hour: String,
    /// 거부여부 (0:정상 1:거부)
    pub rfus_yn: String,
    /// 체결여부 (1:주문,정정,취소,거부 2:체결)
    pub cntg_yn: String,
    /// 접수여부 (1:주문접수 2:확인 3:취소(FOK/IOC))
    pub acpt_yn: String,
    /// 지점번호
    pub brnc_no: String,
    /// 주문 수량 (주문통보인 경우 미출력, 체결통보인 경우 주문수량 출력)
    pub oder_qty: String,
    /// 계좌명
    pub acnt_name: String,
    /// 체결종목명
    pub cntg_isnm: String,
    /// 해외종목구분 (4:홍콩(HKD) 5:상해B(USD) 6:NASDAQ 7:NYSE 8:AMEX 9:OTCB C:홍콩(CNY) A:상해A(CNY) B:심천B(HKD) D:도쿄 E:하노이 F:호치민)
    pub oder_cond: String,
    /// 담보유형코드 (0:현금 15:해외주식담보대출)
    pub debt_gb: String,
    /// 담보대출일자 (YYYYMMDD)
    pub debt_date: String,
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

pub trait RealtimeData: Sized {
    /// 구분자(^)로 나뉜 문자열에서 구조체 생성
    fn from_delimited_string(text: &str) -> Option<Self>;
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverseasQuoteData {
    /// 실시간종목코드
    pub rsym: String,
    /// 종목코드
    pub symb: String,
    /// 소수점자리수
    pub zdiv: String,
    /// 현지일자
    pub xymd: String,
    /// 현지시간
    pub xhms: String,
    /// 한국일자
    pub kymd: String,
    /// 한국시간
    pub khms: String,
    /// 매수총잔량
    pub bvol: String,
    /// 매도총잔량
    pub avol: String,
    /// 매수총잔량대비
    pub bdvl: String,
    /// 매도총잔량대비
    pub advl: String,
    /// 매수호가1
    pub pbid1: String,
    /// 매도호가1
    pub pask1: String,
    /// 매수잔량1
    pub vbid1: String,
    /// 매도잔량1
    pub vask1: String,
    /// 매수잔량대비1
    pub dbid1: String,
    /// 매도잔량대비1
    pub dask1: String,
}
/// 해외 실시간 데이터 구조체
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverseasRealtimeData {
    /// 실시간종목코드
    pub rsym: String,
    /// 종목코드
    pub symb: String,
    /// 수수점자리수
    pub zdiv: String,
    /// 현지영업일자 (8자리)
    pub tymd: String,
    /// 현지일자 (6자리)
    pub xymd: String,
    /// 현지시간 (6자리)
    pub xhms: String,
    /// 한국일자 (6자리)
    pub kymd: String,
    /// 한국시간 (6자리)
    pub khms: String,
    /// 시가
    pub open: String,
    /// 고가
    pub high: String,
    /// 저가
    pub low: String,
    /// 현재가
    pub last: String,
    /// 대비구분 (상승/하락 등)
    pub sign: String,
    /// 전일대비
    pub diff: String,
    /// 등락율
    pub rate: String,
    /// 매수호가
    pub pbid: String,
    /// 매도호가
    pub pask: String,
    /// 매수잔량
    pub vbid: String,
    /// 매도잔량
    pub vask: String,
    /// 체결량
    pub evol: String,
    /// 거래량
    pub tvol: String,
    /// 거래대금
    pub tamt: String,
    /// 매도체결량
    pub bivl: String,
    /// 매수체결량
    pub asvl: String,
    /// 체결강도
    pub strn: String,
    /// 시장구분 (1:장중, 2:장전, 3:장후)
    pub mtyp: String,
}

/// 해외 실시간 호가 데이터 구조체
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverseasOrderbookData {
    /// 실시간종목코드
    pub rsym: String,
    /// 종목코드
    pub symb: String,
    /// 소숫점자리수
    pub zdiv: String,
    /// 현지일자
    pub xymd: String,
    /// 현지시간
    pub xhms: String,
    /// 한국일자
    pub kymd: String,
    /// 한국시간
    pub khms: String,
    /// 매수총잔량
    pub bvol: String,
    /// 매도총잔량
    pub avol: String,
    /// 매수총잔량대비
    pub bdvl: String,
    /// 매도총잔량대비
    pub advl: String,

    // 1단계 호가 정보
    /// 매수호가1
    pub pbid1: String,
    /// 매도호가1
    pub pask1: String,
    /// 매수잔량1
    pub vbid1: String,
    /// 매도잔량1
    pub vask1: String,
    /// 매수잔량대비1
    pub dbid1: String,
    /// 매도잔량대비1
    pub dask1: String,

    // 2단계 호가 정보
    /// 매수호가2
    pub pbid2: String,
    /// 매도호가2
    pub pask2: String,
    /// 매수잔량2
    pub vbid2: String,
    /// 매도잔량2
    pub vask2: String,
    /// 매수잔량대비2
    pub dbid2: String,
    /// 매도잔량대비2
    pub dask2: String,

    // 3단계 호가 정보
    /// 매수호가3
    pub pbid3: String,
    /// 매도호가3
    pub pask3: String,
    /// 매수잔량3
    pub vbid3: String,
    /// 매도잔량3
    pub vask3: String,
    /// 매수잔량대비3
    pub dbid3: String,
    /// 매도잔량대비3
    pub dask3: String,

    // 4단계 호가 정보
    /// 매수호가4
    pub pbid4: String,
    /// 매도호가4
    pub pask4: String,
    /// 매수잔량4
    pub vbid4: String,
    /// 매도잔량4
    pub vask4: String,
    /// 매수잔량대비4
    pub dbid4: String,
    /// 매도잔량대비4
    pub dask4: String,

    // 5단계 호가 정보
    /// 매수호가5
    pub pbid5: String,
    /// 매도호가5
    pub pask5: String,
    /// 매수잔량5
    pub vbid5: String,
    /// 매도잔량5
    pub vask5: String,
    /// 매수잔량대비5
    pub dbid5: String,
    /// 매도잔량대비5
    pub dask5: String,

    // 6단계 호가 정보
    /// 매수호가6
    pub pbid6: String,
    /// 매도호가6
    pub pask6: String,
    /// 매수잔량6
    pub vbid6: String,
    /// 매도잔량6
    pub vask6: String,
    /// 매수잔량대비6
    pub dbid6: String,
    /// 매도잔량대비6
    pub dask6: String,

    // 7단계 호가 정보
    /// 매수호가7
    pub pbid7: String,
    /// 매도호가7
    pub pask7: String,
    /// 매수잔량7
    pub vbid7: String,
    /// 매도잔량7
    pub vask7: String,
    /// 매수잔량대비7
    pub dbid7: String,
    /// 매도잔량대비7
    pub dask7: String,

    // 8단계 호가 정보
    /// 매수호가8
    pub pbid8: String,
    /// 매도호가8
    pub pask8: String,
    /// 매수잔량8
    pub vbid8: String,
    /// 매도잔량8
    pub vask8: String,
    /// 매수잔량대비8
    pub dbid8: String,
    /// 매도잔량대비8
    pub dask8: String,

    // 9단계 호가 정보
    /// 매수호가9
    pub pbid9: String,
    /// 매도호가9
    pub pask9: String,
    /// 매수잔량9
    pub vbid9: String,
    /// 매도잔량9
    pub vask9: String,
    /// 매수잔량대비9
    pub dbid9: String,
    /// 매도잔량대비9
    pub dask9: String,

    // 10단계 호가 정보
    /// 매수호가10
    pub pbid10: String,
    /// 매도호가10
    pub pask10: String,
    /// 매수잔량10
    pub vbid10: String,
    /// 매도잔량10
    pub vask10: String,
    /// 매수잔량대비10
    pub dbid10: String,
    /// 매도잔량대비10
    pub dask10: String,
}
impl RealtimeData for OverseasRealtimeData {
    fn from_delimited_string(text: &str) -> Option<Self> {
        let fields: Vec<&str> = text.split('^').collect();

        if fields.len() < 26 {
            return None;
        }

        Some(Self {
            rsym: fields[0].to_string(),
            symb: fields[1].to_string(),
            zdiv: fields[2].to_string(),
            tymd: fields[3].to_string(),
            xymd: fields[4].to_string(),
            xhms: fields[5].to_string(),
            kymd: fields[6].to_string(),
            khms: fields[7].to_string(),
            open: fields[8].to_string(),
            high: fields[9].to_string(),
            low: fields[10].to_string(),
            last: fields[11].to_string(),
            sign: fields[12].to_string(),
            diff: fields[13].to_string(),
            rate: fields[14].to_string(),
            pbid: fields[15].to_string(),
            pask: fields[16].to_string(),
            vbid: fields[17].to_string(),
            vask: fields[18].to_string(),
            evol: fields[19].to_string(),
            tvol: fields[20].to_string(),
            tamt: fields[21].to_string(),
            bivl: fields[22].to_string(),
            asvl: fields[23].to_string(),
            strn: fields[24].to_string(),
            mtyp: fields[25].to_string(),
        })
    }
}
// impl OverseasRealtimeData {
//     /// 구분자(^)로 나뉜 문자열에서 구조체 생성
//     pub fn from_delimited_string(text: &str) -> Option<Self> {
//         let fields: Vec<&str> = text.split('^').collect();

//         if fields.len() < 26 {
//             return None;
//         }

//         Some(Self {
//             rsym: fields[0].to_string(),
//             symb: fields[1].to_string(),
//             zdiv: fields[2].to_string(),
//             tymd: fields[3].to_string(),
//             xymd: fields[4].to_string(),
//             xhms: fields[5].to_string(),
//             kymd: fields[6].to_string(),
//             khms: fields[7].to_string(),
//             open: fields[8].to_string(),
//             high: fields[9].to_string(),
//             low: fields[10].to_string(),
//             last: fields[11].to_string(),
//             sign: fields[12].to_string(),
//             diff: fields[13].to_string(),
//             rate: fields[14].to_string(),
//             pbid: fields[15].to_string(),
//             pask: fields[16].to_string(),
//             vbid: fields[17].to_string(),
//             vask: fields[18].to_string(),
//             evol: fields[19].to_string(),
//             tvol: fields[20].to_string(),
//             tamt: fields[21].to_string(),
//             bivl: fields[22].to_string(),
//             asvl: fields[23].to_string(),
//             strn: fields[24].to_string(),
//             mtyp: fields[25].to_string(),
//         })
//     }
// }

/// 해외 실시간 데이터 관련 오류
#[derive(Debug)]
pub enum OverseasRealtimeError {
    ConnectionError(String),
    AuthError(String),
    MessageError(String),
    EnvError(String),
}

impl Error for OverseasRealtimeError {}

impl fmt::Display for OverseasRealtimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OverseasRealtimeError::ConnectionError(msg) => write!(f, "연결 오류: {}", msg),
            OverseasRealtimeError::AuthError(msg) => write!(f, "인증 오류: {}", msg),
            OverseasRealtimeError::MessageError(msg) => write!(f, "메시지 오류: {}", msg),
            OverseasRealtimeError::EnvError(msg) => write!(f, "환경 변수 오류: {}", msg),
        }
    }
}
impl From<reqwest::Error> for OverseasRealtimeError {
    fn from(error: reqwest::Error) -> Self {
        OverseasRealtimeError::ConnectionError(error.to_string())
    }
}
/// 해외 실시간 데이터 클라이언트
pub struct OverseasRealtimeClient {
    app_key: String,
    app_secret: String,
    approval_key: String,
    cust_type: CustType,
}

impl OverseasRealtimeClient {
    /// 새로운 클라이언트 생성
    pub async fn new(
        app_key: String,
        app_secret: String,
        cust_type: CustType,
    ) -> Result<Self, Box<dyn Error>> {
        let client = reqwest::Client::new();

        let url = "https://openapi.koreainvestment.com:9443/oauth2/Approval";

        let body = json!({
            "grant_type": "client_credentials",
            "appkey": app_key,
            "secretkey": app_secret
        });

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let response = client.post(url).headers(headers).json(&body).send().await?;

        let approval_response: TokenResponse = response.json().await?;
        Ok(Self {
            app_key,
            app_secret,
            approval_key: (approval_response.approval_key),
            cust_type,
        })
    }

    /// 환경 변수에서 클라이언트 생성
    pub async fn from_env(cust_type: CustType) -> Result<Self, OverseasRealtimeError> {
        #[cfg(feature = "ex")]
        dotenv().ok();

        let app_key = env::var("PUB_KEY").map_err(|_| {
            OverseasRealtimeError::EnvError("APP_KEY not set in .env file".to_string())
        })?;
        let app_secret = env::var("SCREST_KEY").map_err(|_| {
            OverseasRealtimeError::EnvError("APP_SECRET not set in .env file".to_string())
        })?;
        let client = reqwest::Client::new();

        let url = "https://openapi.koreainvestment.com:9443/oauth2/Approval";

        let body = json!({
            "grant_type": "client_credentials",
            "appkey": app_key,
            "secretkey": app_secret
        });

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let response = client.post(url).headers(headers).json(&body).send().await?;

        let approval_response: TokenResponse = response.json().await?;
        Ok(Self {
            app_key,
            app_secret,
            approval_key: (approval_response.approval_key),
            cust_type,
        })
    }

    /// 해외 실시간 데이터 스트림 시작
    pub async fn start_stream<T: RealtimeData + Send + 'static>(
        &self,
        symbol: &str,
        // r#type: OverseasRealtimeInfoType,
        mut callback: impl FnMut(T) + Send + 'static,
    ) -> Result<StreamController, OverseasRealtimeError> {
        let oauth = self;

        // WebSocket URL
        let url = "ws://ops.koreainvestment.com:21000/tryitout/HDFSCNT0";

        // WebSocket 연결
        let (ws_stream, _) = connect_async(url)
            .await
            .map_err(|e| OverseasRealtimeError::ConnectionError(e.to_string()))?;

        let (mut write, mut read) = ws_stream.split();

        // 접속 키 (API 승인 요청 후 받은 approval_key 사용)
        let approval_key = &oauth.approval_key;

        // WebSocket 요청 데이터
        let request_data = json!({
            "header": {
                "approval_key": approval_key,
                "custtype": self.cust_type,    // P: 개인, B: 법인
                "tr_type": "1",     // 1: 등록, 2: 해제
                "content-type": "utf-8"
            },
            "body": {
                "input": {
                    "tr_id": "HDFSCNT0",
                    "tr_key": symbol  // 예: "DNASAAPL" - 나스닥 애플 종목
                }
            }
        });

        // JSON 메시지를 WebSocket으로 전송
        write
            .send(Message::Text(request_data.to_string().into()))
            .await
            .map_err(|e| OverseasRealtimeError::MessageError(e.to_string()))?;

        // 채널 생성
        let (tx, mut rx) = mpsc::channel::<ControlMessage>(32);

        // 수신 작업 시작
        tokio::spawn(async move {
            let mut is_running = true;

            while let Some(message) = read.next().await {
                // 제어 메시지 확인
                if let Ok(control_msg) = rx.try_recv() {
                    match control_msg {
                        ControlMessage::Stop => {
                            is_running = false;
                            break;
                        }
                    }
                }

                if !is_running {
                    break;
                }
                println!("{:?}",message);
                match message {
                    Ok(Message::Text(text)) => {
                        if text.starts_with('{') {
                            // JSON 응답 로깅 또는 처리
                            println!("수신된 JSON: {}", text);
                            // 여기서 JSON 파싱 및 처리 로직 추가
                        } else {
                            // 구분자(^)로 나뉜 실시간 데이터 파싱
                            if let Some(data) = T::from_delimited_string(&text) {
                                callback(data);
                            }
                        }
                    }
                    Ok(_) => {}
                    Err(_) => {
                        break;
                    }
                }
            }
        });

        Ok(StreamController { tx })
    }

    /// 해외 실시간 데이터 스트림 시작 (채널 반환)
    pub async fn start_stream_channel(
        &self,
        symbol: &str,
        // r#type: OverseasRealtimeInfoType,
    ) -> Result<(mpsc::Receiver<OverseasRealtimeData>, StreamController), OverseasRealtimeError>
    {
        let (data_tx, data_rx) = mpsc::channel::<OverseasRealtimeData>(100);

        let controller = self
            .start_stream(symbol, move |data| {
                let _ = data_tx.try_send(data);
            })
            .await?;

        Ok((data_rx, controller))
    }
}

/// 스트림 제어 메시지
enum ControlMessage {
    Stop,
}

/// 스트림 컨트롤러
pub struct StreamController {
    tx: mpsc::Sender<ControlMessage>,
}

impl StreamController {
    /// 스트림 중지
    pub async fn stop(&self) -> Result<(), OverseasRealtimeError> {
        self.tx
            .send(ControlMessage::Stop)
            .await
            .map_err(|e| OverseasRealtimeError::MessageError(e.to_string()))
    }
}
