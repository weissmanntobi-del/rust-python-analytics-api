run:
	cargo run

check:
	cargo check

fmt:
	cargo fmt

test:
	cargo test

up:
	docker compose up --build

down:
	docker compose down -v
