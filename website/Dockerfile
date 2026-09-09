# syntax=docker/dockerfile:1

# 构建阶段
FROM rust:1.98-alpine AS builder

RUN apk add --no-cache \
    build-base \
    perl \
    pkgconfig \
    openssl-dev \
    ca-certificates \
    clang \
    lld

RUN rustup target add wasm32-unknown-unknown \
    && rustup component add rust-src

ARG CARGO_LEPTOS_VERSION=0.3.7
RUN cargo install cargo-leptos --version ${CARGO_LEPTOS_VERSION} --locked

WORKDIR /build

# 先复制清单与所有 crate 的 Cargo.toml
COPY Cargo.toml Cargo.lock ./
COPY .cargo/config.toml ./.cargo/config.toml

COPY app/Cargo.toml ./app/
COPY frontend/Cargo.toml ./frontend/
COPY protocol/Cargo.toml ./protocol/
COPY renderer/Cargo.toml ./renderer/
COPY server/Cargo.toml ./server/
COPY worker/Cargo.toml ./worker/

# 为所有 crate 生成最小占位源码
RUN mkdir -p \
        app/src \
        frontend/src \
        protocol/src \
        renderer/src \
        server/src \
        worker/src \
    && touch \
        app/src/lib.rs \
        frontend/src/lib.rs \
        protocol/src/lib.rs \
        renderer/src/lib.rs \
        server/src/lib.rs \
        server/src/main.rs \
        worker/src/lib.rs

# 下载依赖
RUN cargo fetch

# 复制源码
COPY ./ ./

# 构建
RUN cargo leptos build --release

# 运行阶段
FROM alpine:3.21

RUN apk add --no-cache ca-certificates tzdata \
    && addgroup -S website \
    && adduser -S website -G website

WORKDIR /app

COPY --from=builder /build/target/release/server /app/server
COPY --from=builder /build/target/site /app/site

ENV LEPTOS_SITE_ROOT=/app/site \
    LEPTOS_SITE_ADDR=0.0.0.0:3000 \
    LEPTOS_RELOAD_PORT=3001

EXPOSE 3000

USER website

CMD [ "/app/server" ]