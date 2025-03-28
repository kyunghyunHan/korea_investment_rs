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
/// 해외 실시간 데이터 클라이언트
pub struct DomesticRealtimeClient {
    app_key: String,
    app_secret: String,
    approval_key: String,
    cust_type: CustType,
}
impl DomesticRealtimeClient {
    // async fn start_stream<T: RealtimeData + Send + 'static>(
    //     &self,
    //     symbol: &str,
    //     r#type: OverseasRealtimeInfoType,
    //     mut callback: impl FnMut(T) + Send + 'static,
    // ) -> Result<StreamController, OverseasRealtimeError> {
    //     let oauth = self;

    //     // WebSocket URL - 타입에 따른 TR 코드 사용
    //     let tr_code = r#type.get_tr_code();
    //     let url = format!("ws://ops.koreainvestment.com:21000/tryitout/{}", tr_code);

    //     // WebSocket 연결
    //     let (ws_stream, _) = connect_async(url)
    //         .await
    //         .map_err(|e| OverseasRealtimeError::ConnectionError(e.to_string()))?;

    //     let (mut write, mut read) = ws_stream.split();

    //     // 접속 키 (API 승인 요청 후 받은 approval_key 사용)
    //     let approval_key = &oauth.approval_key;

    //     // WebSocket 요청 데이터
    //     let request_data = json!({
    //         "header": {
    //             "approval_key": approval_key,
    //             "custtype": self.cust_type,    // P: 개인, B: 법인
    //             "tr_type": "1",     // 1: 등록, 2: 해제
    //             "content-type": "utf-8"
    //         },
    //         "body": {
    //             "input": {
    //                 "tr_id": tr_code,
    //                 "tr_key": symbol
    //             }
    //         }
    //     });
    // }
}
