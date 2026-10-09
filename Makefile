.PHONY: fmt lint test check cobertura audit deny hooks dev run run-log

# Formatear el workspace
fmt:
	cargo fmt --all

# Clippy sobre todo el código, incluidos los tests, sin avisos
lint:
	cargo clippy --workspace --all-targets -- -D warnings

# Tests (los e2e necesitan Docker)
test:
	cargo test --workspace

# Lo mismo que exige CI antes de dar un cambio por terminado
check:
	cargo fmt --all -- --check
	$(MAKE) lint
	$(MAKE) test

# Cobertura con las líneas no cubiertas
cobertura:
	cargo llvm-cov --workspace --show-missing-lines

audit:
	cargo audit

deny:
	cargo deny check

# Activa el hook de pre-commit del repositorio (formato)
hooks:
	git config core.hooksPath hooks

# MongoDB y Redis de desarrollo
dev:
	docker compose -f docker-compose.dev.yml up -d --build

run:
	RUST_LOG=info cargo run -p quizz-api --bin quizz

run-log:
	RUST_LOG=quizz_api=info cargo run -p quizz-api --bin quizz
