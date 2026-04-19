# korea_investment_rs

한국투자증권 Open API용 Rust 라이브러리입니다. 현재는 전체 API를 전부 커버하지는 않지만, 공통 transport 레이어와 주요 국내/해외 주식 API를 중심으로 확장 가능한 형태로 정리되어 있습니다.

## 설치

```toml
[dependencies]
korea_investment_rs = "0.2.1"
tokio = { version = "1", features = ["full"] }
```

## 환경 변수

인증:

- `PUB_KEY`
- `SCREST_KEY`

계좌 예제:

- `KIS_CANO`
- `KIS_ACNT_PRDT_CD`

`examples` 실행 시에는 `--features ex` 옵션을 사용하면 `.env`를 읽습니다.

## 현재 구조

- 공통 transport
  - `GET` / `POST`
  - hashkey 생성
  - 실전/모의 `TR_ID` 분기
  - `tr_cont` 헤더 및 연속조회 응답 헤더 수집
- 국내주식
  - 시세 기본 조회
  - 주문/계좌 핵심 API
- 해외주식
  - 시세 기본 조회
  - 주문/계좌 핵심 API
- 국내선물옵션
  - 주문가능 조회 1종
- 장내채권
  - 잔고조회 / 현재가 조회

## 사용 예시

국내 잔고 조회:

```bash
cargo run --example get_balance --features ex
```

국내 호가 조회:

```bash
cargo run --example get_orderbook --features ex
```

해외 잔고 조회:

```bash
cargo run --example get_overseas_balance --features ex
```

해외 호가 조회:

```bash
cargo run --example get_overseas_asking_price --features ex
```

선물옵션 주문가능 조회:

```bash
cargo run --example get_future_possible_order --features ex
```

채권 현재가 조회:

```bash
cargo run --example get_bond_price --features ex
```

## 구현 현황

엑셀 원본 기준 전체 API는 338개입니다.

- REST 278
- WebSocket 60

### 1. 공통 transport

구현됨:

- `GET` 요청 공통 처리
- `POST` 요청 공통 처리
- hashkey 생성
- 실전/모의 도메인 분기
- 실전/모의 `TR_ID` 분기
- `tr_cont` 응답 헤더 수집
- raw 응답 보존 구조 (`RawApiBody`)

미구현:

- 연속조회 자동 반복 수집기
- 시트 기반 코드 생성
- 공통 오류 코드 매핑

### 2. 국내주식 시세

구현됨:

- `get_inquire_price`
- `get_inquire_price2`
- `get_inquire_index_price`
- `get_inquire_period_price`
- `get_recent_ticks`
- `get_today_minutes`
- `get_minutes_by_day`
- `get_orderbook`
- `get_investor_trend`
- `get_member_trend`
- 분석 raw endpoint 묶음
- 순위 raw endpoint 묶음
- 조건검색 목록조회 / 결과조회
- 관심종목 그룹조회 / 그룹별 종목조회
- 재무비율 / 수익성 / 안정성 / 성장성 / 재무제표 계열
- 종목기본정보 / 상품기본정보 / 투자의견 계열

미구현:

- 시간외 현재가/호가 계열 추가 확장
- ETF/ETN/NAV 계열
- 예탁원정보 일정 계열

### 3. 국내주식 주문/계좌

구현됨:

- 현금 매수 주문
- 현금 매도 주문
- 정정/취소 주문
- 예약주문
- 예약주문 조회
- 예약주문 정정/취소
- 잔고 조회
- 실현손익 잔고 조회
- 매수가능 조회
- 일별 주문체결 조회
- 정정취소 가능주문 조회
- 기간별 매매손익 현황 조회
- 기간별 손익 일별합산 조회
- 신용매수가능조회
- 주식통합증거금 현황
- 기간별계좌권리현황조회
- 퇴직연금 잔고조회
- 퇴직연금 예수금조회
- 퇴직연금 매수가능조회
- 퇴직연금 미체결내역
- 퇴직연금 체결기준잔고

