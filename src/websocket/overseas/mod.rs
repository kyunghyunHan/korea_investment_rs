use crate::types::CustType;
#[cfg(feature = "ex")]
use dotenv::dotenv;
pub mod models;
pub mod types;
use futures_util::{SinkExt, stream::StreamExt};
use models::{
    OverseasOrderNotificationData, OverseasOrderbookData, OverseasQuoteData, OverseasRealtimeData,
    RealtimeData,
};
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue};
use serde::Deserialize;
use serde_json::json;
use std::env;
use std::error::Error;
use std::fmt;
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use types::OverseasRealtimeInfoType;
/// 해외 실시간 데이터 관련 오류
#[derive(Debug)]
pub enum OverseasRealtimeError {
    ConnectionError(String),
    AuthError(String),
    MessageError(String),
    EnvError(String),
}
#[derive(Deserialize, Debug)]
struct TokenResponse {
    approval_key: String,
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
    async fn start_stream<T: RealtimeData + Send + 'static>(
        &self,
        symbol: &str,
        r#type: OverseasRealtimeInfoType,
        mut callback: impl FnMut(T) + Send + 'static,
    ) -> Result<StreamController, OverseasRealtimeError> {
        let oauth = self;

        // WebSocket URL - 타입에 따른 TR 코드 사용
        let tr_code = r#type.get_tr_code();
        let url = format!("ws://ops.koreainvestment.com:21000/tryitout/{}", tr_code);

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
                    "tr_id": tr_code,
                    "tr_key": symbol
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

                match message {
                    Ok(Message::Text(text)) => {
                        // JSON 형식인지 확인
                        if text.starts_with('{') {
                            println!("수신된 JSON: {}", text);
                            // 여기서 구독 확인 또는 오류 처리 가능
                        } else {
                            // 구분자(^)로 나뉜 실시간 데이터 파싱
                            if let Some(data) = T::from_delimited_string(&text) {
                                callback(data);
                            } else {
                                println!("데이터 파싱 실패: {}", text);
                            }
                        }
                    }
                    Ok(other) => {
                        println!("WebSocket에서 텍스트가 아닌 메시지 수신: {:?}", other);
                    }
                    Err(e) => {
                        println!("WebSocket 에러: {:?}", e);
                        break;
                    }
                }
            }
            println!("WebSocket 연결 종료");
        });

        Ok(StreamController { tx })
    }
    /// 해외 실시간 데이터 스트림 시작 (채널 반환) - 제네릭 버전
    pub async fn start_stream_channel<T: RealtimeData + Send + 'static>(
        &self,
        symbol: &str,
        r#type: OverseasRealtimeInfoType,
    ) -> Result<(mpsc::Receiver<T>, StreamController), OverseasRealtimeError> {
        let (data_tx, data_rx) = mpsc::channel::<T>(100);

        let controller = self
            .start_stream(symbol, r#type, move |data: T| {
                let _ = data_tx.try_send(data);
            })
            .await?;

        Ok((data_rx, controller))
    }

    // 편의 메서드들 - 타입별로 특화된 스트림 시작 함수들
    //1) Overseas Stock Real-Time Delayed Transaction Price [Real-Time-007]
    // 실시간 시세 데이터용
    pub async fn start_delayed_transaction_price(
        &self,
        symbol: &str,
        callback: impl FnMut(OverseasRealtimeData) + Send + 'static,
    ) -> Result<StreamController, OverseasRealtimeError> {
        self.start_stream(
            symbol,
            OverseasRealtimeInfoType::DelayedTradePrice,
            callback,
        )
        .await
    }

    // 2) Overseas Stock Real-Time Delayed Quotes (Asia) [Real-Time-008]
    pub async fn start_delayed_quotes(
        &self,
        symbol: &str,
        callback: impl FnMut(OverseasOrderbookData) + Send + 'static,
    ) -> Result<StreamController, OverseasRealtimeError> {
        self.start_stream(symbol, OverseasRealtimeInfoType::DelayedQuoteAsia, callback)
            .await
    }

    //3) Overseas Stock Real-Time Transaction Notification [Real-Time-009]
    pub async fn start_transaction_notification(
        &self,
        symbol: &str,
        callback: impl FnMut(OverseasOrderNotificationData) + Send + 'static,
    ) -> Result<StreamController, OverseasRealtimeError> {
        self.start_stream(
            symbol,
            OverseasRealtimeInfoType::TradeNotification,
            callback,
        )
        .await
    }

    //4) Overseas Stock Real-Time Quotes (U.S.) [Real-Time-021]
    pub async fn start_quote_usa(
        &self,
        symbol: &str,
        callback: impl FnMut(OverseasOrderNotificationData) + Send + 'static,
    ) -> Result<StreamController, OverseasRealtimeError> {
        self.start_stream(symbol, OverseasRealtimeInfoType::QuoteUSA, callback)
            .await
    }

    /*channel */
    //1) Overseas Stock Real-Time Delayed Transaction Price [Real-Time-007]
    //실시간 지연 체결가
    pub async fn start_delayed_transaction_price_channel(
        &self,
        symbol: &str,
    ) -> Result<(mpsc::Receiver<OverseasRealtimeData>, StreamController), OverseasRealtimeError>
    {
        self.start_stream_channel::<OverseasRealtimeData>(
            symbol,
            OverseasRealtimeInfoType::DelayedTradePrice,
        )
        .await
    }
    //실시간 지연 호가
    pub async fn start_delayed_quotes_channel(
        &self,
        symbol: &str,
    ) -> Result<(mpsc::Receiver<OverseasRealtimeData>, StreamController), OverseasRealtimeError>
    {
        self.start_stream_channel::<OverseasRealtimeData>(
            symbol,
            OverseasRealtimeInfoType::DelayedQuoteAsia,
        )
        .await
    }
    //실시간 체결 통보
    pub async fn start_transaction_notification_channel(
        &self,
        symbol: &str,
    ) -> Result<(mpsc::Receiver<OverseasRealtimeData>, StreamController), OverseasRealtimeError>
    {
        self.start_stream_channel::<OverseasRealtimeData>(
            symbol,
            OverseasRealtimeInfoType::TradeNotification,
        )
        .await
    }
    //실시간 호가 미국
    pub async fn start_quote_channel(
        &self,
        symbol: &str,
    ) -> Result<(mpsc::Receiver<OverseasRealtimeData>, StreamController), OverseasRealtimeError>
    {
        self.start_stream_channel::<OverseasRealtimeData>(
            symbol,
            OverseasRealtimeInfoType::QuoteUSA,
        )
        .await
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
