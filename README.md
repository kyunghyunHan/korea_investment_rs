# Korea Investment API Client for Rust

`korea_investment_rs`는 한국투자증권의 Open Trading API를 쉽게 사용할 수 있게 해주는 Rust 라이브러리입니다. REST API와 WebSocket을 통해 국내/해외 주식 시세 조회 및 실시간 데이터 수신, 주문 등 다양한 기능을 제공합니다.

## 주요 기능

- ✅ **국내주식** 시세 조회 및 주문
- ✅ **해외주식** 시세 조회 및 실시간 데이터
- ✅ WebSocket을 통한 실시간 시세 구독
- ✅ OAuth 인증을 통한 간편한 API 접근

## 빠른 시작

### 설치

`Cargo.toml`에 다음 의존성을 추가하세요:

```toml
[dependencies]
korea_investment_rs = "0.1.0"
tokio = { version = "1", features = ["full"] }
```

### 인증 설정

API 사용을 위해 발급받은 앱 키와 시크릿 키를 설정하세요:

```rust
use korea_investment_rs::auth::Auth;

async fn main() {
    let auth = Auth::new("YOUR_APP_KEY", "YOUR_APP_SECRET").await.unwrap();
    println!("발급된 토큰: {}", auth.access_token);
}
```

### 예제: 일별 시세 조회

```rust
use korea_investment_rs::domestic::quotations::InquireDailyItemChartPrice;

#[tokio::main]
async fn main() {
    // 환경 변수에서 API 키 로드
    dotenv::dotenv().ok();
    let app_key = std::env::var("APP_KEY").expect("APP_KEY not set");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET not set");
    
    // 인증 및 API 객체 생성
    let auth = Auth::new(app_key, app_secret).await.unwrap();
    
    // 삼성전자(005930) 일별 시세 조회
    let params = InquireDailyItemChartPrice::new("J", "1", "005930");
    let response = params.send(&auth).await.unwrap();
    
    println!("조회 결과: {:?}", response);
}
```

## 사용 가능한 API 목록

### 국내주식

- `/uapi/domestic-stock/v1/quotations/inquire-price`: 현재가 시세 조회
- `/uapi/domestic-stock/v1/quotations/inquire-daily-itemchartprice`: 일별 시세 조회
- `/uapi/domestic-stock/v1/quotations/inquire-elw-price`: ELW 시세 조회

### 해외주식

- WebSocket을 통한 해외주식 실시간 시세 (`HDFSCNT0`)
- 해외주식 현재가 및 호가 조회 

## 실행 예제

라이브러리에 포함된 예제 코드를 실행하려면:

```bash
# 일별 시세 조회 예제 실행
cargo run --example get_inquire_daily_itemchartprice --features ex

# 실시간 시세 구독 예제 실행
cargo run --example overseas_realtime_websocket --features ex
```

## OAuth 토큰 및 WebSocket 접속키 발급

```rust
use korea_investment_rs::websocket::oauth::ApproveOauth;

async fn get_approval_key() {
    let app_key = std::env::var("APP_KEY").expect("APP_KEY not set");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET not set");
    
    let oauth = ApproveOauth::new(app_key, app_secret).await.unwrap();
    println!("WebSocket 접속키: {}", oauth.approval_key);
}
```

## 주의사항

- API를 사용하기 위해서는 한국투자증권의 Open API 서비스 신청이 필요합니다.
- 실제 환경에서 사용하기 전에 모의투자 환경에서 충분히 테스트하세요.
- 여러 API 호출 시 초당 요청 제한을 고려하세요.

## 라이선스

MIT License

---


## Example
### 1.Oauth
### 2.Domestic(국내시장)
- 
### 3.Overseas(해외시장)
#### 1) 
### 4.Socket
#### 1)해외주식 실시간지연체결가[실시간-007]
- 
#### 2)해외주식 실시간지연호가(아시아)[실시간-008]
#### 3)해외주식 실시간체결통보[실시간-009]
#### 4)해외주식 실시간호가(미국)[실시간-021]
