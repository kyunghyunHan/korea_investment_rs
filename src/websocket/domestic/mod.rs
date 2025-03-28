use crate::types::CustType;
#[cfg(feature = "ex")]
use dotenv::dotenv;

use futures_util::{SinkExt, stream::StreamExt};

use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue};
use serde::Deserialize;
use serde_json::json;
use std::env;
use std::error::Error;
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
/// 해외 실시간 데이터 클라이언트
pub struct DomesticRealtimeClient {
    app_key: String,
    app_secret: String,
    approval_key: String,
    cust_type: CustType,
}
#[derive(Deserialize, Debug)]
struct TokenResponse {
    approval_key: String,
}
pub trait RealtimeData: Sized {
    /// 구분자(^)로 나뉜 문자열에서 구조체 생성
    fn from_delimited_string(text: &str) -> Option<Self>;
}
/// 해외주식 실시간시세 관련 정보 유형
#[derive(Debug, Clone, PartialEq)]
pub enum DomesticRealtimeInfoType {
    /// 해외주식 실시간지연체결가[실시간-007]
    DelayedTradePrice,
    /// 해외주식 실시간지연호가(아시아)[실시간-008]
    DelayedQuoteAsia,
    /// 해외주식 실시간체결통보[실시간-009]
    TradeNotification,
    /// 해외주식 실시간호가(미국)[실시간-021]
    QuoteUSA,
}
impl DomesticRealtimeInfoType {
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

impl DomesticRealtimeClient {
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
    pub async fn from_env(cust_type: CustType) -> Result<Self, DomesticRealtimeError> {
        #[cfg(feature = "ex")]
        dotenv().ok();

        let app_key = env::var("PUB_KEY").map_err(|_| {
            DomesticRealtimeError::EnvError("APP_KEY not set in .env file".to_string())
        })?;
        let app_secret = env::var("SCREST_KEY").map_err(|_| {
            DomesticRealtimeError::EnvError("APP_SECRET not set in .env file".to_string())
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

    async fn start_stream<T: RealtimeData + Send + 'static>(
        &self,
        symbol: &str,
        r#type: DomesticRealtimeInfoType,
        mut callback: impl FnMut(T) + Send + 'static,
    ) -> Result<StreamController, DomesticRealtimeError> {
        let oauth = self;

        // WebSocket URL - 타입에 따른 TR 코드 사용
        let tr_code = r#type.get_tr_code();
        let url = format!("ws://ops.koreainvestment.com:21000/tryitout/{}", tr_code);

        // WebSocket 연결
        let (ws_stream, _) = connect_async(url)
            .await
            .map_err(|e| DomesticRealtimeError::ConnectionError(e.to_string()))?;

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
            .map_err(|e| DomesticRealtimeError::MessageError(e.to_string()))?;

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
    pub async fn stop(&self) -> Result<(), DomesticRealtimeError> {
        self.tx
            .send(ControlMessage::Stop)
            .await
            .map_err(|e| DomesticRealtimeError::MessageError(e.to_string()))
    }
}
/// 해외 실시간 데이터 관련 오류
#[derive(Debug)]
pub enum DomesticRealtimeError {
    ConnectionError(String),
    AuthError(String),
    MessageError(String),
    EnvError(String),
}
impl From<reqwest::Error> for DomesticRealtimeError {
    fn from(error: reqwest::Error) -> Self {
        DomesticRealtimeError::ConnectionError(error.to_string())
    }
}