미구현:

- 신용주문
- 손익/권리/통합증거금 계열

### 4. 해외주식 시세

구현됨:

- `get_overseas_price`
- `get_overseas_product_info`
- `get_overseas_daily_chartprice`
- `get_overseas_daily_price`
- `get_overseas_period_price`
- `get_overseas_today_minutes`
- `get_overseas_minutes_by_day`
- `get_overseas_index_minutes`
- `get_overseas_asking_price`
- `get_overseas_multi_price`
- 조건검색 / 랭킹 / 뉴스 / 권리 raw endpoint 묶음

미구현:

- 업종별 분석 계열 일부

### 5. 해외주식 주문/계좌

구현됨:

- 해외주식 매수 주문
- 해외주식 매도 주문
- 해외주식 정정/취소 주문
- 해외주식 미국주간 매수/매도 주문
- 해외주식 미국주간 정정/취소
- 해외주식 예약주문 접수
- 해외주식 예약주문 조회
- 해외주식 예약주문 접수취소
- 해외주식 잔고 조회
- 해외주식 체결기준현재잔고 조회
- 해외주식 결제기준잔고 조회
- 해외주식 주문체결 내역 조회
- 해외주식 매수가능금액 조회
- 해외주식 기간손익 조회
- 해외주식 미체결내역 조회
- 해외주식 일별거래내역 조회

미구현:

- 체결기준현재잔고 / 결제기준잔고 / 기간손익 등 일부

### 6. 국내선물옵션

구현됨:

- 선물옵션 주문가능 조회

미구현:

- 주문/정정취소
- 잔고/손익
- 시세/분봉
- 실시간 피드

### 7. 장내채권

구현됨:

- 장내채권 잔고조회
- 장내채권 현재가(시세)

미구현:

- 매수/매도/정정취소 주문
- 체결/일별/기간별 시세
- 평균단가/발행정보/기본조회 추가 타입화

### 8. WebSocket

구현됨:

- 해외 실시간지연체결가
- 해외 실시간지연호가(아시아)
- 해외 실시간체결통보
- 해외 실시간호가(미국)
- 기존 국내/해외 실시간 클라이언트 구조
- 국내 실시간 raw 구독 클라이언트 정리
- 국내 주요 TR 코드 enum 정리

미구현 또는 정리 필요:

- 국내 실시간 typed 모델
- 선물옵션/채권 실시간 계열
- 누락된 feed별 typed wrapper 보강

## 예제 목록

기존 예제:

- `get_inquire_period_price`
- `get_inquire_index_price`
- `get_inquire_daily_itemchartprice`
- `get_overseas_price`
- `get_overseas_daily_chartprice`
- `get_overseas_index_minutes`

이번에 추가된 예제:

- `get_balance`
- `get_balance_realized_pl`
- `get_orderbook`
- `get_domestic_volume_rank`
- `get_condition_search_titles`
- `get_credit_possible_order`
- `get_integrated_margin`
- `get_pension_balance`
- `get_period_trade_profit`
- `get_period_rights`
- `get_reserve_orders`
- `get_overseas_balance`
- `get_overseas_asking_price`
- `get_overseas_news_title`
- `get_overseas_present_balance`
- `get_overseas_period_profit`
- `get_overseas_trade_volume_rank`
- `get_overseas_reserve_orders`
- `place_overseas_daytime_order`
- `get_future_possible_order`
- `get_bond_price`
- `domestic_raw_realtime`

## 인벤토리 추출

엑셀 `API 목록` 시트를 그대로 읽어 요약하려면:

```bash
python3 scripts/extract_kis_api_inventory.py \
  /Users/hangyeonghyeon/Downloads/한국투자증권_오픈API_전체문서_20260419_030008.xlsx \
  --format markdown
```

세부 로드맵은 [docs/api-roadmap.md](docs/api-roadmap.md)에 정리되어 있습니다.
예제 실행법은 [docs/examples.md](docs/examples.md)에 정리되어 있습니다.
