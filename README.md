# Book Link : https://tobiweissmann.gumroad.com/l/wcylm
# Rust for Python Developers: Production-Grade Analytics API

A practical Rust project that teaches Python developers how to build a real backend service with:

- **Axum** for the HTTP API
- **PostgreSQL** for persistent analytics storage
- **Redis** for short-lived cache entries
- **Tokio mpsc** for queue-based background processing
- **API key ingestion** for event tracking from scripts or apps
- **JWT authentication** for admin and reporting endpoints
- **Structured logging + tracing**
- **Docker + docker-compose**
- **SQL migrations**
- **Python example client** for sending events

## What this project demonstrates

Python developers often understand HTTP APIs quickly, but Rust feels different in a few specific areas:

- ownership and borrowing
- explicit error handling
- concurrency with channels
- static typing around JSON and database boundaries
- predictable backend performance

This project bridges that gap with a realistic architecture.

## Architecture

```text
Python client / frontend / SDK
            |
            v
  POST /api/v1/events  -- X-API-Key -->
            |
            v
     Axum handler validates payload
            |
            v
  Tokio mpsc bounded queue (backpressure)
            |
            v
    Background worker persists events
            |
            +--> PostgreSQL
            |
            +--> admin reporting endpoints
                     |
                     +--> Redis cache
```

## Endpoints

### Public / ingestion

- `GET /api/v1/health/live`
- `GET /api/v1/health/ready`
- `POST /api/v1/auth/register`
- `POST /api/v1/auth/login`
- `POST /api/v1/events` (API key required)

### Protected reporting endpoints

- `GET /api/v1/auth/me` (Bearer token)
- `GET /api/v1/events/recent?limit=20` (Bearer token)
- `GET /api/v1/analytics/summary?from=...&to=...` (Bearer token)
- `GET /api/v1/analytics/timeseries?from=...&to=...&bucket=day` (Bearer token)

## Quick start

### 1. Copy environment variables

```bash
cp .env.example .env
```

### 2. Start infrastructure

```bash
docker compose up postgres redis -d
```

### 3. Run the API

```bash
cargo run
```

### 4. Register a user

```bash
curl -X POST http://localhost:8080/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "python@example.com",
    "full_name": "Python Engineer",
    "password": "VeryStrongPassword123"
  }'
```

You will receive:

- a JWT token for admin/reporting requests
- an API key for event ingestion

### 5. Send an event

```bash
curl -X POST http://localhost:8080/api/v1/events \
  -H "Content-Type: application/json" \
  -H "X-API-Key: YOUR_API_KEY" \
  -d '{
    "event_name": "page_view",
    "page_url": "/pricing",
    "session_id": "sess-123",
    "properties": {
      "source": "python-script",
      "campaign": "medium"
    }
  }'
```

### 6. Fetch analytics summary

```bash
curl "http://localhost:8080/api/v1/analytics/summary" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

## Python example client

See `examples/python/send_events.py`.

Install requests if needed:

```bash
pip install requests
```

Then run:

```bash
export API_BASE=http://localhost:8080
export API_KEY=YOUR_API_KEY
python examples/python/send_events.py
```

## Project layout

```text
src/
  app.rs
  config.rs
  dto/
  error.rs
  models/
  queue.rs
  repository/
  routes/
  services/
  state.rs
```

## Suggested learning path

1. Start with the ingestion endpoint.
2. Follow how the event enters the queue.
3. Inspect the worker that writes to Postgres.
4. Compare API-key auth vs JWT auth.
5. Inspect Redis caching on analytics summary.
6. Extend with batching, metrics, or tenant isolation.

## Extension ideas

- batch inserts in the worker
- dead-letter queue table for failed events
- Prometheus metrics
- per-project API keys
- idempotency keys
- Kafka or NATS instead of in-process queue
- clickhouse sink for high-volume analytics
- OpenAPI generation
- rate limiting per API key

## Notes

This project is intentionally practical instead of overly abstract.  
It is a strong portfolio project for backend, platform, and infra-minded Rust learners.

Because this environment does not include a Rust toolchain, I could not run `cargo check` here. The structure and code are designed to be realistic and close to runnable, but you should validate locally before publishing or selling.
