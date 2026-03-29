fmt:
	cargo fmt

test:
	cargo llvm-cov --show-missing-lines

dev:
	docker compose -f docker-compose.dev.yml up -d --build

exec:
	docker compose -f docker-compose.dev.yml exec nodejs-api bash

prod:
	docker compose -f docker-compose.prod.yml up -d --build

run:
	RUST_LOG=info cargo run -p quizz-api --bin quizz

run-log:
	RUST_LOG=quizz_api=info cargo run --bin quizz
