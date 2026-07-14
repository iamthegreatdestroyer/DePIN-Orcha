# DePIN-Orcha

Layer 6 (Operational Intelligence) orchestrator for Decentralized Physical
Infrastructure Networks. It aggregates earnings and health across multiple DePIN
protocols, surfaces optimization opportunities, and can reallocate resources
between protocols to improve returns.

## What it does

- **Protocol adapters** — Streamr, Storj, Golem, and Grass adapters implementing a
  common `ProtocolAdapter` trait (connect, earnings, health check, apply
  allocation). Earnings are currently simulated per adapter, not pulled from live
  protocol APIs.
- **Orchestration** — a coordinator polls and aggregates per-protocol metrics; an
  earnings optimizer performs pairwise opportunity analysis and greedy allocation;
  a reallocation engine executes/rolls back allocation changes with rate limiting.
- **Node registry + task router** — register/deregister/discover nodes by hardware
  requirements, with CapacityFirst / RoundRobin / LowestLatency routing strategies.
- **Realtime monitor** — dashboard snapshots and alert generation.
- **HTTP + WebSocket API** — actix-web `/api/v1` route tree with API-key auth,
  rate limiting, and a 5s metrics push over WebSocket.
- **Persistence** — SQLite (default `depin_orcha.db`) with schema migrations.

## Run

```bash
cargo run --bin depin-orcha
```

The API listens on `:8080` by default (override with `API_PORT`; host via
`API_HOST`). SQLite path defaults to `depin_orcha.db` (override with
`DATABASE_URL`).

Key endpoints (all under `/api/v1`):

- `GET /health` — health check (no auth)
- `GET /status` — system status (no auth)
- `GET /metrics`, `GET /metrics/history` — current + historical metrics (auth)
- `GET /opportunities` — optimization opportunities (auth)
- `GET /allocation` — optimal allocation plan (auth)
- `POST /reallocate` — execute a reallocation (auth)
- `GET /dashboard` — dashboard snapshot (auth)
- `ws://<host>:8080/ws` — WebSocket metrics stream

## Status

Work in progress (~55% per the completion brief). See `BUILD_STATUS.md` for the
per-module implemented-vs-stubbed breakdown and `CLAUDE.md` for the sprint plan.
