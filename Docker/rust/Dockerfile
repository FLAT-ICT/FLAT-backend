# 参考にした記事
# [RustのDocker Build時間短縮したい](https://zenn.dev/ucwork/articles/acec204571362b)

FROM rust:1.67 as builder
# ★1.a
RUN USER=root cargo new --bin app
WORKDIR /app

# Install dependencies first for cache ★1.b
# Cargo.lock ないって怒られることがある
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
# RUN rustup component add rustfmt
RUN cargo build --release
RUN rm src/*.rs
# 消さないと、Hello, world! が実行された
RUN rm -rf target/*

# Build my app ★1.c
RUN cargo install diesel_cli --no-default-features --features postgres
COPY ./scripts ./scripts
COPY ./migrations ./migrations
COPY ./src ./src
# ★2.b
# build を test では使用していないので冗長
RUN cargo install --locked --path .
# RUN cargo build --release
# cargo install command をキャッシュする方法がわからん。こいつに時間食われてる

# ★2.a
# FROM debian:buster-slim as final
# あとでcargoを使いたいので、rust:busterを使う
FROM rust:1.67-buster as test
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel
COPY --from=builder /app/target/release/flat_backend /usr/local/bin/flat_backend
RUN apt-get update && apt-get -y install git libpq-dev && rm -rf /var/lib/apt/lists/*

FROM debian:buster-slim as prod
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel
COPY --from=builder /app/target/release/flat_backend /usr/local/bin/flat_backend
RUN apt-get update && apt-get -y install git libpq-dev && rm -rf /var/lib/apt/lists/*