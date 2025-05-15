fmt:
	cargo fmt

test:
	cargo llvm-cov --show-missing-lines

dev:
	docker compose -f docker-compose.dev.yml up -d --build

exec:
	docker compose -f docker-compose.dev.yml exec nodejs-api bash
