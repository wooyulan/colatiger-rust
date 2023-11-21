FROM rust:1.73.0 as builder
RUN USER=root cargo new --bin colatiger

WORKDIR /app

COPY Cargo.toml Cargo.lock  ./

COPY src ./src
COPY config ./config

RUN cargo build --release

FROM alpine

WORKDIR /

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=3000


CMD ["/colatiger"]
