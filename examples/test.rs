#[cfg(feature = "ex")]
use dotenv::dotenv;
use futures_util::{SinkExt, stream::StreamExt};
use korea_investment_rs::websocket::oauth::ApproveOauth;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

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

impl OverseasRealtimeData {
    // 구분자(^)로 나뉜 문자열에서 구조체 생성
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

#[tokio::main]
async fn main() {
    #[cfg(feature = "ex")]
    dotenv().ok();

    let app_key = env::var("PUB_KEY").expect("APP_KEY not set in .env file");
    let app_secret = env::var("SCREST_KEY").expect("APP_SECRET not set in .env file");
    let oauth = ApproveOauth::new(app_key, app_secret).await.unwrap();

    // WebSocket URL - 실전(실제) 환경 연결
    let url = "ws://ops.koreainvestment.com:21000/tryitout/HDFSCNT0";

    // WebSocket 연결
    let (ws_stream, _) = connect_async(url).await.expect("WebSocket 연결 실패");
    let (mut write, mut read) = ws_stream.split();

    // 접속 키 (API 승인 요청 후 받은 approval_key 사용)
    let approval_key = oauth.approval_key;

    // WebSocket 요청 데이터 - Python 예제 기반 구조로 수정
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
                "tr_key": "DNASAAPL" // 나스닥 애플 종목 (무료시세)
            }
        }
    });

    // JSON 메시지를 WebSocket으로 전송
    write
        .send(Message::Text(request_data.to_string().into()))
        .await
        .unwrap();

    println!("WebSocket 메시지 전송 완료");
    println!("요청 데이터: {}", request_data.to_string());

    // 메시지 수신 루프
    while let Some(message) = read.next().await {
        match message {
            Ok(Message::Text(text)) => {
                // JSON 형식인지 확인
                if text.starts_with('{') {
                    // JSON 응답 출력
                    println!("JSON 응답: {}", text);
                } else {
                    // 구분자(^)로 나뉜 실시간 데이터 파싱
                    match OverseasRealtimeData::from_delimited_string(&text) {
                        Some(data) => {
                            // 구조체 반환
                            println!("실시간 데이터 수신: {:?}", data);
                        },
                        None => {
                            println!("데이터 파싱 실패: {}", text);
                        }
                    }
                }
            },
            Ok(other) => {
                println!("기타 메시지: {:?}", other);
            },
            Err(e) => {
                eprintln!("WebSocket 에러: {}", e);
                break;
            }
        }
    }
}