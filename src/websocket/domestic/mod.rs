use crate::types::CustType;
#[cfg(feature = "ex")]
use dotenv::dotenv;
use futures_util::{SinkExt, stream::StreamExt};
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue};
use serde::Deserialize;
use serde_json::json;
use std::env;
use std::error::Error;
use std::fmt;
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

pub struct DomesticRealtimeClient {
    approval_key: String,
    cust_type: CustType,
}

#[derive(Deserialize, Debug)]
struct TokenResponse {
    approval_key: String,
}

pub trait RealtimeData: Sized {
    fn from_delimited_string(text: &str) -> Option<Self>;
}

#[derive(Debug, Clone)]
pub struct RawDomesticRealtimeData {
    pub payload: String,
}

impl RealtimeData for RawDomesticRealtimeData {
    fn from_delimited_string(text: &str) -> Option<Self> {
        Some(Self {
            payload: text.to_string(),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DomesticRealtimeInfoType {
    StockTradeKrX,
    StockTradeUnified,
    StockTradeNxt,
    StockQuoteKrX,
    StockQuoteUnified,
    StockQuoteNxt,
    StockExpectedKrX,
    StockExpectedUnified,
    StockExpectedNxt,
    StockAfterHoursExpectedKrX,
    StockAfterHoursQuoteKrX,
    TradeNotification,
    ProgramTradeKrX,
    ProgramTradeUnified,
    ProgramTradeNxt,
    MemberKrX,
    MemberUnified,
    MemberNxt,
    MarketStatusKrX,
    MarketStatusUnified,
    MarketStatusNxt,
    IndexTrade,
    IndexExpected,
    IndexProgramTrade,
    EtfNav,
    ElwTrade,
    ElwQuote,
    ElwExpected,
}

impl DomesticRealtimeInfoType {
    pub fn get_name(&self) -> &'static str {
        match self {
            Self::StockTradeKrX => "국내주식 실시간체결가 (KRX)",
            Self::StockTradeUnified => "국내주식 실시간체결가 (통합)",
            Self::StockTradeNxt => "국내주식 실시간체결가 (NXT)",
            Self::StockQuoteKrX => "국내주식 실시간호가 (KRX)",
            Self::StockQuoteUnified => "국내주식 실시간호가 (통합)",
            Self::StockQuoteNxt => "국내주식 실시간호가 (NXT)",
            Self::StockExpectedKrX => "국내주식 실시간예상체결 (KRX)",
            Self::StockExpectedUnified => "국내주식 실시간예상체결 (통합)",
            Self::StockExpectedNxt => "국내주식 실시간예상체결 (NXT)",
            Self::StockAfterHoursExpectedKrX => "국내주식 시간외 실시간예상체결 (KRX)",
            Self::StockAfterHoursQuoteKrX => "국내주식 시간외 실시간호가 (KRX)",
            Self::TradeNotification => "국내주식 실시간체결통보",
            Self::ProgramTradeKrX => "국내주식 실시간프로그램매매 (KRX)",
            Self::ProgramTradeUnified => "국내주식 실시간프로그램매매 (통합)",
            Self::ProgramTradeNxt => "국내주식 실시간프로그램매매 (NXT)",
            Self::MemberKrX => "국내주식 실시간회원사 (KRX)",
            Self::MemberUnified => "국내주식 실시간회원사 (통합)",
            Self::MemberNxt => "국내주식 실시간회원사 (NXT)",
            Self::MarketStatusKrX => "국내주식 장운영정보 (KRX)",
            Self::MarketStatusUnified => "국내주식 장운영정보 (통합)",
            Self::MarketStatusNxt => "국내주식 장운영정보 (NXT)",
            Self::IndexTrade => "국내지수 실시간체결",
            Self::IndexExpected => "국내지수 실시간예상체결",
            Self::IndexProgramTrade => "국내지수 실시간프로그램매매",
            Self::EtfNav => "국내ETF NAV추이",
            Self::ElwTrade => "ELW 실시간체결가",
            Self::ElwQuote => "ELW 실시간호가",
            Self::ElwExpected => "ELW 실시간예상체결",
        }
    }

    pub fn get_tr_code(&self) -> &'static str {
        match self {
            Self::StockTradeKrX => "H0STCNT0",
            Self::StockTradeUnified => "H0UNCNT0",
            Self::StockTradeNxt => "H0NXCNT0",
            Self::StockQuoteKrX => "H0STASP0",
            Self::StockQuoteUnified => "H0UNASP0",
            Self::StockQuoteNxt => "H0NXASP0",
            Self::StockExpectedKrX => "H0STANC0",
            Self::StockExpectedUnified => "H0UNANC0",
            Self::StockExpectedNxt => "H0NXANC0",
            Self::StockAfterHoursExpectedKrX => "H0STOAC0",
            Self::StockAfterHoursQuoteKrX => "H0STOAA0",
            Self::TradeNotification => "H0STCNI0",
            Self::ProgramTradeKrX => "H0STPGM0",
            Self::ProgramTradeUnified => "H0UNPGM0",
            Self::ProgramTradeNxt => "H0NXPGM0",
            Self::MemberKrX => "H0STMBC0",
            Self::MemberUnified => "H0UNMBC0",
            Self::MemberNxt => "H0NXMBC0",
            Self::MarketStatusKrX => "H0STMKO0",
            Self::MarketStatusUnified => "H0UNMKO0",
            Self::MarketStatusNxt => "H0NXMKO0",
            Self::IndexTrade => "H0UPCNT0",
            Self::IndexExpected => "H0UPANC0",
            Self::IndexProgramTrade => "H0UPPGM0",
            Self::EtfNav => "H0STNAV0",
            Self::ElwTrade => "H0EWCNT0",
            Self::ElwQuote => "H0EWASP0",
            Self::ElwExpected => "H0EWANC0",
        }
    }
}

impl DomesticRealtimeClient {
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
            approval_key: approval_response.approval_key,
            cust_type,
        })
    }

    pub async fn from_env(cust_type: CustType) -> Result<Self, DomesticRealtimeError> {
        #[cfg(feature = "ex")]
        dotenv().ok();

        let app_key = env::var("PUB_KEY")
            .map_err(|_| DomesticRealtimeError::EnvError("APP_KEY not set in .env file".to_string()))?;
        let app_secret = env::var("SCREST_KEY").map_err(|_| {
            DomesticRealtimeError::EnvError("APP_SECRET not set in .env file".to_string())
        })?;
        Self::new(app_key, app_secret, cust_type)
            .await
            .map_err(|error| DomesticRealtimeError::AuthError(error.to_string()))
    }

    async fn start_stream<T: RealtimeData + Send + 'static>(
        &self,
        symbol: &str,
        r#type: DomesticRealtimeInfoType,
        mut callback: impl FnMut(T) + Send + 'static,
    ) -> Result<StreamController, DomesticRealtimeError> {
        let tr_code = r#type.get_tr_code();
        let url = format!("ws://ops.koreainvestment.com:21000/tryitout/{tr_code}");
        let (ws_stream, _) = connect_async(url)
            .await
            .map_err(|e| DomesticRealtimeError::ConnectionError(e.to_string()))?;

        let (mut write, mut read) = ws_stream.split();
        let request_data = json!({
            "header": {
                "approval_key": self.approval_key,
                "custtype": self.cust_type,
                "tr_type": "1",
                "content-type": "utf-8"
            },
            "body": {
                "input": {
                    "tr_id": tr_code,
                    "tr_key": symbol
                }
            }
        });

        write
            .send(Message::Text(request_data.to_string().into()))
            .await
            .map_err(|e| DomesticRealtimeError::MessageError(e.to_string()))?;

        let (tx, mut rx) = mpsc::channel::<ControlMessage>(32);
        tokio::spawn(async move {
            while let Some(message) = read.next().await {
                if let Ok(ControlMessage::Stop) = rx.try_recv() {
                    break;
                }

                match message {
                    Ok(Message::Text(text)) => {
                        if text.starts_with('{') {
                            println!("수신된 JSON: {}", text);
                        } else if let Some(data) = T::from_delimited_string(&text) {
                            callback(data);
                        } else {
                            println!("데이터 파싱 실패: {}", text);
                        }
                    }
                    Ok(other) => {
                        println!("WebSocket에서 텍스트가 아닌 메시지 수신: {:?}", other);
                    }
                    Err(error) => {
                        println!("WebSocket 에러: {:?}", error);
                        break;
                    }
                }
            }
            println!("WebSocket 연결 종료");
        });

        Ok(StreamController { tx })
    }

    pub async fn start_stream_channel<T: RealtimeData + Send + 'static>(
        &self,
        symbol: &str,
        r#type: DomesticRealtimeInfoType,
    ) -> Result<(mpsc::Receiver<T>, StreamController), DomesticRealtimeError> {
        let (data_tx, data_rx) = mpsc::channel::<T>(100);
        let controller = self
            .start_stream(symbol, r#type, move |data: T| {
                let _ = data_tx.try_send(data);
            })
            .await?;
        Ok((data_rx, controller))
    }

    pub async fn start_raw(
        &self,
        symbol: &str,
        r#type: DomesticRealtimeInfoType,
        callback: impl FnMut(RawDomesticRealtimeData) + Send + 'static,
    ) -> Result<StreamController, DomesticRealtimeError> {
        self.start_stream(symbol, r#type, callback).await
    }

    pub async fn start_raw_channel(
        &self,
        symbol: &str,
        r#type: DomesticRealtimeInfoType,
    ) -> Result<(mpsc::Receiver<RawDomesticRealtimeData>, StreamController), DomesticRealtimeError>
    {
        self.start_stream_channel::<RawDomesticRealtimeData>(symbol, r#type)
            .await
    }
}

enum ControlMessage {
    Stop,
}

pub struct StreamController {
    tx: mpsc::Sender<ControlMessage>,
}

impl StreamController {
    pub async fn stop(&self) -> Result<(), DomesticRealtimeError> {
        self.tx
            .send(ControlMessage::Stop)
            .await
            .map_err(|e| DomesticRealtimeError::MessageError(e.to_string()))
    }
}

#[derive(Debug)]
pub enum DomesticRealtimeError {
    ConnectionError(String),
    AuthError(String),
    MessageError(String),
    EnvError(String),
}

impl Error for DomesticRealtimeError {}

impl fmt::Display for DomesticRealtimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConnectionError(msg) => write!(f, "연결 오류: {}", msg),
            Self::AuthError(msg) => write!(f, "인증 오류: {}", msg),
            Self::MessageError(msg) => write!(f, "메시지 오류: {}", msg),
            Self::EnvError(msg) => write!(f, "환경 변수 오류: {}", msg),
        }
    }
}

impl From<reqwest::Error> for DomesticRealtimeError {
    fn from(error: reqwest::Error) -> Self {
        Self::ConnectionError(error.to_string())
    }
}
