# KIS API Roadmap

This repository does not yet cover the full Korea Investment Open API surface.
The April 19, 2026 workbook contains 338 APIs:

- 278 REST APIs
- 60 WebSocket APIs
- 256 `GET` endpoints
- 82 `POST` endpoints

Current code coverage is still narrow:

- Domestic quotations: spot price, price2, index price, period price, ticks, minute candles
- Overseas quotations: current price, product info, daily and minute chart helpers
- OAuth: token issuance with local cache
- WebSocket: a subset of domestic and overseas realtime feeds

The main gap is not just missing endpoint wrappers. The library also lacks common infrastructure needed for broad API coverage:

- Generic `POST` request handling
- Hashkey generation for order endpoints
- Continuous pagination / `tr_cont`
- Practice vs production `TR_ID` switching beyond a few endpoints
- Order/account request and response modeling
- A consistent provider-level public API across domains

Recommended implementation order:

1. Common transport layer
2. OAuth adjunct APIs
3. Domestic trading/account APIs
4. Domestic quotation expansion
5. Overseas trading/account APIs
6. Overseas quotation/ranking expansion
7. Futures/options and bond domains
8. WebSocket coverage completion

Phase details:

1. Common transport layer
- Add generic `GET` and `POST` helpers.
- Add hashkey support.
- Add support for continuation headers and typed pagination helpers.
- Normalize practice/production domain and `TR_ID` selection.

2. OAuth adjunct APIs
- `Hashkey`
- `접근토큰폐기(P)`
- `실시간 (웹소켓) 접속키 발급` cleanup under a unified auth module

3. Domestic trading/account APIs
- Cash order
- Revise/cancel order
- Balance
- Possible order amount
- Daily order/fill history
- Unfilled orders

4. Domestic quotation expansion
- Orderbook / expected fill
- Time-extended session data
- ETF/ETN and sector/index coverage
- Ranking and condition search APIs

5. Overseas trading/account APIs
- Order / revise / cancel
- Balance / present balance / settlement balance
- Order and fill history
- Possible buy amount

6. Overseas quotation/ranking expansion
- Multi-symbol quotes
- Detailed quotes and orderbook
- Ranking APIs
- Rights and news APIs

7. Derivatives and bond domains
- Domestic futures/options
- Overseas futures/options
- Exchange bonds

8. WebSocket coverage completion
- Missing domestic feed variants
- Missing overseas quote / delayed / notification variants
- Domestic and overseas derivatives realtime feeds

Initial prioritization for this repository:

- Highest value: domestic trading + overseas trading
- Next: common transport refactor
- Then: quotation breadth and websocket completion

Useful extraction command:

```bash
python3 scripts/extract_kis_api_inventory.py \
  /Users/hangyeonghyeon/Downloads/한국투자증권_오픈API_전체문서_20260419_030008.xlsx \
  --format markdown
```
