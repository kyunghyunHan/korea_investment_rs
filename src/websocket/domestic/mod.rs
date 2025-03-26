
use crate::websocket::oauth::ApproveOauth;
#[cfg(feature = "ex")]
use dotenv::dotenv;
use futures_util::{SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use tokio::sync::mpsc;
use std::error::Error;
use std::fmt;

/// 해외 실시간 데이터 구조체
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomesticRealtimeData {
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

impl DomesticRealtimeData {
    /// 구분자(^)로 나뉜 문자열에서 구조체 생성
    pub fn from_delimited_string(text: &str) -> Option<Self> {
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

/// 해외 실시간 데이터 관련 오류
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
            DomesticRealtimeError::ConnectionError(msg) => write!(f, "연결 오류: {}", msg),
            DomesticRealtimeError::AuthError(msg) => write!(f, "인증 오류: {}", msg),
            DomesticRealtimeError::MessageError(msg) => write!(f, "메시지 오류: {}", msg),
            DomesticRealtimeError::EnvError(msg) => write!(f, "환경 변수 오류: {}", msg),
        }
    }
}

/// 해외 실시간 데이터 클라이언트
pub struct DomesticRealtimeClient {
    app_key: String,
    app_secret: String,
}

impl DomesticRealtimeClient {
    /// 새로운 클라이언트 생성
    pub fn new(app_key: String, app_secret: String) -> Self {
        Self {
            app_key,
            app_secret,
        }
    }

    /// 환경 변수에서 클라이언트 생성
    pub fn from_env() -> Result<Self, DomesticRealtimeError> {
        #[cfg(feature = "ex")]
        dotenv().ok();

        let app_key = env::var("PUB_KEY")
            .map_err(|_| DomesticRealtimeError::EnvError("APP_KEY not set in .env file".to_string()))?;
        let app_secret = env::var("SCREST_KEY")
            .map_err(|_| DomesticRealtimeError::EnvError("APP_SECRET not set in .env file".to_string()))?;

        Ok(Self {
            app_key,
            app_secret,
        })
    }

    /// 해외 실시간 데이터 스트림 시작
    pub async fn start_stream(
        &self,
        symbol: &str,
        mut callback: impl FnMut(DomesticRealtimeData) + Send + 'static,
    ) -> Result<StreamController, DomesticRealtimeError> {
        let oauth = ApproveOauth::new(self.app_key.clone(), self.app_secret.clone())
            .await
            .map_err(|e| DomesticRealtimeError::AuthError(e.to_string()))?;

        // WebSocket URL
        let url = "ws://ops.koreainvestment.com:21000/tryitout/H0STCNT0";

        // WebSocket 연결
        let (ws_stream, _) = connect_async(url)
            .await
            .map_err(|e| DomesticRealtimeError::ConnectionError(e.to_string()))?;
        
        let (mut write, mut read) = ws_stream.split();

        // 접속 키 (API 승인 요청 후 받은 approval_key 사용)
        let approval_key = oauth.approval_key;

        // WebSocket 요청 데이터
        let request_data = json!({
            "header": {
                "approval_key": approval_key,
                "custtype": "P",    // P: 개인, B: 법인
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
                        if !text.starts_with('{') {
                            // 구분자(^)로 나뉜 실시간 데이터 파싱
                            if let Some(data) = DomesticRealtimeData::from_delimited_string(&text) {
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
    ) -> Result<(mpsc::Receiver<DomesticRealtimeData>, StreamController), DomesticRealtimeError> {
        let (data_tx, data_rx) = mpsc::channel::<DomesticRealtimeData>(100);
        
        let controller = self.start_stream(symbol, move |data| {
            let _ = data_tx.try_send(data);
        }).await?;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_from_delimited_string() {
        let text = "DNASAAPL^AAPL^2^20240115^240115^163000^240116^013000^190.62^191.37^189.95^190.95^2^0.72^0.38^190.95^190.97^241928^33337^1800^6523064^1244686432.52^3235000^3288064^1.02^1";
        let data = DomesticRealtimeData::from_delimited_string(text).unwrap();
        assert_eq!(data.rsym, "DNASAAPL");
        assert_eq!(data.symb, "AAPL");
        assert_eq!(data.last, "190.95");
    }
}

// 예제 사용 방법
pub async fn example_usage() -> Result<(), Box<dyn Error>> {
    // 클라이언트 생성
    let client = DomesticRealtimeClient::from_env()?;
    
    // 콜백 함수로 데이터 처리
    let controller = client.start_stream("DNASAAPL", |data| {
        println!("실시간 데이터: {:?}", data);
    }).await?;
    
    // 또는 채널로 데이터 처리
    let (mut data_rx, controller2) = client.start_stream_channel("DNASNASD").await?;
    
    tokio::spawn(async move {
        while let Some(data) = data_rx.recv().await {
            println!("채널 데이터: {:?}", data);
        }
    });
    
    // 10초 후 스트림 중지
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    controller.stop().await?;
    controller2.stop().await?;
    
    Ok(())
}