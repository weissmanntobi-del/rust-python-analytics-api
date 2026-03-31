# Python to Rust mental model

## 1. Types are a design tool, not an afterthought

In Python, you can often move quickly by shaping data at runtime.
In Rust, you are rewarded for modeling the boundaries up front.

Examples in this project:

- request DTOs
- database models
- queue payloads
- auth claims
- analytics response shapes

## 2. Error handling is explicit

Python often raises exceptions from deep inside a call chain.
Rust makes errors visible in the function signature.

This project uses:

- `Result<T, AppError>`
- `?` propagation
- explicit mapping to HTTP status codes

## 3. Concurrency is engineered, not implied

In Python, concurrency is often discussed in terms of:
- asyncio tasks
- multiprocessing
- background workers

In Rust, this project demonstrates:
- async request handling with Tokio
- a bounded channel
- a dedicated persistence worker
- clear ownership of data crossing tasks

## 4. JSON boundaries deserve strong structure

Instead of passing around untyped dictionaries everywhere, Rust nudges you to define:
- what is required
- what is optional
- what is allowed to be empty
- what gets serialized back to the caller

## 5. Borrowing becomes easier when data flow is clear

If you keep ownership simple at boundaries, Rust gets much friendlier.
This project mostly clones short-lived request data at task boundaries instead of forcing complicated lifetimes.
That is often the right tradeoff in backend services.
