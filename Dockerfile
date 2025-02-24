FROM rust:1.84-slim-bullseye

WORKDIR /app

RUN apt-get update && \
    apt-get install -y postgresql-client gdb git

ADD Cargo.toml .
ADD Cargo.lock .
ADD main .

EXPOSE 3000
