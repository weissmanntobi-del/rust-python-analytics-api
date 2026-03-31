# Architecture Notes

## Why a queue?

For Python developers coming to Rust, this project is a good example of where Rust shines:

- high request throughput
- explicit backpressure
- strong typing at system boundaries
- low overhead async execution

The ingestion endpoint does **not** write directly to PostgreSQL.  
Instead, it validates the event and tries to place it into a bounded `tokio::sync::mpsc` queue.

That gives you:

- fast request acknowledgment
- isolation between request path and storage latency
- a clear place to reason about queue saturation
- a simple concurrency model before introducing Kafka or NATS

## Why API keys for ingestion?

In many analytics systems, the sender is not a human logging into a UI.  
It may be:

- a Python script
- a batch job
- a backend service
- a lightweight SDK

That makes API keys a natural ingestion mechanism.

## Why JWT for reporting?

Reporting endpoints are usually accessed by signed-in users or admins.  
JWT is useful here because:

- it maps well to authenticated dashboards
- it keeps the API self-contained
- it is easy to explain to backend developers coming from Flask/FastAPI/Django or Spring Boot

## Where Redis helps

The summary endpoint reads aggregate data that is often requested repeatedly over short windows.  
Redis gives you a cheap performance win with a small TTL cache.

## Production extensions

- structured per-tenant partitioning
- batching and retries in the worker
- rate limiting and abuse protection
- request-level tracing IDs
- persistent queue or external broker
- dashboards and anomaly detection
