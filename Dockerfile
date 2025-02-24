FROM rust:1.85-bookworm

WORKDIR /app

RUN apt-get update && \
    apt-get install -y postgresql-client gdb git

ADD Cargo.toml .
ADD Cargo.lock .
ADD main ./main

EXPOSE 3000
