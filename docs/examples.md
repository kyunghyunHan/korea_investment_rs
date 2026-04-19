# Example Guide

이 문서는 저장소에 들어있는 예제 실행 방법을 한 곳에 모아둔 문서입니다.

## 준비

`.env` 또는 환경 변수에 아래 값을 준비합니다.

```env
PUB_KEY=...
SCREST_KEY=...
KIS_CANO=12345678
KIS_ACNT_PRDT_CD=01
```

대부분의 예제는 `--features ex` 를 사용하면 `.env`를 읽습니다.

기본 실행 형식:

```bash
cargo run --example <example_name> --features ex
```

실전 전용 API 예제는 모의투자에서 실패할 수 있습니다.

## 국내 시세

```bash
cargo run --example get_inquire_period_price --features ex
cargo run --example get_inquire_index_price --features ex
cargo run --example get_inquire_daily_itemchartprice --features ex
cargo run --example get_orderbook --features ex
```

## 국내 랭킹/분석/조건검색

```bash
cargo run --example get_domestic_volume_rank --features ex
cargo run --example get_condition_search_titles --features ex
```

## 국내 주문/계좌

```bash
cargo run --example get_balance --features ex
cargo run --example get_balance_realized_pl --features ex
cargo run --example get_period_trade_profit --features ex
cargo run --example get_credit_possible_order --features ex
cargo run --example get_integrated_margin --features ex
cargo run --example get_period_rights --features ex
cargo run --example get_pension_balance --features ex
cargo run --example get_reserve_orders --features ex
```

## 해외 시세

```bash
cargo run --example get_overseas_price --features ex
cargo run --example get_overseas_daily_chartprice --features ex
cargo run --example get_overseas_index_minutes --features ex
cargo run --example get_overseas_asking_price --features ex
```

## 해외 랭킹/뉴스/조건검색

```bash
cargo run --example get_overseas_news_title --features ex
cargo run --example get_overseas_trade_volume_rank --features ex
```

## 해외 주문/계좌

```bash
cargo run --example get_overseas_balance --features ex
cargo run --example get_overseas_present_balance --features ex
cargo run --example get_overseas_period_profit --features ex
cargo run --example get_overseas_reserve_orders --features ex
cargo run --example place_overseas_daytime_order --features ex
```

## 선물옵션 / 채권

```bash
cargo run --example get_future_possible_order --features ex
cargo run --example get_bond_price --features ex
```

## WebSocket

승인키 발급 예제:

```bash
cargo run --example creat_approval_key --features ex
```

국내 raw 실시간 구독:

```bash
cargo run --example domestic_raw_realtime --features ex
```

해외 실시간 예제:

```bash
cargo run --example real_time_delayed_execution_price --features ex
```

## 참고

- 예제 이름은 `Cargo.toml`의 `[[example]]` 선언이 없어도 `examples/` 아래 파일명 기준으로 실행할 수 있습니다.
- 일부 예제는 계좌 상태나 장 운영 시간에 따라 응답이 비어 있을 수 있습니다.
- 예약주문, 퇴직연금, 미국주간주문, 채권 일부는 실전 계좌 전용입니다.
