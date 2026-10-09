# syntax=docker/dockerfile:1
# Builder (Debian 13 "trixie") y runtime (distroless debian13) comparten la misma glibc.
FROM lukemathwalker/cargo-chef:0.1.78-rust-1.99-slim-trixie AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Solo dependencias: esta capa queda en caché mientras Cargo.toml/Cargo.lock no cambien.
RUN cargo chef cook --release --locked -p quizz-api --bin quizz --recipe-path recipe.json
COPY . .
RUN cargo build --release --locked -p quizz-api --bin quizz

FROM gcr.io/distroless/cc-debian13:nonroot AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/quizz /app/quizz
# La política RBAC va embebida en el binario. configuration.yaml NO se copia: se monta en
# /app/configuration.yaml (solo lectura) o se pasa por variables QUIZZ_*.
USER nonroot:nonroot
EXPOSE 8008
ENTRYPOINT ["/app/quizz"]
